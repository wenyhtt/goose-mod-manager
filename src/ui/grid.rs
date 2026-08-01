use eframe::egui::{self, CornerRadius, Pos2, Rect, Stroke, StrokeKind, Vec2};
use crate::app::GooseModManager;
use crate::models::ModEntry;
use crate::theme::{CARD_BG, CARD_BG_HOVER, CARD_RADIUS, FOCUS_COLOR, ICON_COLOR, TEXT_WHITE, poppins_sm};
use crate::icons::paint_download_icon;

pub fn render_grid(app: &GooseModManager, ui: &mut egui::Ui) {
    let page_mods = app.page_mods().to_vec();
    let available = ui.available_size();
    let gap = 10.0;
    let padding = 10.0;
    let total_w = available.x - padding * 2.0;
    let total_h = available.y - padding * 2.0;
    let card_w = (total_w - (app.cols as f32 - 1.0) * gap) / app.cols as f32;
    let card_h = (total_h - (app.rows as f32 - 1.0) * gap) / app.rows as f32;
    let origin = ui.cursor().min + Vec2::new(padding, padding);

    // Collect interaction data first (mutable borrows for allocate_rect)
    let mut card_data: Vec<(Rect, bool, bool, ModEntry)> = Vec::new();
    for row in 0..app.rows {
        for col in 0..app.cols {
            let idx = row * app.cols + col;
            if idx >= page_mods.len() {
                continue;
            }
            let x = origin.x + col as f32 * (card_w + gap);
            let y = origin.y + row as f32 * (card_h + gap);
            let card_rect = Rect::from_min_size(Pos2::new(x, y), Vec2::new(card_w, card_h));
            let card_resp = ui.allocate_rect(card_rect, egui::Sense::click());
            let is_hovered = card_resp.hovered();
            let has_focus = card_resp.has_focus();
            if is_hovered {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            }
            card_data.push((card_rect, is_hovered, has_focus, page_mods[idx].clone()));
        }
    }

    // Paint all cards (needs both painter and ui for image rendering)
    for (rect, is_hovered, has_focus, mod_entry) in &card_data {
        let painter = ui.painter().clone();
        paint_card(app, &painter, ui, *rect, *is_hovered, *has_focus, mod_entry);
    }

    // Reserve the space
    ui.allocate_space(available);
}

fn paint_card(
    _app: &GooseModManager,
    painter: &egui::Painter,
    ui: &mut egui::Ui,
    rect: Rect,
    is_hovered: bool,
    has_focus: bool,
    mod_entry: &ModEntry,
) {
    // Card background
    let bg = if is_hovered { CARD_BG_HOVER } else { CARD_BG };
    painter.rect_filled(rect, CornerRadius::same(CARD_RADIUS), bg);

    if has_focus {
        painter.rect_stroke(
            rect.expand(4.0),
            CornerRadius::same(CARD_RADIUS + 4),
            Stroke::new(2.0, FOCUS_COLOR),
            StrokeKind::Outside,
        );
    }

    // Image area (top ~58%)
    let image_height = rect.height() * 0.58;
    let image_rect = Rect::from_min_size(rect.min, Vec2::new(rect.width(), image_height));

    let top_rounding = CornerRadius {
        nw: CARD_RADIUS,
        ne: CARD_RADIUS,
        sw: 0,
        se: 0,
    };

    // Use egui::Image with include_bytes via ImageSource::Bytes
    let image = egui::Image::new(egui::ImageSource::Bytes {
        uri: "bytes://card_image.png".into(),
        bytes: egui::load::Bytes::Static(include_bytes!("../../assets/card_image.png")),
    });

    // Cover crop: calculate UV to fill the rect without stretching,
    // cropping excess from the center (like CSS object-fit: cover)
    let rect_aspect = image_rect.width() / image_rect.height();
    let cropped_image = if let Some(natural_size) = image.load_and_calc_size(ui, Vec2::splat(f32::INFINITY)) {
        let img_aspect = natural_size.x / natural_size.y;
        let uv = if img_aspect > rect_aspect {
            // Image is wider — crop sides
            let visible = rect_aspect / img_aspect;
            let off = (1.0 - visible) / 2.0;
            Rect::from_min_max(Pos2::new(off, 0.0), Pos2::new(1.0 - off, 1.0))
        } else {
            // Image is taller — crop top/bottom
            let visible = img_aspect / rect_aspect;
            let off = (1.0 - visible) / 2.0;
            Rect::from_min_max(Pos2::new(0.0, off), Pos2::new(1.0, 1.0 - off))
        };
        image.uv(uv)
    } else {
        image // fallback while loading
    };

    cropped_image
        .corner_radius(top_rounding)
        .fit_to_exact_size(image_rect.size())
        .paint_at(ui, image_rect);

    // Info area
    let info_top = image_rect.bottom() + 2.0;
    let info_padding = 12.0;

    // Mod name
    painter.text(
        Pos2::new(rect.left() + info_padding, info_top + 10.0),
        egui::Align2::LEFT_TOP,
        &mod_entry.name,
        poppins_sm(),
        TEXT_WHITE,
    );

    // Bottom row
    let bottom_y = rect.bottom() - info_padding - 8.0;

    // Category
    painter.text(
        Pos2::new(rect.left() + info_padding, bottom_y),
        egui::Align2::LEFT_CENTER,
        &mod_entry.category,
        poppins_sm(),
        TEXT_WHITE,
    );

    // Download icon
    let icon_center = Pos2::new(rect.right() - info_padding - 12.0, bottom_y);
    paint_download_icon(painter, icon_center, 20.0, ICON_COLOR);

    // Size
    painter.text(
        Pos2::new(icon_center.x - 22.0, bottom_y),
        egui::Align2::RIGHT_CENTER,
        &mod_entry.size,
        poppins_sm(),
        TEXT_WHITE,
    );
}
