use crate::{
    App,
    model::{self, Config},
    worker,
};
use egui::{
    self, Align2, Color32, FontFamily, FontId, Pos2, Rect, RichText, Sense, Stroke, StrokeKind,
    Vec2,
};
use std::sync::OnceLock;

pub const WIDGET_SIZE: Vec2 = Vec2::new(340., 232.);
pub const SETTINGS_SIZE: Vec2 = Vec2::new(420., 660.);
const ACCENT: Color32 = Color32::from_rgb(0, 95, 184);
const ACCENT_DARK: Color32 = Color32::from_rgb(96, 205, 255);
const LIGHT_USAGE: Color32 = Color32::from_rgb(0x95, 0xEC, 0x69);
const SEMIBOLD: &str = "ui-semibold";

struct FontCatalog {
    db: fontdb::Database,
    names: Vec<String>,
}
fn catalog() -> &'static FontCatalog {
    static CATALOG: OnceLock<FontCatalog> = OnceLock::new();
    CATALOG.get_or_init(|| {
        let mut db = fontdb::Database::new();
        db.load_system_fonts();
        let mut names: Vec<String> = db
            .faces()
            .filter_map(|f| f.families.first().map(|n| n.0.clone()))
            .filter(|n| !n.starts_with('@'))
            .collect();
        names.sort_by_key(|n| n.to_lowercase());
        names.dedup();
        FontCatalog { db, names }
    })
}
fn face(name: &str, bold: bool) -> Option<fontdb::ID> {
    // Some installed families use nonstandard OS/2 weights. Prefer the named
    // Regular/Semibold faces before CSS numeric weight matching.
    for suffix in if bold {
        vec!["semibold", "demibold"]
    } else {
        vec!["regular"]
    } {
        if let Some(f) = catalog().db.faces().find(|f| {
            f.families.iter().any(|(n, _)| n.eq_ignore_ascii_case(name))
                && f.style == fontdb::Style::Normal
                && f.post_script_name.to_lowercase().ends_with(suffix)
        }) {
            return Some(f.id);
        }
    }
    catalog().db.query(&fontdb::Query {
        families: &[fontdb::Family::Name(name)],
        weight: if bold {
            fontdb::Weight::SEMIBOLD
        } else {
            fontdb::Weight::NORMAL
        },
        ..Default::default()
    })
}
fn face_weight(name: &str, requested: u16) -> Option<fontdb::ID> {
    let db = &catalog().db;
    db.faces()
        .filter(|f| {
            f.style == fontdb::Style::Normal
                && f.families.iter().any(|(n, _)| n.eq_ignore_ascii_case(name))
        })
        .min_by_key(|f| {
            let n = f.post_script_name.to_lowercase();
            let weight = if n.ends_with("thin") {
                100
            } else if n.ends_with("extralight") {
                200
            } else if n.ends_with("light") {
                300
            } else if n.ends_with("regular") || n.ends_with("normal") {
                400
            } else if n.ends_with("medium") {
                500
            } else if n.ends_with("semibold") || n.ends_with("demibold") {
                600
            } else if n.ends_with("extrabold") {
                800
            } else if n.ends_with("bold") {
                700
            } else if n.ends_with("black") || n.ends_with("heavy") {
                900
            } else {
                f.weight.0
            };
            (
                (weight as i32 - requested as i32).abs(),
                if n.ends_with("semibold") { 0 } else { 1 },
            )
        })
        .map(|f| f.id)
}
pub fn font_report(name: &str) -> serde_json::Value {
    let info = |bold| {
        face(name,bold).and_then(|id| catalog().db.face(id)).map(|f| serde_json::json!({"family":f.families.first().map(|f|&f.0),"weight":f.weight.0,"face_index":f.index,"postscript":f.post_script_name}))
    };
    serde_json::json!({"requested":name,"regular":info(false),"semibold":info(true),"installed_families":catalog().names.len()})
}
fn semibold(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(SEMIBOLD.into()))
}
pub fn apply_appearance(ctx: &egui::Context, c: &Config) {
    ctx.set_theme(match c.theme.as_str() {
        "light" => egui::ThemePreference::Light,
        "dark" => egui::ThemePreference::Dark,
        _ => egui::ThemePreference::System,
    });
    ctx.set_zoom_factor(1.);
    static CACHE: std::sync::Mutex<Option<((String, u16, u16), egui::FontDefinitions)>> =
        std::sync::Mutex::new(None);
    let key = (c.font.clone(), c.font_weight, c.heading_weight);
    let fonts = {
        let mut cache = CACHE.lock().unwrap();
        if let Some((_, fonts)) = cache.as_ref().filter(|(k, _)| k == &key) {
            fonts.clone()
        } else {
            let mut fonts = egui::FontDefinitions::default();
            let mut regular = Vec::new();
            let mut bold = Vec::new();
            // Keep the user's family first, with a CJK fallback and an actual semibold face.
            let has_cjk = face(&c.font, false)
                .and_then(|id| {
                    catalog().db.with_face_data(id, |data, index| {
                        ttf_parser::Face::parse(data, index)
                            .ok()
                            .is_some_and(|f| f.glyph_index('中').is_some())
                    })
                })
                .unwrap_or(false);
            let candidates = if has_cjk {
                vec![c.font.as_str()]
            } else {
                vec![c.font.as_str(), "Microsoft YaHei UI"]
            };
            for (slot, name) in candidates.iter().enumerate() {
                for (heavy, list) in [(false, &mut regular), (true, &mut bold)] {
                    let requested = if heavy {
                        c.heading_weight
                    } else {
                        c.font_weight
                    };
                    if let Some(id) = face_weight(name, requested) {
                        let key = format!("face-{slot}-{heavy}");
                        if let Some(data) = catalog().db.with_face_data(id, |bytes, index| {
                            let mut data = egui::FontData::from_owned(bytes.to_vec());
                            data.index = index;
                            if let Some(axis) =
                                data.variation_axes().iter().find(|a| a.tag == *b"wght")
                            {
                                data.tweak.coords.push(
                                    b"wght",
                                    (requested as f32).clamp(axis.range.min, axis.range.max),
                                );
                            }
                            data
                        }) {
                            fonts.font_data.insert(key.clone(), data.into());
                            list.push(key);
                        }
                    }
                }
            }
            let defaults = fonts
                .families
                .get(&FontFamily::Proportional)
                .cloned()
                .unwrap_or_default();
            regular.extend(defaults.clone());
            bold.extend(regular.clone());
            fonts.families.insert(FontFamily::Proportional, regular);
            fonts
                .families
                .insert(FontFamily::Name(SEMIBOLD.into()), bold);
            *cache = Some((key, fonts.clone()));
            fonts
        }
    };
    ctx.set_fonts(fonts);
    ctx.all_styles_mut(|s| {
        s.text_styles
            .insert(egui::TextStyle::Body, FontId::proportional(14.));
        s.text_styles
            .insert(egui::TextStyle::Button, FontId::proportional(14.));
        s.text_styles
            .insert(egui::TextStyle::Small, FontId::proportional(12.));
        s.text_styles
            .insert(egui::TextStyle::Heading, semibold(24.));
        s.text_styles
            .insert(egui::TextStyle::Monospace, FontId::proportional(14.));
        s.spacing.item_spacing = Vec2::new(8., 8.);
        s.spacing.button_padding = Vec2::new(12., 6.);
        s.spacing.interact_size = Vec2::new(32., 32.);
        s.spacing.combo_height = 240.;
        let p = Palette::new(s.visuals.dark_mode);
        s.visuals.override_text_color = Some(p.text);
        s.visuals.panel_fill = p.surface;
        s.visuals.window_fill = p.surface;
        s.visuals.extreme_bg_color = p.field;
        s.visuals.faint_bg_color = p.hover;
        s.visuals.selection.bg_fill = p.accent.gamma_multiply(0.22);
        s.visuals.selection.stroke = Stroke::new(1., p.accent);
        s.visuals.handle_shape = egui::style::HandleShape::Circle;
        s.visuals.window_corner_radius = 8.into();
        s.visuals.widgets.noninteractive.bg_stroke = Stroke::new(1., p.border);
        s.visuals.widgets.noninteractive.fg_stroke = Stroke::new(1., p.secondary);
        for (w, bg, border) in [
            (&mut s.visuals.widgets.inactive, p.field, p.border),
            (&mut s.visuals.widgets.hovered, p.hover, p.border),
            (&mut s.visuals.widgets.active, p.pressed, p.accent),
            (&mut s.visuals.widgets.open, p.hover, p.accent),
        ] {
            w.bg_fill = bg;
            w.weak_bg_fill = bg;
            w.bg_stroke = Stroke::new(1., border);
            w.fg_stroke = Stroke::new(1., p.text);
            w.corner_radius = 4.into();
            w.expansion = 0.;
        }
    });
    ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(if c.topmost {
        egui::WindowLevel::AlwaysOnTop
    } else {
        egui::WindowLevel::Normal
    }));
}
#[derive(Clone, Copy)]
struct Palette {
    surface: Color32,
    field: Color32,
    hover: Color32,
    pressed: Color32,
    text: Color32,
    secondary: Color32,
    border: Color32,
    accent: Color32,
    green: Color32,
    track: Color32,
}
impl Palette {
    fn new(dark: bool) -> Self {
        if dark {
            Self {
                surface: Color32::from_rgb(32, 32, 32),
                field: Color32::from_rgb(45, 45, 45),
                hover: Color32::from_rgb(54, 54, 54),
                pressed: Color32::from_rgb(40, 40, 40),
                text: Color32::from_gray(245),
                secondary: Color32::from_gray(185),
                border: Color32::from_gray(67),
                accent: ACCENT_DARK,
                green: Color32::from_rgb(88, 215, 168),
                track: Color32::from_gray(55),
            }
        } else {
            Self {
                surface: Color32::from_rgb(249, 249, 249),
                field: Color32::WHITE,
                hover: Color32::from_rgb(242, 242, 242),
                pressed: Color32::from_rgb(234, 234, 234),
                text: Color32::from_rgb(31, 31, 31),
                secondary: Color32::from_rgb(97, 97, 97),
                border: Color32::from_rgb(218, 218, 218),
                accent: ACCENT,
                green: Color32::from_rgb(36, 177, 127),
                track: Color32::from_rgb(231, 231, 231),
            }
        }
    }
}
fn text(
    p: &egui::Painter,
    point: Pos2,
    anchor: Align2,
    value: impl ToString,
    size: f32,
    bold: bool,
    color: Color32,
) {
    p.text(
        point,
        anchor,
        value.to_string(),
        if bold {
            semibold(size)
        } else {
            FontId::proportional(size)
        },
        color,
    );
}
fn icon_button(ui: &mut egui::Ui, rect: Rect, gear: bool, p: Palette) -> egui::Response {
    let response = ui.interact(
        rect,
        ui.id().with(if gear { "settings" } else { "hide" }),
        Sense::click(),
    );
    let paint = ui.painter();
    if response.hovered() || response.is_pointer_button_down_on() {
        paint.rect_filled(
            rect,
            4,
            if response.is_pointer_button_down_on() {
                p.pressed
            } else {
                p.hover
            },
        );
        paint.rect_stroke(rect, 4, Stroke::new(1., p.border), StrokeKind::Inside);
    }
    let center = rect.center();
    let stroke = Stroke::new(1.25, p.text);
    if gear {
        let mut points = Vec::new();
        for i in 0..32 {
            let angle = i as f32 * std::f32::consts::TAU / 32.;
            let radius = if i % 4 == 1 || i % 4 == 2 { 7. } else { 5.5 };
            points.push(center + Vec2::angled(angle) * radius);
        }
        paint.add(egui::Shape::closed_line(points, stroke));
        paint.circle_stroke(center, 2.4, stroke);
    } else {
        paint.line_segment(
            [center + Vec2::new(-5., 0.), center + Vec2::new(5., 0.)],
            stroke,
        );
    }
    response.on_hover_text(if gear { "设置" } else { "隐藏到托盘" })
}
fn switch(ui: &mut egui::Ui, label: &str, value: &mut bool, p: Palette) {
    let (row, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 32.), Sense::hover());
    let rect = Rect::from_center_size(
        Pos2::new(row.right() - 20., row.center().y),
        Vec2::new(40., 20.),
    );
    let response = ui.interact(row, ui.id().with(label), Sense::click());
    if response.clicked() {
        *value = !*value;
    }
    let painter = ui.painter();
    text(
        painter,
        Pos2::new(row.left(), row.center().y),
        Align2::LEFT_CENTER,
        label,
        14.,
        false,
        p.text,
    );
    let fill = if *value {
        p.accent
    } else if response.hovered() {
        p.hover
    } else {
        p.field
    };
    painter.rect(
        rect,
        10,
        fill,
        Stroke::new(1., if *value { p.accent } else { p.secondary }),
        StrokeKind::Inside,
    );
    let thumb = Pos2::new(
        if *value {
            rect.right() - 10.
        } else {
            rect.left() + 10.
        },
        rect.center().y,
    );
    painter.circle_filled(
        thumb,
        6.,
        if *value {
            if ui.visuals().dark_mode {
                Color32::from_gray(25)
            } else {
                Color32::WHITE
            }
        } else {
            p.secondary
        },
    );
    if response.has_focus() {
        painter.rect_stroke(
            row.expand(2.),
            4,
            Stroke::new(1., p.accent),
            StrokeKind::Inside,
        );
    }
}
fn label(ui: &mut egui::Ui, title: &str, p: Palette) {
    ui.add_space(4.);
    ui.label(RichText::new(title).size(12.).color(p.secondary));
}
fn radio(ui: &mut egui::Ui, value: &mut String, key: &str, title: &str, p: Palette) {
    let galley = ui
        .painter()
        .layout_no_wrap(title.into(), FontId::proportional(14.), p.text);
    let (r, response) =
        ui.allocate_exact_size(Vec2::new(galley.size().x + 28., 32.), Sense::click());
    if response.clicked() {
        *value = key.into();
    }
    let selected = value == key;
    let center = Pos2::new(r.left() + 9., r.center().y);
    ui.painter().circle(
        center,
        8.,
        p.field,
        Stroke::new(
            if selected { 2. } else { 1. },
            if selected {
                p.accent
            } else if response.hovered() {
                p.secondary
            } else {
                p.border
            },
        ),
    );
    if selected {
        ui.painter().circle_filled(center, 4., p.accent);
    }
    ui.painter().galley(
        Pos2::new(r.left() + 24., r.center().y - galley.size().y / 2.),
        galley,
        p.text,
    );
    if response.has_focus() {
        ui.painter()
            .rect_stroke(r, 4, Stroke::new(1., p.accent), StrokeKind::Inside);
    }
}
fn edit(ui: &mut egui::Ui, value: &mut String, secret: bool, hint: &str) {
    let mut output = egui::TextEdit::singleline(value)
        .password(secret)
        .hint_text(hint)
        .desired_width(ui.available_width())
        .min_size(egui::vec2(0., 32.))
        .margin(egui::Margin::symmetric(10, 7))
        .show(ui);
    if secret {
        output.state.clear_undoer();
        output.state.store(ui.ctx(), output.response.id);
    }
}

