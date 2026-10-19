// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (C) 2026 CPE Monitor contributors.
//! Native hit testing keeps dragging in the compositor. No rendering or disk I/O
//! runs in the move/size message path; only the completed placement is queued.

use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use serde::{Deserialize, Serialize};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
    mpsc,
};
use windows_sys::Win32::{
    Foundation::*,
    Graphics::Gdi::*,
    UI::{HiDpi::GetDpiForWindow, Shell::*, WindowsAndMessaging::*},
};
const RATIO: f64 = 340.0 / 232.0;
const ID: usize = 0x43504503;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct Placement {
    pub x: i32,
    pub y: i32,
    pub width_dip: f32,
    pub height_dip: f32,
}
pub fn load() -> Option<Placement> {
    crate::storage::read("window.json")
        .ok()
        .and_then(|b| serde_json::from_slice::<Placement>(&b).ok())
        .filter(|p| {
            p.width_dip.is_finite()
                && p.height_dip.is_finite()
                && p.width_dip >= 221.
                && p.width_dip <= 680.
                && ((p.width_dip / p.height_dip) as f64 - RATIO).abs() < 0.02
        })
}
struct Shared {
    enabled: AtomicBool,
    moving: AtomicBool,
    ctx: egui::Context,
    placement: Mutex<Placement>,
    move_origin: Mutex<Option<(POINT, RECT)>>,
    writer: mpsc::Sender<Placement>,
}
pub struct Controller {
    hwnd: isize,
    shared: Arc<Shared>,
}
fn writer() -> mpsc::Sender<Placement> {
    let (tx, rx) = mpsc::channel::<Placement>();
    std::thread::spawn(move || {
        while let Ok(mut p) = rx.recv() {
            while let Ok(new) = rx.try_recv() {
                p = new;
            }
            if let Ok(b) = serde_json::to_vec_pretty(&p) {
                let _ = crate::storage::write("window.json", &b);
            }
        }
    });
    tx
}
unsafe fn dpi(hwnd: HWND) -> f64 {
    (unsafe { GetDpiForWindow(hwnd) }).max(96) as f64 / 96.
}
unsafe fn geometry(hwnd: HWND) -> Placement {
    unsafe {
        let mut r = RECT::default();
        let mut c = RECT::default();
        GetWindowRect(hwnd, &mut r);
        GetClientRect(hwnd, &mut c);
        let d = dpi(hwnd);
        Placement {
            x: r.left,
            y: r.top,
            width_dip: ((c.right - c.left) as f64 / d) as f32,
            height_dip: ((c.bottom - c.top) as f64 / d) as f32,
        }
    }
}
pub fn snap(mut r: RECT, work: RECT, threshold: i32) -> RECT {
    let w = r.right - r.left;
    let h = r.bottom - r.top;
    if (r.left - work.left).abs() <= threshold {
        r.left = work.left;
        r.right = r.left + w;
    } else if (r.right - work.right).abs() <= threshold {
        r.right = work.right;
        r.left = r.right - w;
    }
    if (r.top - work.top).abs() <= threshold {
        r.top = work.top;
        r.bottom = r.top + h;
    } else if (r.bottom - work.bottom).abs() <= threshold {
        r.bottom = work.bottom;
        r.top = r.bottom - h;
    }
    r
}
fn moved_from_origin(original: RECT, start: POINT, cursor: POINT) -> RECT {
    let dx = cursor.x - start.x;
    let dy = cursor.y - start.y;
    RECT {
        left: original.left + dx,
        top: original.top + dy,
        right: original.right + dx,
        bottom: original.bottom + dy,
    }
}
pub fn constrain(mut r: RECT, edge: usize, d: f64, frame: [i32; 2]) -> RECT {
    let fw = frame[0] as f64;
    let fh = frame[1] as f64;
    let vertical = edge == WMSZ_TOP as usize || edge == WMSZ_BOTTOM as usize;
    let width = if vertical {
        ((r.bottom - r.top) as f64 - fh) * RATIO
    } else {
        (r.right - r.left) as f64 - fw
    };
    let width = width.clamp(221. * d, 680. * d).round() as i32 + frame[0];
    let height = (((width - frame[0]) as f64 / RATIO).round() as i32) + frame[1];
    if matches!(edge, 1 | 4 | 7) {
        r.left = r.right - width;
    } else if vertical {
        let center = (r.left + r.right) / 2;
        r.left = center - width / 2;
        r.right = r.left + width;
    } else {
        r.right = r.left + width;
    }
    if matches!(edge, 3 | 4 | 5) {
        r.top = r.bottom - height;
    } else if matches!(edge, 1 | 2) {
        let center = (r.top + r.bottom) / 2;
        r.top = center - height / 2;
        r.bottom = r.top + height;
    } else {
        r.bottom = r.top + height;
    }
    r
}
impl Controller {
    pub fn install(
        cc: &crate::runtime::CreationContext<'_>,
        saved: Option<Placement>,
        scale: f32,
        widget: bool,
    ) -> Option<Self> {
        let RawWindowHandle::Win32(handle) = cc.window.window_handle().ok()?.as_raw() else {
            return None;
        };
        let hwnd = handle.hwnd.get() as HWND;
        unsafe {
            let current = geometry(hwnd);
            let p = saved.unwrap_or(Placement {
                width_dip: 340. * scale,
                height_dip: 232. * scale,
                ..current
            });
            let shared = Arc::new(Shared {
                enabled: AtomicBool::new(widget),
                moving: AtomicBool::new(false),
                ctx: cc.egui_ctx.clone(),
                placement: Mutex::new(p),
                move_origin: Mutex::new(None),
                writer: writer(),
            });
            let ptr = Box::into_raw(Box::new(shared.clone()));
            if SetWindowSubclass(hwnd, Some(subclass), ID, ptr as usize) == 0 {
                drop(Box::from_raw(ptr));
                return None;
            }
            let controller = Self {
                hwnd: hwnd as isize,
                shared,
            };
            if widget {
                controller.apply_widget_frame();
            }
            Some(controller)
        }
    }
    pub fn prepare_widget_mode(&self, widget: bool) {
        let previous = self.shared.enabled.swap(widget, Ordering::Relaxed);
        if widget && !previous {
            self.apply_widget_frame();
        }
    }
    fn apply_widget_frame(&self) {
        unsafe {
            let hwnd = self.hwnd as HWND;
            let style = GetWindowLongPtrW(hwnd, GWL_STYLE) as u32;
            let ex = GetWindowLongPtrW(hwnd, GWL_EXSTYLE) as u32;
            SetWindowLongPtrW(hwnd, GWL_STYLE, widget_style(style) as isize);
            SetWindowLongPtrW(hwnd, GWL_EXSTYLE, widget_extended_style(ex) as isize);
            SetWindowPos(
                hwnd,
                std::ptr::null_mut(),
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE | SWP_FRAMECHANGED,
            );
        }
    }
    pub fn scale(&self) -> f32 {
        unsafe { geometry(self.hwnd as HWND).width_dip / 340. }
    }
    pub fn set_settings(&self, settings: bool) {
        if settings {
            self.shared.enabled.store(false, Ordering::Relaxed);
        } else {
            self.prepare_widget_mode(true);
            self.restore();
        }
    }
    pub fn restore(&self) {
        unsafe {
            let hwnd = self.hwnd as HWND;
            let p = *self.shared.placement.lock().unwrap();
            let d = dpi(hwnd);
            let mut monitor = MONITORINFO {
                cbSize: std::mem::size_of::<MONITORINFO>() as u32,
                ..Default::default()
            };
            GetMonitorInfoW(
                MonitorFromPoint(
                    POINT {
                        x: p.x + (p.width_dip * d as f32 / 2.) as i32,
                        y: p.y + (p.height_dip * d as f32 / 2.) as i32,
                    },
                    MONITOR_DEFAULTTONEAREST,
                ),
                &mut monitor,
            );
            let work = monitor.rcWork;
            let mut outer = RECT::default();
            let mut client = RECT::default();
            GetWindowRect(hwnd, &mut outer);
            GetClientRect(hwnd, &mut client);
            let w = (p.width_dip * d as f32).round() as i32
                + ((outer.right - outer.left) - (client.right - client.left));
            let h = (p.height_dip * d as f32).round() as i32
                + ((outer.bottom - outer.top) - (client.bottom - client.top));
            let x = p.x.clamp(work.left, (work.right - w).max(work.left));
            let y = p.y.clamp(work.top, (work.bottom - h).max(work.top));
            SetWindowPos(
                hwnd,
                std::ptr::null_mut(),
                x,
                y,
                w,
                h,
                SWP_NOZORDER | SWP_NOACTIVATE,
            );
        }
    }
}
// Winit keeps these flags even for an undecorated window. Removing them
// prevents DefWindowProc from drawing legacy caption buttons during a drag.
fn widget_style(style: u32) -> u32 {
    style & !(WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX | WS_MAXIMIZEBOX)
}
fn widget_extended_style(style: u32) -> u32 {
    (style & !(WS_EX_APPWINDOW | WS_EX_WINDOWEDGE | WS_EX_CLIENTEDGE | WS_EX_DLGMODALFRAME))
        | WS_EX_TOOLWINDOW
}
unsafe extern "system" fn subclass(
    hwnd: HWND,
    msg: u32,
    wp: WPARAM,
    lp: LPARAM,
    _: usize,
    data: usize,
) -> LRESULT {
    unsafe {
        let shared = &*(data as *const Arc<Shared>);
        if msg == WM_NCDESTROY {
            RemoveWindowSubclass(hwnd, Some(subclass), ID);
            let result = DefSubclassProc(hwnd, msg, wp, lp);
            drop(Box::from_raw(data as *mut Arc<Shared>));
            return result;
        }
        // GDI paints every client pixel. Suppress the default erase during
        // decoration/size changes so no light background flashes through.
        if msg == WM_ERASEBKGND {
            return 1;
        }
        if !shared.enabled.load(Ordering::Relaxed) {
            return DefSubclassProc(hwnd, msg, wp, lp);
        }
        match msg {
            WM_STYLECHANGING => {
                let styles = &mut *(lp as *mut STYLESTRUCT);
                if wp as i32 == GWL_STYLE {
                    styles.styleNew = widget_style(styles.styleNew);
                } else if wp as i32 == GWL_EXSTYLE {
                    styles.styleNew = widget_extended_style(styles.styleNew);
                }
                return 0;
            }
            WM_NCCALCSIZE | WM_NCPAINT => return 0,
            WM_NCACTIVATE => {
                // Keep winit's focus bookkeeping, but ask DefWindowProc not to
                // repaint the non-client area (documented lParam=-1 behavior).
                return DefSubclassProc(hwnd, msg, wp, -1);
            }
            WM_NCRBUTTONUP if wp == HTCAPTION as usize => return 0,
            WM_NCLBUTTONDOWN if wp == HTCAPTION as usize => {
                // Capture the original button-down coordinates, before the
                // system modal loop and winit's queued mouse-move workaround.
                let cursor = POINT {
                    x: (lp as u32 & 0xffff) as i16 as i32,
                    y: ((lp as u32 >> 16) & 0xffff) as i16 as i32,
                };
                let mut rect = RECT::default();
                GetWindowRect(hwnd, &mut rect);
                *shared.move_origin.lock().unwrap() = Some((cursor, rect));
            }
            WM_NCHITTEST => {
                let x = (lp as u32 & 0xffff) as i16 as i32;
                let y = ((lp as u32 >> 16) & 0xffff) as i16 as i32;
                let mut r = RECT::default();
                GetWindowRect(hwnd, &mut r);
                let d = dpi(hwnd);
                let b = (5. * d).round() as i32;
                let (l, t, rr, bb) = (
                    x < r.left + b,
                    y < r.top + b,
                    x >= r.right - b,
                    y >= r.bottom - b,
                );
                let hit = match (l, t, rr, bb) {
                    (true, true, _, _) => HTTOPLEFT,
                    (_, true, true, _) => HTTOPRIGHT,
                    (true, _, _, true) => HTBOTTOMLEFT,
                    (_, _, true, true) => HTBOTTOMRIGHT,
                    (true, _, _, _) => HTLEFT,
                    (_, true, _, _) => HTTOP,
                    (_, _, true, _) => HTRIGHT,
                    (_, _, _, true) => HTBOTTOM,
                    _ => HTCLIENT,
                };
                if hit != HTCLIENT {
                    return hit as isize;
                }
                let scale = (r.right - r.left) as f64 / 340.;
                // Native title drag excludes both action buttons; the remainder of
                // the card is also draggable except for its footer tooltip.
                if y < r.top + (53. * scale) as i32 && x < r.right - (80. * scale) as i32 {
                    return HTCAPTION as isize;
                }
                return HTCLIENT as isize;
            }
            WM_NCLBUTTONDBLCLK if wp == HTCAPTION as usize => return 0,
            WM_SYSCOMMAND if wp & 0xfff0 == SC_MAXIMIZE as usize => return 0,
            WM_ENTERSIZEMOVE => {
                shared.moving.store(true, Ordering::Relaxed);
                let mut cursor = POINT::default();
                let mut rect = RECT::default();
                GetCursorPos(&mut cursor);
                GetWindowRect(hwnd, &mut rect);
                let mut origin = shared.move_origin.lock().unwrap();
                if origin.is_none() {
                    *origin = Some((cursor, rect));
                }
            }
            WM_MOVING => {
                let rect = &mut *(lp as *mut RECT);
                // WM_MOVING's proposed rectangle is based on the previously
                // snapped position. Use total pointer travel instead, otherwise
                // small outward mouse deltas are continually swallowed.
                if let Some((start, original)) = *shared.move_origin.lock().unwrap() {
                    let mut cursor = POINT::default();
                    GetCursorPos(&mut cursor);
                    *rect = moved_from_origin(original, start, cursor);
                }
                let mut monitor = MONITORINFO {
                    cbSize: std::mem::size_of::<MONITORINFO>() as u32,
                    ..Default::default()
                };
                GetMonitorInfoW(
                    MonitorFromRect(rect, MONITOR_DEFAULTTONEAREST),
                    &mut monitor,
                );
                *rect = snap(*rect, monitor.rcWork, (8. * dpi(hwnd)) as i32);
                return 1;
            }
            WM_SIZING => {
                let mut w = RECT::default();
                let mut c = RECT::default();
                GetWindowRect(hwnd, &mut w);
                GetClientRect(hwnd, &mut c);
                let rect = &mut *(lp as *mut RECT);
                *rect = constrain(
                    *rect,
                    wp,
                    dpi(hwnd),
                    [
                        (w.right - w.left) - (c.right - c.left),
                        (w.bottom - w.top) - (c.bottom - c.top),
                    ],
                );
                return 1;
            }
            WM_EXITSIZEMOVE => {
                shared.moving.store(false, Ordering::Relaxed);
                *shared.move_origin.lock().unwrap() = None;
                let p = geometry(hwnd);
                *shared.placement.lock().unwrap() = p;
                let _ = shared.writer.send(p);
                shared.ctx.request_repaint();
            }
            WM_GETMINMAXINFO => {
                let m = &mut *(lp as *mut MINMAXINFO);
                let d = dpi(hwnd);
                m.ptMinTrackSize = POINT {
                    x: (221. * d) as i32,
                    y: (150.8 * d) as i32,
                };
                m.ptMaxTrackSize = POINT {
                    x: (680. * d) as i32,
                    y: (464. * d) as i32,
                };
                return 0;
            }
            _ => (),
        }
        DefSubclassProc(hwnd, msg, wp, lp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn native_widget_frame_survives_style_rewrites_and_settings() {
        unsafe {
            let class: Vec<u16> = "STATIC\0".encode_utf16().collect();
            let hwnd = CreateWindowExW(
                WS_EX_APPWINDOW | WS_EX_WINDOWEDGE,
                class.as_ptr(),
                class.as_ptr(),
                WS_OVERLAPPEDWINDOW,
                0,
                0,
                340,
                232,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null(),
            );
            assert!(!hwnd.is_null(), "hidden test window creation failed");
            struct Owned(HWND);
            impl Drop for Owned {
                fn drop(&mut self) {
                    unsafe {
                        DestroyWindow(self.0);
                    }
                }
            }
            let _owned = Owned(hwnd);
            let (writer, _rx) = mpsc::channel();
            let shared = Arc::new(Shared {
                enabled: AtomicBool::new(false),
                moving: AtomicBool::new(false),
                ctx: egui::Context::default(),
                placement: Mutex::new(geometry(hwnd)),
                move_origin: Mutex::new(None),
                writer,
            });
            let ptr = Box::into_raw(Box::new(shared.clone()));
            assert_ne!(SetWindowSubclass(hwnd, Some(subclass), ID, ptr as usize), 0);
            let controller = Controller {
                hwnd: hwnd as isize,
                shared,
            };
            for _ in 0..3 {
                controller.prepare_widget_mode(false);
                SetWindowLongPtrW(hwnd, GWL_STYLE, WS_OVERLAPPEDWINDOW as isize);
                SetWindowLongPtrW(
                    hwnd,
                    GWL_EXSTYLE,
                    (WS_EX_APPWINDOW | WS_EX_WINDOWEDGE) as isize,
                );
                assert_ne!(
                    GetWindowLongPtrW(hwnd, GWL_STYLE) as u32 & WS_CAPTION,
                    0,
                    "settings keep native caption"
                );
                controller.prepare_widget_mode(true);
                // Winit may rewrite all flags when visibility, theme or size changes.
                SetWindowLongPtrW(hwnd, GWL_STYLE, WS_OVERLAPPEDWINDOW as isize);
                SetWindowLongPtrW(
                    hwnd,
                    GWL_EXSTYLE,
                    (WS_EX_APPWINDOW | WS_EX_WINDOWEDGE) as isize,
                );
                let style = GetWindowLongPtrW(hwnd, GWL_STYLE) as u32;
                let ex = GetWindowLongPtrW(hwnd, GWL_EXSTYLE) as u32;
                assert_eq!(
                    style & (WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX | WS_MAXIMIZEBOX),
                    0
                );
                assert_ne!(
                    style & WS_THICKFRAME,
                    0,
                    "native edge resizing remains available"
                );
                // Windows may restore WS_EX_WINDOWEDGE while WS_THICKFRAME is set.
                // Suppressed NC painting keeps that resize-related flag invisible.
                assert_eq!(ex & (WS_EX_APPWINDOW | WS_EX_CLIENTEDGE), 0);
                assert_ne!(
                    ex & WS_EX_TOOLWINDOW,
                    0,
                    "widget excluded from taskbar and Alt+Tab"
                );
                assert_eq!(SendMessageW(hwnd, WM_NCPAINT, 1, 0), 0);
                SendMessageW(hwnd, WM_NCACTIVATE, 1, 0);
                SendMessageW(hwnd, WM_NCACTIVATE, 0, 0);
                assert_eq!(GetWindowLongPtrW(hwnd, GWL_STYLE) as u32 & WS_CAPTION, 0);
            }
        }
    }

    #[test]
    fn slow_drag_can_escape_snapped_edge() {
        let original = RECT {
            left: 0,
            top: 0,
            right: 340,
            bottom: 232,
        };
        let work = RECT {
            left: 0,
            top: 0,
            right: 1920,
            bottom: 1080,
        };
        let start = POINT { x: 40, y: 30 };
        let mut last = original;
        for delta in 1..=20 {
            last = snap(
                moved_from_origin(
                    original,
                    start,
                    POINT {
                        x: 40 + delta,
                        y: 30 + delta,
                    },
                ),
                work,
                8,
            );
        }
        assert_eq!((last.left, last.top), (20, 20));
        assert_eq!((last.right - last.left, last.bottom - last.top), (340, 232));
    }
    #[test]
    fn all_resize_edges_keep_ratio_and_anchor() {
        for edge in 1..=8 {
            let old = RECT {
                left: -700,
                top: 80,
                right: -200,
                bottom: 500,
            };
            let r = constrain(old, edge, 1.5, [0, 0]);
            let (w, h) = (r.right - r.left, r.bottom - r.top);
            assert!((w as f64 / h as f64 - RATIO).abs() < 0.01);
            if matches!(edge, 1 | 4 | 7) {
                assert_eq!(r.right, old.right);
            }
            if matches!(edge, 3 | 4 | 5) {
                assert_eq!(r.bottom, old.bottom);
            }
        }
    }
    #[test]
    fn snap_negative_monitor_and_threshold() {
        let work = RECT {
            left: -1920,
            top: -100,
            right: 0,
            bottom: 980,
        };
        let r = RECT {
            left: -1911,
            top: -95,
            right: -1571,
            bottom: 137,
        };
        let s = snap(r, work, 12);
        assert_eq!((s.left, s.top), (-1920, -100));
        assert_eq!(s.right - s.left, 340);
        let far = RECT { left: -1850, ..r };
        assert_eq!(snap(far, work, 12).left, -1850);
    }
    #[test]
    fn resize_clamps() {
        let r = constrain(
            RECT {
                left: 0,
                top: 0,
                right: 99999,
                bottom: 99999,
            },
            8,
            1.,
            [0, 0],
        );
        assert_eq!((r.right, r.bottom), (680, 464));
    }
    #[test]
    fn snap_right_bottom_without_resizing() {
        let work = RECT {
            left: 0,
            top: 0,
            right: 2560,
            bottom: 1400,
        };
        let s = snap(
            RECT {
                left: 2214,
                top: 1163,
                right: 2554,
                bottom: 1395,
            },
            work,
            12,
        );
        assert_eq!((s.right, s.bottom), (2560, 1400));
        assert_eq!((s.right - s.left, s.bottom - s.top), (340, 232));
    }
}
