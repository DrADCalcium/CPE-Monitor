#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod runtime;
mod storage;
use zeroize::{Zeroize, Zeroizing};
mod fluent;
mod model;
mod router;
mod window;
mod worker;

use fluent::{SETTINGS_SIZE, WIDGET_SIZE, apply_appearance};
use model::{Config, Snapshot};
use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    time::{Duration, Instant},
};
use tray_icon::{
    TrayIcon, TrayIconBuilder,
    menu::{CheckMenuItem, Menu, MenuEvent, MenuItem, PredefinedMenuItem},
};

enum TrayAction {
    Toggle,
    Autostart,
    Exit,
}
struct App {
    config: Config,
    draft: Config,
    password: Zeroizing<String>,
    font_filter: String,
    interval_text: String,
    settings: bool,
    error: String,
    snapshot: Option<Snapshot>,
    status: String,
    online: bool,
    tx: mpsc::Sender<worker::Command>,
    rx: mpsc::Receiver<worker::Update>,
    _tray: Option<TrayIcon>,
    tray_rx: mpsc::Receiver<TrayAction>,
    toggle: Option<MenuItem>,
    startup: Option<CheckMenuItem>,
    visible: Arc<AtomicBool>,
    adapter_list: Vec<(String, Vec<String>)>,
    capture: Option<String>,
    capture_at: Instant,
    capture_sent: bool,
    quitting: bool,
    native: Option<window::Controller>,
    restore_widget: bool,
    appearance_due: Option<Instant>,
}
fn icon() -> image::RgbaImage {
    image::load_from_memory(include_bytes!("../CTM.ico"))
        .expect("icon")
        .into_rgba8()
}
impl App {
    fn new(
        cc: &crate::runtime::CreationContext<'_>,
        capture: Option<String>,
        settings: bool,
    ) -> Self {
        let mut config = model::load_config();
        if capture.is_some() {
            for arg in std::env::args() {
                if let Some(v) = arg.strip_prefix("--preview-theme=") {
                    config.theme = v.into();
                }
                if let Some(v) = arg.strip_prefix("--preview-scale=") {
                    config.ui_scale = v.parse().unwrap_or(1.);
                }
                if let Some(v) = arg.strip_prefix("--preview-font=") {
                    config.font = v.into();
                }
            }
        }
        let native = window::Controller::install(cc, window::load(), config.ui_scale);
        apply_appearance(&cc.egui_ctx, &config);
        if settings || native.is_none() {
            cc.egui_ctx
                .send_viewport_cmd(egui::ViewportCommand::InnerSize(if settings {
                    SETTINGS_SIZE
                } else {
                    WIDGET_SIZE
                }));
        }
        let snapshot = storage::read("cache.json")
            .ok()
            .and_then(|b| serde_json::from_slice(&b).ok());
        let (tx, rx) = if capture.is_none() {
            worker::start(config.clone(), cc.egui_ctx.clone())
        } else {
            let (tx, _) = mpsc::channel();
            let (_, rx) = mpsc::channel();
            (tx, rx)
        };
        let (tray_tx, tray_rx) = mpsc::channel();
        let visible = Arc::new(AtomicBool::new(true));
        let mut tray = None;
        let mut toggle = None;
        let mut startup = None;
        if capture.is_none() {
            let menu = Menu::new();
            let hide = MenuItem::new("隐藏窗口", true, None);
            let refresh = MenuItem::new("立即刷新", true, None);
            let auto = CheckMenuItem::new("开机自启", true, config.autostart, None);
            let quit = MenuItem::new("退出", true, None);
            let result = (|| -> anyhow::Result<TrayIcon> {
                menu.append_items(&[
                    &hide,
                    &refresh,
                    &auto,
                    &PredefinedMenuItem::separator(),
                    &quit,
                ])?;
                let img = icon();
                let ti =
                    tray_icon::Icon::from_rgba(img.as_raw().clone(), img.width(), img.height())?;
                Ok(TrayIconBuilder::new()
                    .with_icon(ti)
                    .with_tooltip("CPE 流量监控")
                    .with_menu(Box::new(menu))
                    .build()?)
            })();
            match result {
                Ok(t) => tray = Some(t),
                Err(e) => {
                    eprintln!("Tray: {e}");
                }
            }
            let (hide_id, refresh_id, auto_id, quit_id) = (
                hide.id().clone(),
                refresh.id().clone(),
                auto.id().clone(),
                quit.id().clone(),
            );
            let ctx = cc.egui_ctx.clone();
            let sender = tx.clone();
            let events = tray_tx.clone();
            MenuEvent::set_event_handler(Some(move |e: MenuEvent| {
                if e.id == hide_id {
                    let _ = events.send(TrayAction::Toggle);
                } else if e.id == refresh_id {
                    let _ = sender.send(worker::Command::Refresh);
                } else if e.id == auto_id {
                    let _ = events.send(TrayAction::Autostart);
                } else if e.id == quit_id {
                    let _ = events.send(TrayAction::Exit);
                }
                ctx.request_repaint();
            }));
            let ctx = cc.egui_ctx.clone();
            let events = tray_tx;
            let v = visible.clone();
            tray_icon::TrayIconEvent::set_event_handler(Some(move |e| {
                if matches!(
                    e,
                    tray_icon::TrayIconEvent::DoubleClick {
                        button: tray_icon::MouseButton::Left,
                        ..
                    }
                ) {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
                    ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
                    if !v.load(Ordering::Relaxed) {
                        let _ = events.send(TrayAction::Toggle);
                    }
                    ctx.request_repaint();
                }
            }));
            toggle = Some(hide);
            startup = Some(auto);
        }
        let mut app = Self {
            draft: config.clone(),
            interval_text: config.interval.to_string(),
            font_filter: String::new(),
            config,
            password: Zeroizing::new(String::new()),
            settings: false,
            error: String::new(),
            snapshot,
            status: if capture.is_some() {
                "已连接"
            } else {
                "等待连接…"
            }
            .into(),
            online: capture.is_some(),
            tx,
            rx,
            _tray: tray,
            tray_rx,
            toggle,
            startup,
            visible,
            adapter_list: model::adapters(),
            capture,
            capture_at: Instant::now(),
            capture_sent: false,
            quitting: false,
            native,
            restore_widget: !settings,
            appearance_due: None,
        };
        if settings {
            app.open_settings(&cc.egui_ctx);
        }
        app
    }
    fn set_visible(&mut self, ctx: &egui::Context, show: bool) {
        self.visible.store(show, Ordering::Relaxed);
        ctx.send_viewport_cmd(egui::ViewportCommand::Visible(show));
        if show {
            ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
        }
        if let Some(t) = &self.toggle {
            t.set_text(if show { "隐藏窗口" } else { "显示窗口" });
        }
    }
    fn open_settings(&mut self, ctx: &egui::Context) {
        self.appearance_due = None;
        self.draft = self.config.clone();
        self.interval_text = self.draft.interval.to_string();
        self.font_filter.clear();
        self.password.zeroize();
        self.adapter_list = model::adapters();
        if let Some(n) = &self.native {
            n.set_settings(true);
        }
        self.settings = true;
        self.error.clear();
        ctx.set_zoom_factor(1.);
        ctx.send_viewport_cmd(egui::ViewportCommand::Resizable(false));
        ctx.send_viewport_cmd(egui::ViewportCommand::Decorations(true));
        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(SETTINGS_SIZE));
        ctx.send_viewport_cmd(egui::ViewportCommand::Title("设置 · CPE 流量监控".into()));
    }
    fn close_settings(&mut self, ctx: &egui::Context) {
        self.appearance_due = None;
        self.settings = false;
        self.restore_widget = true;
        ctx.send_viewport_cmd(egui::ViewportCommand::Resizable(true));
        apply_appearance(ctx, &self.config);
        self.password.zeroize();
        ctx.send_viewport_cmd(egui::ViewportCommand::Decorations(false));
        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(WIDGET_SIZE));
        ctx.send_viewport_cmd(egui::ViewportCommand::Title("CPE 流量监控".into()));
    }
    fn save(&mut self, ctx: &egui::Context) {
        let result = (|| -> anyhow::Result<()> {
            self.draft.interval = self
                .interval_text
                .trim()
                .parse()
                .map_err(|_| anyhow::anyhow!("刷新间隔请输入 1–1440 的整数"))?;
            model::validate(&self.draft)?;
            if !self.password.is_empty() {
                self.draft.password = model::protect(&self.password, false)?;
            }
            if self.draft.password.is_empty() {
                anyhow::bail!("请填写后台登录密码");
            }
            let previous = self.config.autostart;
            model::autostart(self.draft.autostart)?;
            if let Err(e) = model::save_config(&self.draft) {
                let _ = model::autostart(previous);
                return Err(e);
            }
            Ok(())
        })();
        match result {
            Ok(()) => {
                self.config = self.draft.clone();

                if let Some(a) = &self.startup {
                    a.set_checked(self.config.autostart);
                }
                let _ = self
                    .tx
                    .send(worker::Command::Configure(self.config.clone()));
                self.close_settings(ctx);
            }
            Err(e) => self.error = format!("{e:#}"),
        }
    }
}

