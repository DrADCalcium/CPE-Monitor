// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (C) 2026 CPE Monitor contributors.
//! Click-through layered child of Explorer's primary horizontal taskbar.
use crate::{
    model::{self, Config, Snapshot, TaskbarSide},
    runtime::Surface,
};
use chrono::{Local, NaiveDate};
use std::ptr::{null, null_mut};
use windows_sys::{
    Win32::{
        Foundation::{HWND, POINT, RECT, SIZE},
        Graphics::Gdi::*,
        UI::{HiDpi::GetDpiForWindow, WindowsAndMessaging::*},
    },
    w,
};

#[derive(PartialEq)]
struct Content {
    remaining: String,
    countdown: String,
    fraction: Option<f64>,
}
impl Content {
    fn from(snapshot: Option<&Snapshot>, today: NaiveDate) -> Self {
        let quota = snapshot.and_then(|s| {
            let total = s.total.filter(|t| t.is_finite() && *t > 0.)?;
            let remaining = (total - s.used).clamp(0., total);
            Some((remaining, remaining / total))
        });
        let remaining = match quota {
            Some((v, _)) => format!("剩余 {}", model::bytes(v)),
            None if snapshot.is_some() => "额度未设置".into(),
            None => "等待首次读取".into(),
        };
        let countdown = snapshot
            .and_then(|s| s.reset_day)
            .filter(|day| (1..=31).contains(day))
            .map(|day| {
                format!(
                    "{} 天后重置",
                    (model::next_reset(today, day) - today).num_days()
                )
            })
            .unwrap_or_else(|| "重置日 —".into());
        Self {
            remaining,
            countdown,
            fraction: quota.map(|(_, r)| r),
        }
    }
}

#[derive(PartialEq)]
struct Layout {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    dpi: u32,
}
fn px(dip: i32, dpi: u32) -> i32 {
    (dip as f64 * dpi as f64 / 96.).round() as i32
}
fn layout(bar: RECT, tray_left: i32, dpi: u32, side: TaskbarSide, offset: i32) -> Option<Layout> {
    let bar_height = bar.bottom - bar.top;
    let width = px(196, dpi).min(tray_left - bar.left);
    let height = px(29, dpi).min(bar_height);
    if width < px(100, dpi) || height < px(18, dpi) || bar_height > bar.right - bar.left {
        return None;
    }
    let margin = px(6, dpi);
    let anchor = match side {
        TaskbarSide::Left => bar.left + margin,
        TaskbarSide::Right => tray_left - width - margin,
    };
    let x = (anchor + px(offset.clamp(-8192, 8192), dpi)).clamp(bar.left, tray_left - width);
    Some(Layout {
        x,
        y: bar.top + (bar_height - height) / 2,
        width,
        height,
        dpi,
    })
}

fn shell_layout(side: TaskbarSide, offset: i32) -> Option<(HWND, Layout)> {
    unsafe {
        let hwnd = FindWindowW(w!("Shell_TrayWnd"), null());
        let mut bar = RECT::default();
        if hwnd.is_null() || GetWindowRect(hwnd, &mut bar) == 0 {
            return None;
        }
        // A child follows the taskbar when Explorer moves or auto-hides it.
        let dpi = GetDpiForWindow(hwnd).max(96);
        let notify = FindWindowExW(hwnd, null_mut(), w!("TrayNotifyWnd"), null());
        let mut icons = RECT::default();
        let tray_left = if IsWindowVisible(notify) != 0
            && GetWindowRect(notify, &mut icons) != 0
            && icons.left > bar.left
            && icons.left < bar.right
        {
            icons.left
        } else {
            // Some Explorer layouts expose no notification child window.
            bar.right - px(180, dpi)
        };
        Some((hwnd, layout(bar, tray_left, dpi, side, offset)?))
    }
}

// A hover highlight, running-app indicator or flyout can change individual
// samples. Require agreement across the widget and keep the last color when
// the taskbar is occluded or the background is near the contrast threshold.
fn sampled_ink(samples: [Option<u32>; 5], previous: Option<u8>) -> Option<u8> {
    let light_threshold = if previous == Some(250) { 154 } else { 144 };
    let dark_threshold = if previous == Some(24) { 134 } else { 144 };
    let light = samples
        .iter()
        .flatten()
        .filter(|&&v| v > light_threshold)
        .count();
    let dark = samples
        .iter()
        .flatten()
        .filter(|&&v| v <= dark_threshold)
        .count();
    if light >= 3 {
        Some(24)
    } else if dark >= 3 {
        Some(250)
    } else {
        previous
    }
}

