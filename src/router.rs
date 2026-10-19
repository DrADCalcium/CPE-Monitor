use crate::model::{Config, Snapshot, origin, protect};
use anyhow::{Context, Result, bail};
use hmac::{Hmac, KeyInit, Mac};
use sha2::{Digest, Sha256};
use std::io::Read;
use std::{
    collections::{HashMap, VecDeque},
    time::Duration,
};
use zeroize::{Zeroize, Zeroizing};
type Fields = HashMap<String, String>;

#[derive(Debug)]
pub struct ApiError(pub String);
impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CPE 接口错误 {}", self.0)
    }
}
impl std::error::Error for ApiError {}
fn fields(raw: &str) -> Result<Fields> {
    let doc = roxmltree::Document::parse(raw).context("CPE 返回了非 XML 数据")?;
    let root = doc.root_element();
    if root.tag_name().name() == "error" {
        return Err(ApiError(
            root.children()
                .find(|c| c.has_tag_name("code"))
                .and_then(|c| c.text())
                .filter(|s| s.len() <= 9 && s.bytes().all(|c| c.is_ascii_digit()))
                .unwrap_or("unknown")
                .to_owned(),
        )
        .into());
    }
    Ok(root
        .children()
        .filter(|c| c.is_element())
        .map(|c| {
            (
                c.tag_name().name().to_owned(),
                c.text().unwrap_or("").to_owned(),
            )
        })
        .collect())
}
fn hmac(key: &[u8], message: &[u8]) -> Vec<u8> {
    let mut m = Hmac::<Sha256>::new_from_slice(key).unwrap();
    m.update(message);
    m.finalize().into_bytes().to_vec()
}
fn required<'a>(f: &'a Fields, k: &str) -> Result<&'a str> {
    f.get(k)
        .map(String::as_str)
        .with_context(|| format!("接口缺少字段 {k}"))
}
fn number(f: &Fields, k: &str) -> Result<f64> {
    let v: f64 = required(f, k)?.parse()?;
    if !v.is_finite() || v < 0. {
        bail!("接口数值异常：{k}");
    }
    Ok(v)
}
fn rounds(f: &Fields, k: &str) -> Result<u32> {
    let n = required(f, k)?.parse()?;
    if !(1..=2_000_000).contains(&n) {
        bail!("登录迭代参数异常");
    }
    Ok(n)
}
fn salted(password: &str, salt: &str, n: u32) -> Result<[u8; 32]> {
    Ok(pbkdf2::pbkdf2_hmac_array::<Sha256, 32>(
        password.as_bytes(),
        &hex::decode(salt)?,
        n,
    ))
}
fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

