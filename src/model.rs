// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (C) 2026 CPE Monitor contributors.
use anyhow::{Context, Result, bail};
use base64::{Engine, engine::general_purpose::STANDARD};
use chrono::{Datelike, Local, NaiveDate};
use serde::{Deserialize, Serialize};
use std::{net::Ipv4Addr, path::PathBuf};
use zeroize::{Zeroize, Zeroizing};

pub const AUTOSTART_ARG: &str = "--autostart";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TaskbarSide {
    #[default]
    Right,
    Left,
}

#[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TaskbarConfig {
    pub enabled: bool,
    pub side: TaskbarSide,
    pub offset: i32,
    /// Leave empty to use the main window's selected font.
    pub font: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub router: String,
    pub password: String,
    pub theme: String,
    pub interval: u32,
    pub adapter: String,
    pub ip: String,
    pub font: String,
    pub font_weight: u16,
    pub heading_weight: u16,
    pub ui_scale: f32,
    pub autostart: bool,
    pub autostart_hidden: bool,
    pub topmost: bool,
    pub taskbar: TaskbarConfig,
    pub quota_gb: Option<f64>,
    pub reset_day: Option<u32>,
    pub position: Option<[f32; 2]>,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            router: String::new(),
            password: String::new(),
            theme: "system".into(),
            interval: 10,
            adapter: String::new(),
            ip: String::new(),
            font: "MiSans".into(),
            font_weight: 400,
            heading_weight: 600,
            ui_scale: 1.0,
            autostart: false,
            autostart_hidden: false,
            topmost: false,
            taskbar: TaskbarConfig::default(),
            quota_gb: None,
            reset_day: None,
            position: None,
        }
    }
}
pub fn data_dir() -> PathBuf {
    std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("data")
}
pub fn load_config() -> Config {
    crate::storage::read("config.json")
        .ok()
        .and_then(|b| serde_json::from_slice(&b).ok())
        .unwrap_or_default()
}
pub fn save_config(c: &Config) -> Result<()> {
    crate::storage::write("config.json", &serde_json::to_vec_pretty(c)?)
}
pub fn protect(value: &str, decrypt: bool) -> Result<String> {
    use windows_sys::Win32::{Foundation::LocalFree, Security::Cryptography::*};
    let mut bytes = Zeroizing::new(if decrypt {
        STANDARD.decode(value).context("密码数据损坏")?
    } else {
        value.as_bytes().to_vec()
    });
    let input = CRYPT_INTEGER_BLOB {
        cbData: bytes.len() as u32,
        pbData: bytes.as_mut_ptr(),
    };
    let mut output = CRYPT_INTEGER_BLOB {
        cbData: 0,
        pbData: std::ptr::null_mut(),
    };
    unsafe {
        let ok = if decrypt {
            CryptUnprotectData(
                &input,
                std::ptr::null_mut(),
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
                CRYPTPROTECT_UI_FORBIDDEN,
                &mut output,
            )
        } else {
            CryptProtectData(
                &input,
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
                CRYPTPROTECT_UI_FORBIDDEN,
                &mut output,
            )
        };
        if ok == 0 {
            bail!(
                "Windows 密码加解密失败：{}",
                std::io::Error::last_os_error()
            );
        }
        let slice = std::slice::from_raw_parts_mut(output.pbData, output.cbData as usize);
        let result = if decrypt {
            String::from_utf8(slice.to_vec()).context("密码编码错误")
        } else {
            Ok(STANDARD.encode(&*slice))
        };
        slice.zeroize();
        LocalFree(output.pbData as _);
        result
    }
}
pub fn origin(input: &str) -> Result<String> {
    let url = reqwest::Url::parse(input.trim()).context("地址应包含 http:// 或 https://")?;
    if !matches!(url.scheme(), "http" | "https")
        || !url.username().is_empty()
        || url.password().is_some()
    {
        bail!("请输入内网 HTTP/HTTPS 地址");
    }
    let ip: Ipv4Addr = url
        .host_str()
        .unwrap_or("")
        .parse()
        .context("路由地址请使用内网 IPv4 地址")?;
    if !(ip.is_private() || ip.is_loopback()) {
        bail!("路由地址必须是内网 IPv4 地址");
    }
    Ok(url.origin().ascii_serialization())
}
pub fn validate(c: &Config) -> Result<()> {
    if !(100..=900).contains(&c.font_weight) || !(100..=900).contains(&c.heading_weight) {
        bail!("字重应为 100–900");
    }
    if !c.ui_scale.is_finite() || !(0.75..=1.25).contains(&c.ui_scale) {
        bail!("界面缩放应为 75%–125%");
    }
    origin(&c.router)?;
    if !(1..=1440).contains(&c.interval) {
        bail!("刷新间隔应为 1–1440 分钟");
    }
    if !c.ip.trim().is_empty() {
        c.ip.trim()
            .parse::<Ipv4Addr>()
            .context("刷新条件 IP 格式不正确")?;
    }
    if c.reset_day.is_some_and(|d| !(1..=31).contains(&d)) {
        bail!("清零日应为 1–31");
    }
    if c.quota_gb.is_some_and(|q| !q.is_finite() || q <= 0.) {
        bail!("套餐额度必须大于 0");
    }
    Ok(())
}
pub fn autostart(enabled: bool) -> Result<()> {
    let hkcu = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
    let (key, _) = hkcu.create_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Run")?;
    if enabled {
        key.set_value(
            "CPE-Monitor",
            &format!("\"{}\" {AUTOSTART_ARG}", std::env::current_exe()?.display()),
        )?;
    } else {
        match key.delete_value("CPE-Monitor") {
            Ok(_) => (),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
            Err(e) => return Err(e.into()),
        }
    }
    Ok(())
}
pub fn adapters() -> Vec<(String, Vec<String>)> {
    ipconfig::get_adapters()
        .unwrap_or_default()
        .into_iter()
        .map(|a| {
            (
                a.friendly_name().to_owned(),
                a.ip_addresses()
                    .iter()
                    .filter(|ip| ip.is_ipv4())
                    .map(ToString::to_string)
                    .collect(),
            )
        })
        .collect()
}
pub fn eligible(c: &Config) -> bool {
    if c.adapter.is_empty() && c.ip.trim().is_empty() {
        return true;
    }
    adapters().iter().any(|(name, ips)| {
        (c.adapter.is_empty() || name == &c.adapter)
            && !ips.is_empty()
            && (c.ip.trim().is_empty() || ips.iter().any(|ip| ip == c.ip.trim()))
    })
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Snapshot {
    pub used: f64,
    pub total: Option<f64>,
    pub lifetime: f64,
    pub reset_day: Option<u32>,
    pub updated: chrono::DateTime<Local>,
}
pub fn next_reset(today: NaiveDate, day: u32) -> NaiveDate {
    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        (1..=d.min(31))
            .rev()
            .find_map(|day| NaiveDate::from_ymd_opt(y, m, day))
            .unwrap()
    }
    let this = date(today.year(), today.month(), day);
    if this > today {
        this
    } else {
        let (y, m) = if today.month() == 12 {
            (today.year() + 1, 1)
        } else {
            (today.year(), today.month() + 1)
        };
        date(y, m, day)
    }
}
pub fn bytes(v: f64) -> String {
    if v >= 1024_f64.powi(4) {
        format!("{:.2} TB", v / 1024_f64.powi(4))
    } else {
        format!("{:.2} GB", v / 1024_f64.powi(3))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dates() {
        assert_eq!(
            next_reset(NaiveDate::from_ymd_opt(2026, 12, 31).unwrap(), 1).to_string(),
            "2027-01-01"
        );
        assert_eq!(
            next_reset(NaiveDate::from_ymd_opt(2028, 2, 1).unwrap(), 31).to_string(),
            "2028-02-29"
        );
    }
    #[test]
    fn private_address() {
        assert!(origin("http://10.0.0.1/html/home.html").is_ok());
        assert!(origin("https://example.com").is_err());
        assert!(origin("http://8.8.8.8").is_err());
    }
    #[test]
    fn password_roundtrip() {
        let p = protect("测试 secret", false).unwrap();
        assert_ne!(p, "测试 secret");
        assert_eq!(protect(&p, true).unwrap(), "测试 secret");
    }
}