fn foreground_color(bar: HWND, layout: &Layout, previous: Option<u8>) -> u8 {
    let mut samples = [None; 5];
    unsafe {
        let mut r = RECT::default();
        GetWindowRect(bar, &mut r);
        let dc = GetDC(null_mut());
        if !dc.is_null() {
            for (i, sample) in samples.iter_mut().enumerate() {
                let point = POINT {
                    x: layout.x + layout.width * (2 * i as i32 + 1) / 10,
                    y: r.bottom - px(2, layout.dpi),
                };
                if GetAncestor(WindowFromPoint(point), GA_ROOT) != bar {
                    continue;
                }
                let rgb = GetPixel(dc, point.x, point.y);
                if rgb != CLR_INVALID {
                    *sample = Some(
                        (299 * (rgb & 255) + 587 * ((rgb >> 8) & 255) + 114 * ((rgb >> 16) & 255))
                            / 1000,
                    );
                }
            }
            ReleaseDC(null_mut(), dc);
        }
    }
    if let Some(ink) = sampled_ink(samples, previous) {
        return ink;
    }
    let light = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER)
        .open_subkey(r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize")
        .and_then(|key| key.get_value::<u32, _>("SystemUsesLightTheme"))
        .unwrap_or(0);
    if light == 0 { 250 } else { 24 }
}

#[derive(PartialEq)]
struct Frame {
    layout: Layout,
    content: Content,
    ink: u8,
    font: String,
    weight: u16,
}
impl Frame {
    fn draw_text_mask(&self, mask: &mut Surface, scale: i32, text_height: i32) -> bool {
        let l = &self.layout;
        // Match GDI's antialiasing polarity to the actual foreground. Using
        // white-on-black edge intensities for dark text makes it look heavier.
        let dark_text = self.ink < 128;
        mask.pixels()
            .fill(if dark_text { [255; 4] } else { [0; 4] });
        let dc = mask.dc;
        let margin = px(3, l.dpi) * scale;
        let width = l.width * scale;
        let text_height = text_height * scale;
        let family: Vec<u16> = self.font.encode_utf16().take(31).chain([0]).collect();
        unsafe {
            let font = CreateFontW(
                -px(14, l.dpi * scale as u32).min(text_height),
                0,
                0,
                0,
                self.weight.into(),
                0,
                0,
                0,
                DEFAULT_CHARSET.into(),
                0,
                0,
                ANTIALIASED_QUALITY.into(),
                0,
                family.as_ptr(),
            );
            if font.is_null() {
                return false;
            }
            let old = SelectObject(dc, font);
            SetBkMode(dc, TRANSPARENT as i32);
            SetTextColor(dc, if dark_text { 0 } else { 0xffffff });
            let draw = |text: &str, left: i32, right: i32, align| {
                let wide: Vec<_> = text.encode_utf16().collect();
                let mut rect = RECT {
                    left,
                    top: 0,
                    right,
                    bottom: text_height,
                };
                DrawTextW(
                    dc,
                    wide.as_ptr(),
                    wide.len() as i32,
                    &mut rect,
                    DT_SINGLELINE | DT_VCENTER | DT_END_ELLIPSIS | DT_NOPREFIX | align,
                );
            };
            let countdown: Vec<u16> = self.content.countdown.encode_utf16().collect();
            let mut extent = SIZE::default();
            GetTextExtentPoint32W(dc, countdown.as_ptr(), countdown.len() as i32, &mut extent);
            let split = (width - margin - extent.cx - scale).max(margin);
            draw(&self.content.remaining, margin, split - margin, DT_LEFT);
            draw(&self.content.countdown, split, width - margin, DT_RIGHT);
            SelectObject(dc, old);
            DeleteObject(font);
            GdiFlush(); // The following CPU reads depend on the completed GDI batch.
        }
        true
    }