pub struct Router {
    client: reqwest::blocking::Client,
    base: String,
    tokens: VecDeque<String>,
}
impl Router {
    pub fn new(c: &Config) -> Result<Self> {
        Ok(Self {
            base: origin(&c.router)?,
            tokens: VecDeque::new(),
            client: reqwest::blocking::Client::builder()
                .no_proxy()
                .cookie_store(true)
                .redirect(reqwest::redirect::Policy::none())
                .connect_timeout(Duration::from_secs(4))
                .timeout(Duration::from_secs(8))
                .build()?,
        })
    }
    fn initialize(&mut self) -> Result<()> {
        let r = self
            .client
            .get(format!("{}/", self.base))
            .send()?
            .error_for_status()?;
        let html = bounded_text(r, 1024 * 1024)?;
        self.tokens.clear();
        for p in html.split("name=\"csrf_token\"").skip(1) {
            if let Some(v) = p
                .split("content=\"")
                .nth(1)
                .and_then(|s| s.split('"').next())
            {
                self.tokens.push_back(v.to_owned());
            }
        }
        if self.tokens.is_empty() {
            let f = self.call("webserver/SesTokInfo", None)?;
            self.tokens.push_back(required(&f, "TokInfo")?.to_owned());
        }
        Ok(())
    }
    fn call(&mut self, path: &str, body: Option<&[(&str, String)]>) -> Result<Fields> {
        let url = format!("{}/api/{path}", self.base);
        let mut r = if let Some(f) = body {
            let xml = format!(
                "<request>{}</request>",
                f.iter()
                    .map(|(k, v)| format!("<{k}>{}</{k}>", escape(v)))
                    .collect::<String>()
            );
            self.client
                .post(url)
                .header("Content-Type", "application/xml")
                .body(xml)
        } else {
            self.client.get(url)
        };
        r = r.header("Referer", format!("{}/", self.base));
        if let Some(t) = self.tokens.front() {
            r = r.header("__RequestVerificationToken", t);
        }
        if body.is_some() && self.tokens.len() > 1 {
            self.tokens.pop_front();
        }
        let response = r.send()?.error_for_status()?;
        let headers = response.headers();
        if let Some(t) = headers
            .get("__RequestVerificationTokenone")
            .or_else(|| headers.get("__RequestVerificationToken"))
        {
            self.tokens.clear();
            self.tokens.push_back(t.to_str()?.to_owned());
            if let Some(t) = headers.get("__RequestVerificationTokentwo") {
                self.tokens.push_back(t.to_str()?.to_owned());
            }
        }
        let raw = bounded_text(response, 64 * 1024)?;
        fields(&raw)
    }
    pub fn login(&mut self, c: &Config) -> Result<()> {
        self.initialize()?;
        let password = Zeroizing::new(
            protect(&c.password, true).context("无法读取本机密码，请到设置重新输入")?,
        );
        let mut random = [0u8; 32];
        unsafe {
            use windows_sys::Win32::Security::Cryptography::*;
            if BCryptGenRandom(
                std::ptr::null_mut(),
                random.as_mut_ptr(),
                32,
                BCRYPT_USE_SYSTEM_PREFERRED_RNG,
            ) != 0
            {
                bail!("随机数生成失败");
            }
        }
        let nonce = hex::encode(random);
        let f = self.call(
            "user/challenge_login",
            Some(&[
                ("username", "admin".into()),
                ("firstnonce", nonce.clone()),
                ("mode", "1".into()),
            ]),
        )?;
        let server_nonce = required(&f, "servernonce")?;
        if !server_nonce.starts_with(&nonce) || server_nonce.len() <= nonce.len() {
            bail!("服务器挑战随机数校验失败");
        }
        let salt = Zeroizing::new(salted(
            &password,
            required(&f, "salt")?,
            rounds(&f, "iterations")?,
        )?);
        // Huawei WebUI uses the label/auth-message as HMAC key (unlike standard SCRAM).
        let ck = Zeroizing::new(hmac(b"Client Key", &*salt));
        let stored = Sha256::digest(&*ck);
        let auth = format!("{nonce},{server_nonce},{server_nonce}");
        let sig = hmac(auth.as_bytes(), &stored);
        let proof = hex::encode(ck.iter().zip(sig).map(|(a, b)| a ^ b).collect::<Vec<_>>());
        let mut body = vec![
            ("clientproof", proof.clone()),
            ("finalnonce", server_nonce.to_owned()),
        ];
        if f.get("newType").is_some_and(|s| s == "1") {
            let s = salted(
                &password,
                required(&f, "newSalt")?,
                rounds(&f, "newIterations")?,
            )?;
            let stored = hex::encode(Sha256::digest(hmac(b"Client Key", &s)));
            let server = hex::encode(hmac(b"Server Key", &s));
            body.push((
                "hashOldNewPwd",
                hex::encode(Sha256::digest(format!("{stored}{server}{proof}"))),
            ));
            body.push(("newStoredKey", stored));
            body.push(("newServerKey", server));
        }
        let result = self.call("user/authentication_login", Some(&body));
        for (_, v) in &mut body {
            v.zeroize();
        }
        let result = result?;
        let signature = hex::decode(required(&result, "serversignature")?)?;
        let server_key = Zeroizing::new(hmac(b"Server Key", &*salt));
        let mut check = Hmac::<Sha256>::new_from_slice(auth.as_bytes()).unwrap();
        check.update(&server_key);
        check
            .verify_slice(&signature)
            .map_err(|_| anyhow::anyhow!("服务器登录签名校验失败"))?;
        Ok(())
    }
    pub fn snapshot(&mut self, c: &Config) -> Result<Snapshot> {
        let m = self.call("monitoring/month_statistics", None)?;
        let plan = self.call("monitoring/start_date", None)?;
        let t = self.call("monitoring/traffic-statistics", None)?;
        let total = c
            .quota_gb
            .map(|q| q * 1024_f64.powi(3))
            .or_else(|| {
                plan.get("trafficmaxlimit")
                    .and_then(|n| n.parse::<f64>().ok())
                    .filter(|v| v.is_finite() && *v > 0.)
            })
            .or_else(|| plan.get("DataLimit").and_then(|s| parse_limit(s)));
        let day = c.reset_day.or_else(|| {
            plan.get("StartDay")
                .and_then(|n| n.parse().ok())
                .filter(|d| (1..=31).contains(d))
        });
        Ok(Snapshot {
            used: number(&m, "CurrentMonthDownload")? + number(&m, "CurrentMonthUpload")?,
            total,
            lifetime: number(&t, "TotalDownload")? + number(&t, "TotalUpload")?,
            reset_day: day,
            updated: chrono::Local::now(),
        })
    }
    pub fn refresh(&mut self, c: &Config) -> Result<Snapshot> {
        match self.snapshot(c) {
            Err(e) if expired(&e) => {
                self.login(c)?;
                self.snapshot(c)
            }
            other => other,
        }
    }
}
fn parse_limit(input: &str) -> Option<f64> {
    let s = input.trim().to_uppercase();
    for (unit, mul) in [
        ("TB", 1024_f64.powi(4)),
        ("GB", 1024_f64.powi(3)),
        ("MB", 1024_f64.powi(2)),
        ("KB", 1024.),
        ("B", 1.),
    ] {
        if let Some(v) = s.strip_suffix(unit) {
            return v
                .trim()
                .parse::<f64>()
                .ok()
                .filter(|n| n.is_finite() && *n > 0.)
                .map(|n| n * mul);
        }
    }
    s.parse::<f64>().ok().filter(|n| n.is_finite() && *n > 0.)
}
pub fn auth_failure(e: &anyhow::Error) -> bool {
    e.downcast_ref::<ApiError>().is_some_and(|e| {
        matches!(
            e.0.as_str(),
            "108001" | "108002" | "108006" | "108007" | "108010"
        )
    })
}
pub fn expired(e: &anyhow::Error) -> bool {
    e.downcast_ref::<ApiError>()
        .is_some_and(|e| matches!(e.0.as_str(), "100002" | "125001" | "125002" | "125003"))
}
pub fn describe(e: &anyhow::Error) -> String {
    if let Some(e) = e.downcast_ref::<ApiError>() {
        match e.0.as_str() {
            "108001" | "108002" | "108006" => {
                return "密码错误，已停止自动重试；请在设置中修改密码".into();
            }
            "108007" | "108010" => return "CPE 暂时限制登录，已停止重试；稍后保存设置再试".into(),
            "108003" | "108005" | "108009" => {
                return "CPE 提示已有会话；请退出管理网页后重试".into();
            }
            _ => (),
        }
    }
    if let Some(e) = e.downcast_ref::<reqwest::Error>() {
        return if e.is_timeout() {
            "连接超时"
        } else if e.is_connect() {
            "无法连接 CPE"
        } else {
            "CPE 网络请求失败"
        }
        .into();
    }
    if e.downcast_ref::<ApiError>().is_some() {
        return "CPE 接口返回错误".into();
    }
    "CPE 数据读取或登录校验失败，请检查设置".into()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn limits() {
        assert_eq!(parse_limit("1500GB"), Some(1500. * 1024_f64.powi(3)));
        assert_eq!(parse_limit("0MB"), None);
    }
    #[test]
    fn errors_are_not_zero() {
        assert!(fields("<error><code>100002</code></error>").is_err());
        assert!(number(&fields("<response/>").unwrap(), "CurrentMonthUpload").is_err());
    }
    #[test]
    fn untrusted_error_text_is_not_displayed() {
        let e = fields("<error><code>private-token-example</code></error>").unwrap_err();
        assert!(!describe(&e).contains("private-token-example"));
        let e = reqwest::blocking::Client::new()
            .get("private-host")
            .build()
            .unwrap_err();
        let message = describe(&e.into());
        assert!(!message.contains("private-host"));
    }
}

fn bounded_text(response: reqwest::blocking::Response, limit: u64) -> Result<String> {
    anyhow::ensure!(
        response.content_length().is_none_or(|v| v <= limit),
        "CPE 响应过大"
    );
    let mut bytes = Zeroizing::new(Vec::new());
    response.take(limit + 1).read_to_end(&mut bytes)?;
    anyhow::ensure!(bytes.len() as u64 <= limit, "CPE 响应过大");
    String::from_utf8(bytes.to_vec()).map_err(|_| anyhow::anyhow!("CPE 返回编码异常"))
}