impl App {
    pub(crate) fn widget(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let p = Palette::new(ui.visuals().dark_mode);
        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(p.surface)
                    .corner_radius(8)
                    .stroke(Stroke::new(1., p.border))
                    .inner_margin(0),
            )
            .show(ui, |ui| {
                let rect = ui.max_rect();
                let base = rect.min;
                let x = base.x + 20.;
                let right = rect.right() - 20.;
                let pos = |a: f32, b: f32| base + Vec2::new(a, b);
                let drag = ui.interact(
                    Rect::from_min_max(pos(12., 8.), pos(rect.width() - 80., 53.)),
                    ui.id().with("title-drag"),
                    Sense::drag(),
                );
                if self.native.is_none() && drag.drag_started() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                }
                drag.context_menu(|ui| {
                    if ui.button("立即刷新").clicked() {
                        let _ = self.tx.send(worker::Command::Refresh);
                        ui.close();
                    }
                    if ui.button("设置").clicked() {
                        self.open_settings(ctx);
                        ui.close();
                    }
                    if ui.button("退出").clicked() {
                        self.quitting = true;
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                let painter = ui.painter().clone();
                text(
                    &painter,
                    Pos2::new(x, base.y + 16.),
                    Align2::LEFT_TOP,
                    "CPE 流量监控",
                    16.,
                    true,
                    p.text,
                );
                let host = self
                    .config
                    .router
                    .trim_start_matches("http://")
                    .trim_start_matches("https://");
                text(
                    &painter,
                    Pos2::new(x, base.y + 38.),
                    Align2::LEFT_TOP,
                    format!(
                        "{host} · {}",
                        if self.online {
                            "已连接"
                        } else {
                            "未更新"
                        }
                    ),
                    11.,
                    false,
                    p.secondary,
                );
                text(
                    &painter,
                    Pos2::new(x, base.y + 59.),
                    Align2::LEFT_TOP,
                    "剩余流量",
                    12.,
                    false,
                    if ui.visuals().dark_mode {
                        p.secondary
                    } else {
                        LIGHT_USAGE
                    },
                );
                let remaining = self
                    .snapshot
                    .as_ref()
                    .and_then(|s| s.total.map(|t| (t - s.used).max(0.)));
                let ratio = self
                    .snapshot
                    .as_ref()
                    .and_then(|s| s.total.map(|t| ((t - s.used) / t).clamp(0., 1.) as f32))
                    .unwrap_or(0.);
                let color = if self.snapshot.is_some() && ratio < 0.1 {
                    Color32::from_rgb(208, 128, 28)
                } else {
                    p.green
                };
                text(
                    &painter,
                    Pos2::new(x, base.y + 76.),
                    Align2::LEFT_TOP,
                    remaining.map(model::bytes).unwrap_or_else(|| "— GB".into()),
                    32.,
                    true,
                    color,
                );
                let track = Rect::from_min_max(
                    Pos2::new(x, base.y + 119.),
                    Pos2::new(right, base.y + 127.),
                );
                painter.rect_filled(track, 4, p.track);
                if ratio > 0. {
                    painter.rect_filled(
                        Rect::from_min_size(
                            track.min,
                            Vec2::new(track.width() * ratio, track.height()),
                        ),
                        4,
                        if ui.visuals().dark_mode {
                            color
                        } else {
                            LIGHT_USAGE
                        },
                    );
                }
                text(
                    &painter,
                    Pos2::new(x, base.y + 135.),
                    Align2::LEFT_TOP,
                    self.snapshot
                        .as_ref()
                        .map(|s| format!("已用 {}", model::bytes(s.used)))
                        .unwrap_or_else(|| "已用 —".into()),
                    12.,
                    false,
                    p.text,
                );
                text(
                    &painter,
                    Pos2::new(right, base.y + 135.),
                    Align2::RIGHT_TOP,
                    self.snapshot
                        .as_ref()
                        .and_then(|s| s.total.map(|t| format!("共 {}", model::bytes(t))))
                        .unwrap_or_else(|| "额度未设置".into()),
                    12.,
                    false,
                    p.secondary,
                );
                painter.line_segment(
                    [Pos2::new(x, base.y + 158.), Pos2::new(right, base.y + 158.)],
                    Stroke::new(1., p.border),
                );
                text(
                    &painter,
                    Pos2::new(x, base.y + 167.),
                    Align2::LEFT_TOP,
                    "下次清零",
                    11.,
                    false,
                    p.secondary,
                );
                let reset = self
                    .snapshot
                    .as_ref()
                    .and_then(|s| s.reset_day)
                    .map(|d| {
                        let today = chrono::Local::now().date_naive();
                        let next = model::next_reset(today, d);
                        format!("{next}（{} 天后）", (next - today).num_days())
                    })
                    .unwrap_or_else(|| "未设置".into());
                text(
                    &painter,
                    Pos2::new(x, base.y + 183.),
                    Align2::LEFT_TOP,
                    reset,
                    14.,
                    true,
                    p.text,
                );
                let stamp = self
                    .snapshot
                    .as_ref()
                    .map(|s| {
                        format!(
                            "{} {}",
                            if self.online { "更新" } else { "缓存" },
                            s.updated.format("%H:%M:%S")
                        )
                    })
                    .unwrap_or_else(|| "等待首次读取".into());
                let total = self
                    .snapshot
                    .as_ref()
                    .map(|s| format!("累计 {}", model::bytes(s.lifetime)))
                    .unwrap_or_default();
                text(
                    &painter,
                    Pos2::new(x, base.y + 211.),
                    Align2::LEFT_TOP,
                    stamp,
                    10.,
                    false,
                    p.secondary,
                );
                text(
                    &painter,
                    Pos2::new(right, base.y + 211.),
                    Align2::RIGHT_TOP,
                    total,
                    10.,
                    false,
                    p.secondary,
                );
                ui.interact(
                    Rect::from_min_max(pos(20., 207.), pos(rect.width() - 20., 230.)),
                    ui.id().with("data-status"),
                    Sense::hover(),
                )
                .on_hover_text(format!(
                    "{}\n最近成功读取：{}\n来自 CPE 本地统计。",
                    self.status,
                    self.snapshot
                        .as_ref()
                        .map(|s| s.updated.format("%Y-%m-%d %H:%M:%S").to_string())
                        .unwrap_or_else(|| "无".into())
                ));
                if ctx.input(|i| i.pointer.hover_pos().is_some())
                    || std::env::args().any(|s| s == "--capture-hover")
                {
                    let top = base.y + 12.;
                    let hide =
                        Rect::from_min_size(Pos2::new(rect.right() - 40., top), Vec2::splat(28.));
                    let settings = hide.translate(Vec2::new(-32., 0.));
                    if icon_button(ui, settings, true, p).clicked() {
                        self.open_settings(ctx);
                    }
                    if icon_button(ui, hide, false, p).clicked() {
                        if self._tray.is_some() {
                            self.set_visible(ctx, false);
                        } else {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
                        }
                    }
                }
            });
    }
    pub(crate) fn settings_ui(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let p = Palette::new(ui.visuals().dark_mode);
        if ui.max_rect().height() < 480. {
            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(SETTINGS_SIZE));
            return;
        }
        let before = (
            self.draft.theme.clone(),
            self.draft.font.clone(),
            self.draft.font_weight,
            self.draft.heading_weight,
        );
        let mut save = false;
        let mut cancel = false;
        let header_origin = ui.max_rect().min;
        egui::Panel::top("settings-header")
            .exact_size(64.)
            .resizable(false)
            .show_separator_line(false)
            .frame(egui::Frame::new().fill(p.surface))
            .show(ui, |ui| {
                text(
                    ui.painter(),
                    header_origin + Vec2::new(24., 16.),
                    Align2::LEFT_TOP,
                    "设置",
                    24.,
                    true,
                    p.text,
                );
            });
        egui::Panel::bottom("settings-footer")
            .resizable(false)
            .exact_size(if self.error.is_empty() { 72. } else { 100. })
            .frame(
                egui::Frame::new()
                    .fill(p.surface)
                    .inner_margin(egui::Margin::symmetric(24, 16)),
            )
            .show(ui, |ui| {
                let r = ui.max_rect();
                ui.painter().line_segment(
                    [
                        Pos2::new(r.left() - 24., r.top() - 16.),
                        Pos2::new(r.right() + 24., r.top() - 16.),
                    ],
                    Stroke::new(1., p.border),
                );
                if !self.error.is_empty() {
                    ui.add(
                        egui::Label::new(
                            RichText::new(&self.error)
                                .size(12.)
                                .color(Color32::from_rgb(196, 43, 28)),
                        )
                        .truncate(),
                    )
                    .on_hover_text(&self.error);
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    cancel = ui
                        .add_sized([88., 32.], egui::Button::new("取消"))
                        .clicked();
                    save = ui
                        .add_sized(
                            [88., 32.],
                            egui::Button::new(RichText::new("保存").color(
                                if ui.visuals().dark_mode {
                                    Color32::from_gray(20)
                                } else {
                                    Color32::WHITE
                                },
                            ))
                            .fill(p.accent)
                            .stroke(Stroke::NONE),
                        )
                        .clicked();
                });
            });
        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(p.surface)
                    .inner_margin(egui::Margin::symmetric(24, 0)),
            )
            .show(ui, |ui| {
                let mut scroll = egui::ScrollArea::vertical()
                    .id_salt("settings-scroll")
                    .auto_shrink([false, false]);
                if self.capture.is_some() && std::env::args().any(|s| s == "--capture-weights") {
                    scroll = scroll.vertical_scroll_offset(390.);
                }
                scroll.show(ui, |ui| {
                    label(ui, "路由器地址", p);
                    edit(ui, &mut self.draft.router, false, "http://路由器内网地址");
                    label(ui, "管理后台密码", p);
                    edit(ui, &mut self.password, true, "留空保留已保存密码");
                    label(ui, "外观", p);
                    ui.horizontal(|ui| {
                        for (value, title) in
                            [("system", "跟随系统"), ("light", "浅色"), ("dark", "深色")]
                        {
                            radio(ui, &mut self.draft.theme, value, title, p);
                        }
                    });
                    label(ui, "自动刷新间隔（分钟，1–1440）", p);
                    ui.horizontal(|ui| {
                        let width = ui.available_width() - 80.;
                        ui.add_sized(
                            [width, 32.],
                            egui::TextEdit::singleline(&mut self.interval_text)
                                .margin(egui::Margin::symmetric(10, 7)),
                        );
                        if ui.add_sized([32., 32.], egui::Button::new("−")).clicked() {
                            let n = self
                                .interval_text
                                .parse::<u32>()
                                .unwrap_or(10)
                                .saturating_sub(1)
                                .max(1);
                            self.interval_text = n.to_string();
                        }
                        if ui.add_sized([32., 32.], egui::Button::new("+")).clicked() {
                            let n = self
                                .interval_text
                                .parse::<u32>()
                                .unwrap_or(10)
                                .saturating_add(1)
                                .min(1440);
                            self.interval_text = n.to_string();
                        }
                    });
                    label(ui, "刷新条件 · 网络适配器", p);
                    egui::ComboBox::from_id_salt("adapter")
                        .width(ui.available_width())
                        .wrap_mode(egui::TextWrapMode::Truncate)
                        .height(200.)
                        .selected_text(if self.draft.adapter.is_empty() {
                            "不限"
                        } else {
                            &self.draft.adapter
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.draft.adapter, String::new(), "不限");
                            for (name, ips) in &self.adapter_list {
                                ui.selectable_value(
                                    &mut self.draft.adapter,
                                    name.clone(),
                                    format!("{name} · {}", ips.join(", ")),
                                );
                            }
                        });
                    label(ui, "刷新条件 · 本机 IPv4（留空不限）", p);
                    edit(ui, &mut self.draft.ip, false, "留空表示不限 IP");
                    label(ui, "界面字体", p);
                    egui::ComboBox::from_id_salt("font")
                        .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
                        .width(ui.available_width())
                        .wrap_mode(egui::TextWrapMode::Truncate)
                        .selected_text(&self.draft.font)
                        .show_ui(ui, |ui| {
                            ui.add(
                                egui::TextEdit::singleline(&mut self.font_filter)
                                    .hint_text("搜索已安装字体"),
                            );
                            let filter = self.font_filter.to_lowercase();
                            egui::ScrollArea::vertical()
                                .max_height(200.)
                                .show(ui, |ui| {
                                    for name in &catalog().names {
                                        if filter.is_empty()
                                            || name.to_lowercase().contains(&filter)
                                        {
                                            if ui
                                                .selectable_value(
                                                    &mut self.draft.font,
                                                    name.clone(),
                                                    name,
                                                )
                                                .clicked()
                                            {
                                                ui.close();
                                            }
                                        }
                                    }
                                });
                        });
                    label(ui, "正文字重", p);
                    ui.add(
                        egui::DragValue::new(&mut self.draft.font_weight)
                            .range(100..=900)
                            .speed(2.)
                            .suffix("  / 100–900"),
                    );
                    label(ui, "标题与数字字重", p);
                    ui.add(
                        egui::DragValue::new(&mut self.draft.heading_weight)
                            .range(100..=900)
                            .speed(2.)
                            .suffix("  / 100–900"),
                    );
                    egui::Frame::new()
                        .fill(p.field)
                        .corner_radius(4)
                        .stroke(Stroke::new(1., p.border))
                        .inner_margin(12)
                        .show(ui, |ui| {
                            ui.set_min_width(ui.available_width());
                            ui.label(RichText::new("Aa 中文 · CPE 流量监控").size(14.));
                            ui.label(
                                RichText::new("123.45 GB")
                                    .font(semibold(26.))
                                    .color(p.green),
                            );
                            ui.label(
                                RichText::new("静态字体使用最接近的已安装字重")
                                    .size(11.)
                                    .color(p.secondary),
                            );
                        });
                    ui.add_space(4.);
                    ui.separator();
                    switch(ui, "开机自动启动", &mut self.draft.autostart, p);
                    switch(ui, "窗口始终置顶", &mut self.draft.topmost, p);
                    egui::CollapsingHeader::new("套餐设置").show(ui, |ui| {
                        let mut manual = self.draft.quota_gb.is_some();
                        if ui.checkbox(&mut manual, "手动指定每月额度（GB）").changed() {
                            self.draft.quota_gb = if manual {
                                Some(
                                    self.snapshot
                                        .as_ref()
                                        .and_then(|s| s.total)
                                        .map(|v| v / 1024_f64.powi(3))
                                        .unwrap_or(1.),
                                )
                            } else {
                                None
                            };
                        }
                        if let Some(q) = &mut self.draft.quota_gb {
                            ui.add(egui::DragValue::new(q).range(0.01..=1_000_000.));
                        }
                        let mut manual = self.draft.reset_day.is_some();
                        if ui.checkbox(&mut manual, "手动指定每月清零日").changed() {
                            self.draft.reset_day = if manual { Some(1) } else { None };
                        }
                        if let Some(d) = &mut self.draft.reset_day {
                            ui.add(egui::DragValue::new(d).range(1..=31));
                        }
                        ui.label(
                            RichText::new("留空时从 CPE 读取。仅影响小组件显示。")
                                .size(12.)
                                .color(p.secondary),
                        );
                    });
                    ui.add_space(16.);
                });
            });
        if before
            != (
                self.draft.theme.clone(),
                self.draft.font.clone(),
                self.draft.font_weight,
                self.draft.heading_weight,
            )
        {
            // Avoid loading large CJK faces for each intermediate drag value.
            self.appearance_due =
                Some(std::time::Instant::now() + std::time::Duration::from_millis(180));
        }
        if let Some(due) = self.appearance_due {
            if std::time::Instant::now() >= due {
                self.appearance_due = None;
                apply_appearance(ctx, &self.draft);
                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(SETTINGS_SIZE));
            } else {
                ctx.request_repaint_after(due.saturating_duration_since(std::time::Instant::now()));
            }
        }
        if save {
            self.save(ctx);
        } else if cancel {
            self.close_settings(ctx);
        }
    }
}