    fn draw(&self, hwnd: HWND) -> bool {
        let l = &self.layout;
        let margin = px(3, l.dpi);
        let track_y = l.height - px(4, l.dpi);
        let text_height = track_y - px(2, l.dpi);
        let width = l.width as usize;
        let Ok(mut surface) = Surface::new(hwnd, width, l.height as usize) else {
            return false;
        };
        surface.pixels().fill([0; 4]);
        // Allocate scratch only for the text band and only on a changed frame.
        // At very high DPI native pixels already provide ample detail.
        let mut scale = text_mask_scale(width, text_height as usize);
        let mut mask = loop {
            match Surface::new(hwnd, width * scale, text_height as usize * scale) {
                Ok(mask) => break mask,
                Err(_) if scale > 1 => scale = 1,
                Err(_) => return false,
            }
        };
        if !self.draw_text_mask(&mut mask, scale as i32, text_height) {
            return false;
        }
        resolve_text(mask.pixels(), surface.pixels(), width, scale, self.ink);
        drop(mask);
        let track_width = l.width - 2 * margin;
        let filled = (track_width as f64 * self.content.fraction.unwrap_or(0.)).round() as i32;
        for row in surface
            .pixels()
            .chunks_exact_mut(width)
            .skip(track_y as usize)
            .take(px(2, l.dpi) as usize)
        {
            for (x, pixel) in row[margin as usize..(margin + track_width) as usize]
                .iter_mut()
                .enumerate()
            {
                *pixel = ink_pixel(self.ink, if (x as i32) < filled { 255 } else { 80 });
            }
        }
        unsafe {
            UpdateLayeredWindow(
                hwnd,
                null_mut(),
                null(),
                &SIZE {
                    cx: l.width,
                    cy: l.height,
                },
                surface.dc,
                &POINT::default(),
                0,
                &BLENDFUNCTION {
                    BlendOp: AC_SRC_OVER as u8,
                    BlendFlags: 0,
                    SourceConstantAlpha: 255,
                    AlphaFormat: AC_SRC_ALPHA as u8,
                },
                ULW_ALPHA,
            ) != 0
        }
    }
}

fn text_mask_scale(width: usize, height: usize) -> usize {
    const MAX_SCRATCH_BYTES: usize = 1024 * 1024;
    (2..=3)
        .rev()
        .find(|s| width * height * 4 * s * s <= MAX_SCRATCH_BYTES)
        .unwrap_or(1)
}

fn ink_pixel(ink: u8, alpha: u8) -> [u8; 4] {
    let color = ((u16::from(ink) * u16::from(alpha) + 127) / 255) as u8;
    [color, color, color, alpha]
}

// Resolve only glyph coverage. Progress bars stay aligned to physical pixels.
fn resolve_text(mask: &[[u8; 4]], output: &mut [[u8; 4]], width: usize, scale: usize, ink: u8) {
    let stride = width * scale;
    let count = (scale * scale) as u32;
    for (band, row) in mask
        .chunks_exact(stride * scale)
        .zip(output.chunks_exact_mut(width))
    {
        for (x, pixel) in row.iter_mut().enumerate() {
            let start = x * scale;
            let sum: u32 = band
                .chunks_exact(stride)
                .flat_map(|scan| &scan[start..start + scale])
                .map(|p| u32::from(p[0]))
                .sum();
            let coverage = if ink < 128 { 255 * count - sum } else { sum };
            let alpha = ((coverage + count / 2) / count) as u8;
            *pixel = ink_pixel(ink, alpha);
        }
    }
}

