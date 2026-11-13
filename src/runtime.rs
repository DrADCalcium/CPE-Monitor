// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (C) 2026 CPE Monitor contributors.
//! Event-driven Win32 GDI host. The rasterizer writes directly into a top-down
//! DIB section; there is no GL context, GPU driver, browser or frame timer.
use egui_software_backend::{BufferMutRef, ColorFieldOrder, EguiSoftwareRender};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use std::{sync::Arc, time::Instant};
use windows_sys::Win32::{Foundation::HWND, Graphics::Gdi::*};
use winit::platform::windows::{WindowAttributesExtWindows, WindowExtWindows};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

#[derive(Clone)]
pub struct Wake(pub(crate) Arc<dyn Fn(Instant) + Send + Sync>);
impl Wake {
    pub fn now(&self) {
        (self.0)(Instant::now());
    }
    fn context(&self) -> egui::Context {
        let ctx = egui::Context::default();
        let schedule = self.0.clone();
        ctx.set_request_repaint_callback(move |info| {
            if let Some(when) = Instant::now().checked_add(info.delay) {
                schedule(when);
            }
        });
        ctx
    }
}
pub struct CreationContext<'a> {
    pub egui_ctx: egui::Context,
    pub window: &'a Window,
    pub wake: &'a Wake,
}
pub(crate) struct Surface {
    pub(crate) dc: HDC,
    bitmap: HBITMAP,
    previous: HGDIOBJ,
    bits: *mut [u8; 4],
    width: usize,
    height: usize,
}
impl Surface {
    pub(crate) fn new(hwnd: HWND, width: usize, height: usize) -> anyhow::Result<Self> {
        anyhow::ensure!(
            width > 0 && height > 0 && width <= 16384 && height <= 16384,
            "窗口尺寸无效"
        );
        unsafe {
            let screen = GetDC(hwnd);
            let dc = CreateCompatibleDC(screen);
            let info = BITMAPINFO {
                bmiHeader: BITMAPINFOHEADER {
                    biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                    biWidth: width as i32,
                    biHeight: -(height as i32),
                    biPlanes: 1,
                    biBitCount: 32,
                    biCompression: BI_RGB,
                    ..Default::default()
                },
                ..Default::default()
            };
            let mut bits = std::ptr::null_mut();
            let bitmap = CreateDIBSection(
                screen,
                &info,
                DIB_RGB_COLORS,
                &mut bits,
                std::ptr::null_mut(),
                0,
            );
            ReleaseDC(hwnd, screen);
            if bitmap.is_null() || dc.is_null() {
                if !dc.is_null() {
                    DeleteDC(dc);
                }
                if !bitmap.is_null() {
                    DeleteObject(bitmap);
                }
                anyhow::bail!("无法创建 GDI 绘图缓冲区");
            }
            let previous = SelectObject(dc, bitmap);
            Ok(Self {
                dc,
                bitmap,
                previous,
                bits: bits.cast(),
                width,
                height,
            })
        }
    }
    pub(crate) fn pixels(&mut self) -> &mut [[u8; 4]] {
        unsafe { std::slice::from_raw_parts_mut(self.bits, self.width * self.height) }
    }
    fn present(&self, hwnd: HWND) {
        unsafe {
            let screen = GetDC(hwnd);
            BitBlt(
                screen,
                0,
                0,
                self.width as i32,
                self.height as i32,
                self.dc,
                0,
                0,
                SRCCOPY,
            );
            ReleaseDC(hwnd, screen);
        }
    }
}
impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            SelectObject(self.dc, self.previous);
            DeleteObject(self.bitmap);
            DeleteDC(self.dc);
        }
    }
}
struct Host {
    wake: Wake,
    overlay: crate::taskbar::Taskbar,
    overlay_due: Option<Instant>,
    ctx: egui::Context,
    window: Option<Window>,
    state: Option<egui_winit::State>,
    app: Option<crate::App>,
    renderer: EguiSoftwareRender,
    surface: Option<Surface>,
    builder: egui::ViewportBuilder,
    info: egui::ViewportInfo,
    events: Vec<egui::Event>,
    capture: Option<String>,
    settings: bool,
    visible: bool,
    deadline: Option<Instant>,
    dirty: bool,
    closing: bool,
    region: Option<(bool, u32, u32)>,
    dark_theme: Option<bool>,
    taskbar_settings: Option<bool>,
    error: Option<String>,
}
fn hwnd(window: &Window) -> HWND {
    match window.window_handle().unwrap().as_raw() {
        RawWindowHandle::Win32(h) => h.hwnd.get() as HWND,
        _ => unreachable!(),
    }
}
impl Host {
    fn frame(&mut self, el: &ActiveEventLoop) {
        let Some(window) = self.window.as_ref() else {
            return;
        };
        let was_visible = self.visible;
        let state = self.state.as_mut().unwrap();
        egui_winit::update_viewport_info(&mut self.info, &self.ctx, window, false);
        if self.closing {
            self.info.events.push(egui::ViewportEvent::Close);
        }
        let mut input = state.take_egui_input(window);
        input
            .viewports
            .insert(egui::ViewportId::ROOT, self.info.clone());
        input.events.append(&mut self.events);
        let app = self.app.as_mut().unwrap();
        let (platform, commands, delay, actions) = if self.visible {
            let out = self.ctx.run_ui(input, |ui| {
                app.logic(&ui.ctx().clone());
                app.ui(ui);
            });
            let size = window.inner_size();
            if size.width > 0 && size.height > 0 {
                let w = size.width as usize;
                let h = size.height as usize;
                if self
                    .surface
                    .as_ref()
                    .is_none_or(|s| s.width != w || s.height != h)
                {
                    match Surface::new(hwnd(window), w, h) {
                        Ok(s) => self.surface = Some(s),
                        Err(e) => {
                            self.error = Some(e.to_string());
                            el.exit();
                            return;
                        }
                    }
                }
                let surface = self.surface.as_mut().unwrap();
                surface.pixels().fill([0, 0, 0, 255]);
                let primitives = self.ctx.tessellate(out.shapes, out.pixels_per_point);
                self.renderer.render(
                    &mut BufferMutRef::new(surface.pixels(), w, h),
                    &primitives,
                    &out.textures_delta,
                    out.pixels_per_point,
                );
                surface.present(hwnd(window));
            }
            let vp = out.viewport_output.get(&egui::ViewportId::ROOT).unwrap();
            (
                out.platform_output,
                vp.commands.clone(),
                vp.repaint_delay,
                true,
            )
        } else {
            let out = self.ctx.run_logic(&input, |ctx| app.logic(ctx));
            (
                out.platform_output,
                out.viewport_commands
                    .get(&egui::ViewportId::ROOT)
                    .cloned()
                    .unwrap_or_default(),
                std::time::Duration::MAX,
                false,
            )
        };
        state.handle_platform_output_with_event_loop(window, el, platform);
        self.closing |= commands
            .iter()
            .any(|c| matches!(c, egui::ViewportCommand::Close));
        if commands
            .iter()
            .any(|c| matches!(c, egui::ViewportCommand::CancelClose))
        {
            self.closing = false;
        }
        for c in &commands {
            if let egui::ViewportCommand::Visible(v) = c {
                self.visible = *v;
            }
        }
        if let Some(native) = &app.native {
            native.prepare_widget_mode(!app.settings);
        }
        let mut requested = Vec::new();
        egui_winit::process_viewport_commands(
            &self.ctx,
            &mut self.info,
            commands,
            window,
            &mut requested,
        );
        if self.taskbar_settings != Some(app.settings) {
            window.set_skip_taskbar(!app.settings);
            self.taskbar_settings = Some(app.settings);
        }
        // The Windows shadow workaround reserves an unpainted one-pixel strip
        // above the client area. Our rounded region supplies the card outline.
        window.set_undecorated_shadow(false);
        let dark = self.ctx.theme() == egui::Theme::Dark;
        if self.dark_theme != Some(dark) {
            self.dark_theme = Some(dark);
            window.set_theme(Some(if dark {
                winit::window::Theme::Dark
            } else {
                winit::window::Theme::Light
            }));
        }
        self.info.events.clear();
        if self.closing {
            el.exit();
            return;
        }
        // The OS clips rounded corners without requiring a layered alpha window.
        let region_key = (
            app.settings,
            window.inner_size().width,
            window.inner_size().height,
        );
        if self.region != Some(region_key) {
            self.region = Some(region_key);
            unsafe {
                if app.settings {
                    SetWindowRgn(hwnd(window), std::ptr::null_mut(), 1);
                    // A docked widget may be at the right/bottom edge. Keep the
                    // larger settings window entirely inside its monitor's work area.
                    use windows_sys::Win32::{Foundation::RECT, UI::WindowsAndMessaging::*};
                    let mut r = RECT::default();
                    GetWindowRect(hwnd(window), &mut r);
                    let mut monitor = MONITORINFO {
                        cbSize: std::mem::size_of::<MONITORINFO>() as u32,
                        ..Default::default()
                    };
                    GetMonitorInfoW(
                        MonitorFromWindow(hwnd(window), MONITOR_DEFAULTTONEAREST),
                        &mut monitor,
                    );
                    let work = monitor.rcWork;
                    let x = r
                        .left
                        .clamp(work.left, (work.right - (r.right - r.left)).max(work.left));
                    let y = r
                        .top
                        .clamp(work.top, (work.bottom - (r.bottom - r.top)).max(work.top));
                    if x != r.left || y != r.top {
                        SetWindowPos(
                            hwnd(window),
                            std::ptr::null_mut(),
                            x,
                            y,
                            0,
                            0,
                            SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE,
                        );
                    }
                } else {
                    let s = window.inner_size();
                    let r = (16.0 * window.scale_factor()) as i32;
                    let region =
                        CreateRoundRectRgn(0, 0, s.width as i32 + 1, s.height as i32 + 1, r, r);
                    if SetWindowRgn(hwnd(window), region, 1) == 0 {
                        DeleteObject(region);
                    }
                }
            }
        }
        for action in requested {
            if let egui_winit::ActionRequested::Screenshot(user_data) = action {
                if let Some(s) = self.surface.as_mut() {
                    let w = s.width;
                    let h = s.height;
                    let rgba: Vec<u8> = s
                        .pixels()
                        .iter()
                        .flat_map(|p| [p[2], p[1], p[0], 255])
                        .collect();
                    self.events.push(egui::Event::Screenshot {
                        viewport_id: egui::ViewportId::ROOT,
                        user_data,
                        image: Arc::new(egui::ColorImage::from_rgba_unmultiplied([w, h], &rgba)),
                    });
                    self.dirty = true;
                }
            }
        }
        if was_visible && !self.visible {
            // Background producers only hold Wake, so replacing the context releases
            // the CJK fonts, glyph atlas, layout caches and software textures together.
            self.surface = None;
            self.renderer = new_renderer();
            self.ctx = self.wake.context();
            self.state = Some(new_state(&self.ctx, window));
            self.dark_theme = None;
            self.deadline = None;
        } else if !was_visible && self.visible {
            crate::fluent::apply_appearance(
                &self.ctx,
                if app.settings {
                    &app.draft
                } else {
                    &app.config
                },
            );
            self.dirty = true;
        }
        if actions && self.visible {
            if let Some(d) = Instant::now().checked_add(delay) {
                self.deadline = Some(self.deadline.map_or(d, |v| v.min(d)));
            }
        }
    }
}
impl ApplicationHandler<Instant> for Host {
    fn resumed(&mut self, el: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        // Build hidden and without winit's undecorated shadow workaround.
        // The first frame and native widget styles are installed before showing.
        let hidden = self.builder.clone().with_visible(false);
        let attributes = egui_winit::create_winit_window_attributes(&self.ctx, hidden.clone())
            .with_undecorated_shadow(false)
            .with_skip_taskbar(!self.settings);
        let window = match el.create_window(attributes) {
            Ok(w) => w,
            Err(e) => {
                self.error = Some(e.to_string());
                el.exit();
                return;
            }
        };
        egui_winit::apply_viewport_builder_to_window(&self.ctx, &window, &hidden);
        self.state = Some(new_state(&self.ctx, &window));
        egui_winit::update_viewport_info(&mut self.info, &self.ctx, &window, true);
        self.app = Some(crate::App::new(
            &CreationContext {
                egui_ctx: self.ctx.clone(),
                window: &window,
                wake: &self.wake,
            },
            self.capture.take(),
            self.settings,
        ));
        self.window = Some(window);
        self.frame(el);
        if self.error.is_none() {
            if let Some(window) = &self.window {
                window.set_visible(true);
            }
        }
        self.dirty = true;
    }
    fn user_event(&mut self, _: &ActiveEventLoop, when: Instant) {
        self.deadline = Some(self.deadline.map_or(when, |v| v.min(when)));
    }
    fn window_event(&mut self, el: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        let Some(window) = self.window.as_ref() else {
            return;
        };
        if window.id() != id {
            return;
        }
        let response = self.state.as_mut().unwrap().on_window_event(window, &event);
        match event {
            WindowEvent::CloseRequested => {
                self.closing = true;
                self.dirty = true;
            }
            WindowEvent::RedrawRequested => {
                self.dirty = false;
                self.frame(el);
            }
            WindowEvent::Resized(_) | WindowEvent::ScaleFactorChanged { .. } => self.dirty = true,
            _ => {
                if response.repaint {
                    self.dirty = true;
                }
            }
        }
    }
    fn about_to_wait(&mut self, el: &ActiveEventLoop) {
        let now = Instant::now();
        if self.deadline.is_some_and(|v| v <= now) {
            self.dirty = true;
            self.deadline = None;
        }
        if self.dirty {
            self.dirty = false;
            if self.visible {
                if let Some(w) = &self.window {
                    w.request_redraw();
                }
            } else {
                self.frame(el);
                if self.visible {
                    self.dirty = true;
                }
            }
        }
        // Shell layout and appearance can change while no egui window is visible.
        // Keep this timer independent of egui's repaint deadline.
        if let Some(app) = &self.app {
            let config = if app.settings {
                &app.draft
            } else {
                &app.config
            };
            if config.taskbar.enabled && app.capture.is_none() {
                if self.overlay_due.is_none_or(|t| t <= now) {
                    self.overlay.sync(config, app.snapshot.as_ref());
                    self.overlay_due = Some(now + std::time::Duration::from_secs(1));
                }
            } else {
                self.overlay.close();
                self.overlay_due = None;
            }
        }
        el.set_control_flow(
            self.deadline
                .into_iter()
                .chain(self.overlay_due)
                .min()
                .map_or(ControlFlow::Wait, ControlFlow::WaitUntil),
        );
    }
}
fn new_renderer() -> EguiSoftwareRender {
    EguiSoftwareRender::new(ColorFieldOrder::Bgra).with_caching(false)
}
fn new_state(ctx: &egui::Context, window: &Window) -> egui_winit::State {
    egui_winit::State::new(
        ctx.clone(),
        egui::ViewportId::ROOT,
        window,
        Some(window.scale_factor() as f32),
        window.theme(),
        Some(2048),
    )
}
pub fn run(
    builder: egui::ViewportBuilder,
    capture: Option<String>,
    settings: bool,
) -> anyhow::Result<()> {
    let el = EventLoop::<Instant>::with_user_event().build()?;
    let proxy = el.create_proxy();
    let wake = Wake(Arc::new(move |when| {
        let _ = proxy.send_event(when);
    }));
    let ctx = wake.context();
    let mut host = Host {
        ctx,
        wake,
        overlay: Default::default(),
        overlay_due: None,
        window: None,
        state: None,
        app: None,
        renderer: new_renderer(),
        surface: None,
        builder,
        info: Default::default(),
        events: Vec::new(),
        capture,
        settings,
        visible: true,
        deadline: None,
        dirty: true,
        closing: false,
        region: None,
        dark_theme: None,
        taskbar_settings: None,
        error: None,
    };
    el.run_app(&mut host)?;
    if let Some(e) = host.error {
        anyhow::bail!(e)
    }
    Ok(())
}