impl App {
    fn logic(&mut self, ctx: &egui::Context) {
        let ctx = ctx.clone();
        if self.restore_widget && !self.settings {
            if let Some(n) = &self.native {
                n.set_settings(false);
            }
            self.restore_widget = false;
            ctx.request_repaint();
        }
        while let Ok(u) = self.rx.try_recv() {
            if let Some(s) = u.snapshot {
                self.snapshot = Some(s);
            }
            self.status = u.status;
            self.online = u.online;
        }
        while let Ok(e) = self.tray_rx.try_recv() {
            match e {
                TrayAction::Toggle => {
                    let show = !self.visible.load(Ordering::Relaxed);
                    self.set_visible(&ctx, show);
                }
                TrayAction::Exit => {
                    self.quitting = true;
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
                TrayAction::Autostart => {
                    self.set_visible(&ctx, true);
                    let mut c = self.config.clone();
                    c.autostart = !c.autostart;
                    match model::autostart(c.autostart).and_then(|_| model::save_config(&c)) {
                        Ok(()) => {
                            self.config = c;
                            self.draft.autostart = self.config.autostart;
                        }
                        Err(e) => {
                            let _ = model::autostart(self.config.autostart);
                            self.error = e.to_string();
                            self.open_settings(&ctx);
                            self.error = e.to_string();
                        }
                    }
                    if let Some(a) = &self.startup {
                        a.set_checked(self.config.autostart);
                    }
                }
            }
        }
        if self.settings && !self.quitting && ctx.input(|i| i.viewport().close_requested()) {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            self.close_settings(&ctx);
        }
    }
    fn ui(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();
        if self.settings {
            let had_popup = egui::Popup::is_any_open(&ctx);
            self.settings_ui(ui, &ctx);
            if !had_popup && ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                self.close_settings(&ctx);
            }
        } else {
            if let Some(n) = &self.native {
                let scale = n.scale().clamp(0.65, 2.);
                if (ctx.zoom_factor() - scale).abs() > 0.003 {
                    ctx.set_zoom_factor(scale);
                    ctx.request_repaint();
                }
            }
            self.widget(ui, &ctx);
        }
        if let Some(path) = &self.capture {
            if !self.capture_sent && self.capture_at.elapsed() > Duration::from_secs(3) {
                ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(egui::UserData::default()));
                self.capture_sent = true;
            }
            for event in ctx.input(|i| i.events.clone()) {
                if let egui::Event::Screenshot { image, .. } = event {
                    let rgba: Vec<u8> = image.pixels.iter().flat_map(|c| c.to_array()).collect();
                    let _ = image::save_buffer(
                        path,
                        &rgba,
                        image.width() as u32,
                        image.height() as u32,
                        image::ColorType::Rgba8,
                    );
                    self.quitting = true;
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            }
            ctx.request_repaint_after(Duration::from_millis(100));
        }
    }
}
fn main() -> anyhow::Result<()> {
    if let Err(e) = initialize_data() {
        let message: Vec<u16> = format!("配置初始化失败：{e}\0").encode_utf16().collect();
        let title: Vec<u16> = "CPE 流量监控\0".encode_utf16().collect();
        unsafe {
            windows_sys::Win32::UI::WindowsAndMessaging::MessageBoxW(
                std::ptr::null_mut(),
                message.as_ptr(),
                title.as_ptr(),
                0x10,
            );
        }
        return Err(e);
    }
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|s| s == "--font-report") {
        let reports: Vec<_> = ["MiSans", "Microsoft YaHei", "Segoe UI"]
            .iter()
            .map(|s| fluent::font_report(s))
            .collect();

        let _ = storage::write(
            "font-report.json",
            &serde_json::to_vec_pretty(&reports).unwrap(),
        );
        return Ok(());
    }
    if args.iter().any(|s| s == "--probe") {
        let c = model::load_config();
        let result = (|| -> anyhow::Result<Snapshot> {
            model::validate(&c)?;
            let mut r = router::Router::new(&c)?;
            r.login(&c)?;
            r.refresh(&c)
        })();

        let report = match result {
            Ok(s) => {
                let _ = storage::write("cache.json", &serde_json::to_vec_pretty(&s).unwrap());
                serde_json::json!({"ok":true,"snapshot":s})
            }
            Err(e) => serde_json::json!({"ok":false,"error":router::describe(&e)}),
        };
        let _ = storage::write("probe.json", &serde_json::to_vec_pretty(&report).unwrap());
        return Ok(());
    }
    let capture = args
        .iter()
        .find_map(|s| s.strip_prefix("--capture=").map(str::to_owned));
    // Named mutex keeps autostart/manual launch from producing duplicate pollers.
    let _mutex = if capture.is_none() {
        unsafe {
            use windows_sys::Win32::{
                Foundation::{ERROR_ALREADY_EXISTS, GetLastError},
                System::Threading::CreateMutexW,
            };
            let name: Vec<u16> = "Local\\CPE-Monitor-7AA151C7\0".encode_utf16().collect();
            let h = CreateMutexW(std::ptr::null(), 0, name.as_ptr());
            if GetLastError() == ERROR_ALREADY_EXISTS {
                return Ok(());
            }
            h
        }
    } else {
        std::ptr::null_mut()
    };
    let c = model::load_config();
    let settings = args.iter().any(|s| s == "--settings");
    let img = icon();
    let mut viewport = egui::ViewportBuilder::default()
        .with_title("CPE 流量监控")
        .with_decorations(false)
        .with_transparent(false)
        .with_inner_size(if settings { SETTINGS_SIZE } else { WIDGET_SIZE })
        .with_resizable(!settings)
        .with_icon(egui::IconData {
            rgba: img.as_raw().clone(),
            width: img.width(),
            height: img.height(),
        });
    if let Some(p) = c.position {
        viewport = viewport.with_position(p);
    }
    runtime::run(viewport, capture, settings)
}

fn initialize_data() -> anyhow::Result<()> {
    storage::init()?;
    let path = model::data_dir().join("config.json");
    if !path.exists() {
        model::save_config(&Config::default())?;
    }
    let bytes = storage::read("config.json")?;
    let c: Config = serde_json::from_slice(&bytes)
        .map_err(|_| anyhow::anyhow!("config.json 格式损坏，原文件已保留"))?;
    if !c.password.is_empty() {
        let _plain = Zeroizing::new(model::protect(&c.password, true)?);
    }
    Ok(())
}