#[derive(Default)]
pub struct Taskbar {
    hwnd: HWND,
    last: Option<Frame>,
}
impl Taskbar {
    pub fn sync(&mut self, config: &Config, snapshot: Option<&Snapshot>) {
        let t = &config.taskbar;
        let Some((bar, layout)) = shell_layout(t.side, t.offset) else {
            self.close();
            return;
        };
        let frame = Frame {
            ink: foreground_color(bar, &layout, self.last.as_ref().map(|f| f.ink)),
            layout,
            content: Content::from(snapshot, Local::now().date_naive()),
            font: if t.font.is_empty() {
                config.font.clone()
            } else {
                t.font.clone()
            },
            weight: config.font_weight,
        };
        self.update_window(bar, frame);
    }
    fn update_window(&mut self, bar: HWND, frame: Frame) {
        unsafe {
            // Explorer may replace the taskbar and destroy its child windows.
            // A handle can subsequently be reused.
            if !self.hwnd.is_null() && (!self.has_window() || GetParent(self.hwnd) != bar) {
                self.close();
            }
            if self.hwnd.is_null() {
                self.hwnd = CreateWindowExW(
                    WS_EX_LAYERED | WS_EX_NOACTIVATE | WS_EX_NOPARENTNOTIFY | WS_EX_TRANSPARENT,
                    w!("STATIC"),
                    w!("CPE 流量任务栏"),
                    WS_CHILD,
                    0,
                    0,
                    0,
                    0,
                    bar,
                    null_mut(),
                    null_mut(),
                    null(),
                );
                if self.hwnd.is_null() {
                    return;
                }
                if SetPropW(self.hwnd, w!("CPEMonitor.TaskbarOverlay"), self.hwnd) == 0 {
                    DestroyWindow(self.hwnd);
                    self.hwnd = null_mut();
                    return;
                }
            }
            if self.last.as_ref().map(|f| &f.layout) != Some(&frame.layout) {
                let l = &frame.layout;
                let mut position = POINT { x: l.x, y: l.y };
                if ScreenToClient(bar, &mut position) == 0
                    || SetWindowPos(
                        self.hwnd,
                        HWND_TOP,
                        position.x,
                        position.y,
                        l.width,
                        l.height,
                        SWP_NOACTIVATE,
                    ) == 0
                {
                    self.close();
                    return;
                }
            }
            if self.last.as_ref() != Some(&frame) {
                if !frame.draw(self.hwnd) {
                    self.close();
                    return;
                }
                self.last = Some(frame);
            }
            // Child windows move and stack together with their taskbar parent.
            if GetWindowLongW(self.hwnd, GWL_STYLE) as u32 & WS_VISIBLE == 0 {
                SetWindowPos(
                    self.hwnd,
                    null_mut(),
                    0,
                    0,
                    0,
                    0,
                    SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE | SWP_SHOWWINDOW,
                );
            }
        }
    }
    fn has_window(&self) -> bool {
        !self.hwnd.is_null()
            && unsafe { GetPropW(self.hwnd, w!("CPEMonitor.TaskbarOverlay")) == self.hwnd }
    }
    pub fn close(&mut self) {
        if self.has_window() {
            unsafe {
                RemovePropW(self.hwnd, w!("CPEMonitor.TaskbarOverlay"));
                DestroyWindow(self.hwnd);
            }
        }
        self.hwnd = null_mut();
        self.last = None;
    }
}
impl Drop for Taskbar {
    fn drop(&mut self) {
        self.close();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mask_preserves_background_edges_and_solid_strokes_in_both_themes() {
        for scale in 1..=3 {
            for ink in [24, 250] {
                let width = 3;
                let mut mask = vec![[0; 4]; width * scale * scale];
                for row in mask.chunks_exact_mut(width * scale) {
                    for (x, sample) in row.iter_mut().enumerate() {
                        let coverage = match x / scale {
                            0 => 0,
                            1 if x % scale == 0 => 255,
                            1 => 0,
                            _ => 255,
                        };
                        sample[0] = if ink < 128 { 255 - coverage } else { coverage };
                    }
                }
                let mut pixels = [[99; 4]; 6];
                resolve_text(&mask, &mut pixels, width, scale, ink);
                assert_eq!(pixels[0], [0; 4]);
                assert_eq!(
                    pixels[1],
                    ink_pixel(ink, (255_f32 / scale as f32).round() as u8)
                );
                assert_eq!(pixels[2], [ink, ink, ink, 255]);
                assert_eq!(pixels[3], [99; 4]); // outside the text band
            }
        }
    }

    #[test]
    fn mask_budget_adapts_to_display_scale() {
        for dpi in [96, 120, 144, 192, 240, 288, 384, 480] {
            let w = px(196, dpi) as usize;
            let h = (px(29, dpi) - px(4, dpi) - px(2, dpi)) as usize;
            let scale = text_mask_scale(w, h);
            assert!(w * h * 4 * scale * scale <= 1024 * 1024);
            if dpi <= 192 {
                assert_eq!(scale, 3);
            }
        }
    }

    #[test]
    fn overlay_stays_with_its_taskbar_and_recovers_when_parent_is_replaced() {
        // Two hidden native parents stand in for successive Explorer taskbars.
        struct Parent(HWND);
        impl Parent {
            fn new() -> Self {
                let hwnd = unsafe {
                    CreateWindowExW(
                        0,
                        w!("STATIC"),
                        null(),
                        WS_POPUP,
                        100,
                        100,
                        500,
                        52,
                        null_mut(),
                        null_mut(),
                        null_mut(),
                        null(),
                    )
                };
                assert!(!hwnd.is_null());
                Self(hwnd)
            }
        }
        impl Drop for Parent {
            fn drop(&mut self) {
                unsafe {
                    DestroyWindow(self.0);
                }
            }
        }
        let frame = || Frame {
            layout: Layout {
                x: 120,
                y: 110,
                width: 196,
                height: 29,
                dpi: 96,
            },
            content: Content::from(None, Local::now().date_naive()),
            ink: 24,
            font: "Segoe UI".into(),
            weight: 400,
        };
        let first = Parent::new();
        let replacement = Parent::new();
        let mut overlay = Taskbar::default();
        overlay.update_window(first.0, frame());
        let child = overlay.hwnd;
        assert!(!child.is_null());
        assert_eq!(unsafe { GetParent(child) }, first.0);
        let mut r = RECT::default();
        assert_ne!(unsafe { GetWindowRect(child, &mut r) }, 0);
        assert_eq!((r.left, r.top), (120, 110));
        overlay.update_window(first.0, frame());
        assert_eq!(overlay.hwnd, child);
        drop(first);
        assert!(!overlay.has_window());
        overlay.update_window(replacement.0, frame());
        assert!(overlay.has_window());
        assert_eq!(unsafe { GetParent(overlay.hwnd) }, replacement.0);
        overlay.close();
        assert_ne!(unsafe { IsWindow(replacement.0) }, 0);
    }

    #[test]
    fn transient_samples_do_not_flip_taskbar_text() {
        assert_eq!(
            sampled_ink([Some(220), Some(30), Some(225), None, Some(220)], Some(24)),
            Some(24)
        );
        assert_eq!(sampled_ink([None; 5], Some(24)), Some(24));
        assert_eq!(sampled_ink([Some(142); 5], Some(24)), Some(24));
        assert_eq!(sampled_ink([Some(148); 5], Some(250)), Some(250));
        assert_eq!(sampled_ink([Some(40); 5], Some(24)), Some(250));
        assert_eq!(sampled_ink([Some(220); 5], Some(250)), Some(24));
    }

    #[test]
    fn placement_respects_dpi_offset_and_notification_area() {
        let bar = RECT {
            left: -2000,
            top: 0,
            right: 0,
            bottom: 72,
        };
        let left = layout(bar, -250, 144, TaskbarSide::Left, 100).unwrap();
        assert_eq!(left.x, -2000 + 9 + 150);
        let right = layout(bar, -250, 144, TaskbarSide::Right, -100).unwrap();
        assert_eq!(right.x + right.width, -250 - 9 - 150);
        let clamped = layout(bar, -250, 144, TaskbarSide::Right, 8000).unwrap();
        assert_eq!(clamped.x + clamped.width, -250);
    }

    #[test]
    fn over_quota_and_short_month_countdown() {
        let today = NaiveDate::from_ymd_opt(2026, 2, 26).unwrap();
        let mut s = Snapshot {
            used: 101.,
            total: Some(100.),
            lifetime: 101.,
            reset_day: Some(31),
            updated: Local::now(),
        };
        let content = Content::from(Some(&s), today);
        assert_eq!(content.fraction, Some(0.));
        assert_eq!(content.countdown, "2 天后重置");
        s.total = None;
        assert_eq!(Content::from(Some(&s), today).remaining, "额度未设置");
        assert_eq!(Content::from(None, today).remaining, "等待首次读取");
    }
}
