// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (C) 2026 CPE Monitor contributors.
use crate::{
    model::{self, Config, Snapshot},
    router::{self, Router},
};
use std::{
    sync::mpsc::{self, Receiver, Sender},
    time::{Duration, Instant},
};
pub enum Command {
    Refresh,
    Configure(Config),
}
pub struct Update {
    pub snapshot: Option<Snapshot>,
    pub status: String,
    pub online: bool,
}
pub fn start(mut c: Config, wake: crate::runtime::Wake) -> (Sender<Command>, Receiver<Update>) {
    let (tx, rx) = mpsc::channel();
    let (out, input) = mpsc::channel();
    std::thread::spawn(move || {
        let mut router: Option<Router> = None;
        let mut next = Instant::now();
        let mut blocked = false;
        let mut paused = false;
        loop {
            let wait = next
                .saturating_duration_since(Instant::now())
                .min(Duration::from_secs(15));
            match rx.recv_timeout(wait) {
                Ok(Command::Configure(new)) => {
                    c = new;
                    router = None;
                    blocked = false;
                    next = Instant::now();
                }
                Ok(Command::Refresh) => {
                    next = Instant::now();
                }
                Err(mpsc::RecvTimeoutError::Disconnected) => break,
                Err(mpsc::RecvTimeoutError::Timeout) => (),
            }
            if !model::eligible(&c) {
                if !paused {
                    let _ = out.send(Update {
                        snapshot: None,
                        status: "网络条件不匹配 · 已暂停刷新".into(),
                        online: false,
                    });
                    wake.now();
                    paused = true;
                }
                router = None;
                next = Instant::now() + Duration::from_secs(15);
                continue;
            }
            if paused {
                next = Instant::now();
                paused = false;
            }
            if Instant::now() < next {
                continue;
            }
            if blocked || c.password.is_empty() || c.router.is_empty() {
                let _ = out.send(Update {
                    snapshot: None,
                    status: if blocked {
                        "登录已暂停，请修改密码或稍后重新保存设置"
                    } else {
                        "请在设置中填写路由器地址和后台密码"
                    }
                    .into(),
                    online: false,
                });
                wake.now();
                next = Instant::now() + Duration::from_secs(60);
                continue;
            }
            let _ = out.send(Update {
                snapshot: None,
                status: "正在读取 CPE…".into(),
                online: false,
            });
            wake.now();
            let result = (|| -> anyhow::Result<Snapshot> {
                if router.is_none() {
                    let mut r = Router::new(&c)?;
                    r.login(&c)?;
                    router = Some(r);
                }
                let r = router.as_mut().unwrap();
                r.refresh(&c)
            })();
            let update = match result {
                Ok(s) => {
                    if let Ok(b) = serde_json::to_vec(&s) {
                        let _ = crate::storage::write("cache.json", &b);
                    }
                    Update {
                        snapshot: Some(s),
                        status: "已连接".into(),
                        online: true,
                    }
                }
                Err(e) => {
                    blocked = router::auth_failure(&e);
                    router = None;
                    Update {
                        snapshot: None,
                        status: router::describe(&e),
                        online: false,
                    }
                }
            };
            let _ = out.send(update);
            wake.now();
            next = Instant::now() + Duration::from_secs(c.interval as u64 * 60);
        }
    });
    (tx, input)
}
