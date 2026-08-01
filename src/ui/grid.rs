use eframe::egui::{self, Color32, CornerRadius, Pos2, Rect, Vec2};
use crate::app::GooseModManager;
use crate::models::ModEntry;
use crate::theme::{CARD_BG, CARD_BG_HOVER, CARD_RADIUS, GRID_COLS, GRID_ROWS, ICON_COLOR, TEXT_WHITE, poppins_sm};
use crate::icons::paint_download_icon;

pub fn render_grid(app: &GooseModManager, ui: &mut egui::Ui) {
    let page_mods = app.page_mods().to_vec();
    let available = ui.available_size();
    let gap = 10.0;
    let padding = 10.0;
    let total_w = available.x - padding * 2.0;
    let total_h = available.y - padding * 2.0;
    let card_w = (total_w - (GRID_COLS as f32 - 1.0) * gap) / GRID_COLS as f32;
    let card_h = (total_h - (GRID_ROWS as f32 - 1.0) * gap) / GRID_ROWS as f32;
    let origin = ui.cursor().min + Vec2::new(padding, padding);

    // Collect interaction data first (mutable borrows)
    let mut card_data: Vec<(Rect, bool, &ModEntry)> = Vec::new();
    for row in 0..GRID_ROWS {
        for col in 0..GRID_COLS {
            let idx = row * GRID_COLS + col;
            if idx >= page_mods.len() {
                continue;
            }
            let x = origin.x + col as f32 * (card_w + gap);
            let y = origin.y + row as f32 * (card_h + gap);
            let card_rect = Rect::from_min_size(Pos2::new(x, y), Vec2::new(card_w, card_h));
            let card_resp = ui.allocate_rect(card_rect, egui::Sense::click());
            let is_hovered = card_resp.hovered();
            if is_hovered {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            }
            card_data.push((card_rect, is_hovered, &page_mods[idx]));
        }
    }

    // Now paint all cards (immutable painter borrow)
    let painter = ui.painter();
    for (rect, is_hovered, mod_entry) in &card_data {
        paint_card(app, painter, *rect, *is_hovered, mod_entry);
    }

    // Reserve the space
    ui.allocate_space(available);
}

fn paint_card(
    app: &GooseModManager,
    painter: &egui::Painter,
    rect: Rect,
    is_hovered: bool,
    mod_entry: &ModEntry,
) {
    // Card background
    let bg = if is_hovered { CARD_BG_HOVER } else { CARD_BG };
    painter.rect_filled(rect, CornerRadius::same(CARD_RADIUS), bg);

    // Image area (top ~58%)
    let image_height = rect.height() * 0.58;
    let image_rect = Rect::from_min_size(rect.min, Vec2::new(rect.width(), image_height));

    if let Some(tex) = &app.card_texture {
        let top_rounding = CornerRadius {
            nw: CARD_RADIUS,
            ne: CARD_RADIUS,
            sw: 0,
            se: 0,
        };
        painter.rect_filled(image_rect, top_rounding, Color32::BLACK);

        let uv = Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(1.0, 1.0));
        let mut mesh = egui::Mesh::with_texture(tex.id());

        let r = CARD_RADIUS as f32;
        let segments = 8;

        let tl_center = Pos2::new(image_rect.left() + r, image_rect.top() + r);
        let tr_center = Pos2::new(image_rect.right() - r, image_rect.top() + r);

        let mut positions: Vec<Pos2> = Vec::new();

        // Top-left arc
        for i in 0..=segments {
            let angle = std::f32::consts::PI
                + (std::f32::consts::PI / 2.0) * (i as f32 / segments as f32);
            positions.push(Pos2::new(
                tl_center.x + r * angle.cos(),
                tl_center.y + r * angle.sin(),
            ));
        }

        // Top-right arc
        for i in 0..=segments {
            let angle = 3.0 * std::f32::consts::PI / 2.0
                + (std::f32::consts::PI / 2.0) * (i as f32 / segments as f32);
            positions.push(Pos2::new(
                tr_center.x + r * angle.cos(),
                tr_center.y + r * angle.sin(),
            ));
        }

        // Bottom corners (no rounding)
        positions.push(Pos2::new(image_rect.right(), image_rect.bottom()));
        positions.push(Pos2::new(image_rect.left(), image_rect.bottom()));

        for pos in &positions {
            let u = (pos.x - image_rect.left()) / image_rect.width();
            let v = (pos.y - image_rect.top()) / image_rect.height();
            mesh.vertices.push(egui::epaint::Vertex {
                pos: *pos,
                uv: Pos2::new(
                    uv.min.x + u * (uv.max.x - uv.min.x),
                    uv.min.y + v * (uv.max.y - uv.min.y),
                ),
                color: Color32::WHITE,
            });
        }

        let center = image_rect.center();
        let center_idx = mesh.vertices.len() as u32;
        mesh.vertices.push(egui::epaint::Vertex {
            pos: center,
            uv: Pos2::new(0.5, 0.5),
            color: Color32::WHITE,
        });

        let n = positions.len() as u32;
        for i in 0..n {
            mesh.indices.push(center_idx);
            mesh.indices.push(i);
            mesh.indices.push((i + 1) % n);
        }

        painter.add(egui::Shape::mesh(mesh));
    }

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
