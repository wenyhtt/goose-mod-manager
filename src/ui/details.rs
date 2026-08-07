use crate::app::GooseModManager;
use crate::icons::{paint_download_icon, paint_heart_icon, paint_left_arrow, paint_right_arrow};
use crate::models::ModEntry;
use crate::theme::{
    BG_DARK, CARD_BG, CARD_BG_HOVER, FOCUS_COLOR, ICON_COLOR, TEXT_WHITE, poppins, poppins_sm,
    poppins_xs,
};
use crate::ui::images::{bytes_image, fallback_image, paint_cover_image};
use eframe::egui::{self, Color32, CornerRadius, Id, Pos2, Rect, Sense, Stroke, StrokeKind, Vec2};

pub const DETAIL_IMAGE_CURRENT_ID: &str = "detail_image_current";
pub const DETAIL_IMAGE_NEXT_ID: &str = "detail_image_next";
pub const DETAIL_VIEW_WEB_ID: &str = "detail_view_web";
pub const DETAIL_ARROW_LEFT_ID: &str = "detail_arrow_left";
pub const DETAIL_ARROW_RIGHT_ID: &str = "detail_arrow_right";

pub fn render_details(app: &mut GooseModManager, ctx: &egui::Context) {
    let Some(mod_entry) = app.selected_mod().cloned() else {
        return;
    };

    let viewport = ctx.content_rect();
    let fullscreen = app.rows == 1;
    let dialog_size = if fullscreen {
        viewport.size()
    } else {
        Vec2::new(
            852.0_f32.min((viewport.width() - 24.0).max(320.0)),
            482.0_f32.min((viewport.height() - 24.0).max(320.0)),
        )
    };
    let dialog_rect = Rect::from_center_size(viewport.center(), dialog_size);

    egui::Area::new(Id::new("mod_details"))
        .order(egui::Order::Foreground)
        .fixed_pos(viewport.min)
        .show(ctx, |ui| {
            ui.set_min_size(viewport.size());
            ui.painter()
                .rect_filled(viewport, 0.0, Color32::from_black_alpha(170));

            if !fullscreen
                && !app.details_just_opened
                && ctx.input(|i| i.pointer.any_click())
                && ctx.input(|i| {
                    i.pointer
                        .interact_pos()
                        .is_some_and(|pos| !dialog_rect.contains(pos))
                })
            {
                app.close_details();
                return;
            }

            let rect = dialog_rect;
            let rounding = if fullscreen { 0 } else { 24 };
            ui.painter()
                .rect_filled(rect, CornerRadius::same(rounding), BG_DARK);

            let pad = if fullscreen { 24.0 } else { 14.0 };
            let inner = rect.shrink(pad);
            paint_header(ui, &mod_entry, inner);
            paint_images(ui, app, &mod_entry, inner);
            paint_footer(ui, app, &mod_entry, inner);
        });

    app.details_just_opened = false;
}

fn paint_header(ui: &mut egui::Ui, mod_entry: &ModEntry, rect: Rect) {
    ui.put(
        Rect::from_min_size(rect.min, Vec2::new(rect.width() - 180.0, 30.0)),
        egui::Label::new(
            egui::RichText::new(&mod_entry.name)
                .font(poppins())
                .color(TEXT_WHITE),
        )
        .truncate(),
    );

    if !mod_entry.author.is_empty() {
        ui.painter().text(
            Pos2::new(rect.left(), rect.top() + 44.0),
            egui::Align2::LEFT_CENTER,
            format!("By {}", mod_entry.author),
            poppins_xs(),
            TEXT_WHITE.gamma_multiply(0.65),
        );
    }

    let mut right = rect.right();
    if !mod_entry.downloads.is_empty() {
        right -= stat_pill(
            ui,
            right,
            rect.top() + 14.0,
            &mod_entry.downloads,
            paint_download_icon,
        ) + 8.0;
    }
    if !mod_entry.likes.is_empty() {
        stat_pill(
            ui,
            right,
            rect.top() + 14.0,
            &mod_entry.likes,
            paint_heart_icon,
        );
    }
}

fn paint_images(ui: &mut egui::Ui, app: &mut GooseModManager, mod_entry: &ModEntry, rect: Rect) {
    let top = rect.top() + 82.0;
    let bottom = rect.bottom() - 92.0;
    let gap = 10.0;
    let count = preview_count(mod_entry);
    let peek_next = rect.width() >= 640.0 && count > 1;
    let width = if peek_next {
        rect.width() * 0.58
    } else {
        rect.width()
    };
    let left = Rect::from_min_size(Pos2::new(rect.left(), top), Vec2::new(width, bottom - top));

    let old_clip = ui.clip_rect();
    ui.set_clip_rect(old_clip.intersect(rect));
    let left_resp = ui.interact(left, Id::new(DETAIL_IMAGE_CURRENT_ID), Sense::click());
    paint_preview(ui, mod_entry, app.detail_image_offset, left);

    if peek_next {
        let right = left.translate(Vec2::new(width + gap, 0.0));
        let visible = right.intersect(rect);
        let right_resp = ui.interact(visible, Id::new(DETAIL_IMAGE_NEXT_ID), Sense::click());
        paint_preview(ui, mod_entry, app.detail_image_offset + 1, right);
        if right_resp.has_focus() {
            app.carousel_next();
            left_resp.request_focus();
            ui.ctx().request_repaint();
        }
    }
    ui.set_clip_rect(old_clip);
    focus(ui, left, left_resp.has_focus(), 10);

    if app.details_just_opened || ui.ctx().memory(|mem| mem.focused().is_none()) {
        left_resp.request_focus();
    }
}

fn paint_footer(ui: &mut egui::Ui, app: &mut GooseModManager, mod_entry: &ModEntry, rect: Rect) {
    let y = rect.bottom() - 52.0;
    dialog_button(
        ui,
        "detail_install",
        Rect::from_min_size(Pos2::new(rect.left(), y), Vec2::new(122.0, 46.0)),
        "INSTALL",
        false,
    );
    if dialog_button(
        ui,
        DETAIL_VIEW_WEB_ID,
        Rect::from_min_size(Pos2::new(rect.left() + 134.0, y), Vec2::new(184.0, 46.0)),
        "VIEW ON WEB",
        true,
    )
    .clicked()
    {
        ui.ctx().open_url(egui::OpenUrl::new_tab(&mod_entry.url));
    }

    let count = preview_count(mod_entry);
    if count > 1 {
        let right = Rect::from_min_size(Pos2::new(rect.right() - 52.0, y - 2.0), Vec2::splat(52.0));
        let left = right.translate(Vec2::new(-62.0, 0.0));
        if arrow_button(ui, DETAIL_ARROW_LEFT_ID, left, false).clicked() {
            app.carousel_prev();
        }
        if arrow_button(ui, DETAIL_ARROW_RIGHT_ID, right, true).clicked() {
            app.carousel_next();
        }
    }
}

fn paint_preview(ui: &mut egui::Ui, mod_entry: &ModEntry, index: usize, rect: Rect) {
    let image = if mod_entry.detail_image_bytes.is_empty() {
        if let (Some(bytes), Some(path)) = (&mod_entry.image_bytes, &mod_entry.thumbnail_path) {
            bytes_image(bytes.clone(), path)
        } else {
            fallback_image()
        }
    } else {
        let index = index % mod_entry.detail_image_bytes.len();
        bytes_image(
            mod_entry.detail_image_bytes[index].clone(),
            &mod_entry.detail_image_paths[index],
        )
    };
    paint_cover_image(ui, rect, CornerRadius::same(10), image);
}

fn preview_count(mod_entry: &ModEntry) -> usize {
    if mod_entry.detail_image_bytes.is_empty() {
        usize::from(mod_entry.image_bytes.is_some())
    } else {
        mod_entry.detail_image_bytes.len()
    }
}

fn dialog_button(
    ui: &mut egui::Ui,
    id: &str,
    rect: Rect,
    label: &str,
    enabled: bool,
) -> egui::Response {
    let response = ui.interact(
        rect,
        Id::new(id),
        if enabled {
            Sense::click()
        } else {
            Sense::hover()
        },
    );
    let bg = if enabled && response.hovered() {
        CARD_BG_HOVER
    } else {
        CARD_BG
    };
    let text = if enabled {
        TEXT_WHITE
    } else {
        TEXT_WHITE.gamma_multiply(0.45)
    };
    ui.painter().rect_filled(rect, CornerRadius::same(23), bg);
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        label,
        poppins_sm(),
        text,
    );
    if enabled && response.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    focus(ui, rect, response.has_focus(), 23);
    response
}

fn arrow_button(ui: &mut egui::Ui, id: &str, rect: Rect, right: bool) -> egui::Response {
    let response = ui.interact(rect, Id::new(id), Sense::click());
    if response.clicked() {
        response.request_focus();
    }
    let bg = if response.hovered() {
        CARD_BG_HOVER
    } else {
        CARD_BG
    };
    ui.painter().rect_filled(rect, CornerRadius::same(28), bg);
    if right {
        paint_right_arrow(ui.painter(), rect.center(), 22.0, ICON_COLOR);
    } else {
        paint_left_arrow(ui.painter(), rect.center(), 22.0, ICON_COLOR);
    }
    if response.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    focus(ui, rect, response.has_focus(), 28);
    response
}

fn stat_pill(
    ui: &mut egui::Ui,
    right: f32,
    center_y: f32,
    value: &str,
    icon: fn(&egui::Painter, Pos2, f32, Color32),
) -> f32 {
    let text_size = ui
        .painter()
        .layout_no_wrap(value.to_string(), poppins_xs(), TEXT_WHITE)
        .size();
    let width = text_size.x + 34.0;
    let rect = Rect::from_center_size(
        Pos2::new(right - width / 2.0, center_y),
        Vec2::new(width, 26.0),
    );
    ui.painter()
        .rect_filled(rect, CornerRadius::same(13), CARD_BG);
    ui.painter().text(
        Pos2::new(rect.left() + 6.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        value,
        poppins_xs(),
        TEXT_WHITE,
    );
    icon(
        ui.painter(),
        Pos2::new(rect.right() - 13.0, rect.center().y),
        14.0,
        ICON_COLOR,
    );
    width
}

fn focus(ui: &egui::Ui, rect: Rect, has_focus: bool, radius: u8) {
    if has_focus {
        ui.painter().rect_stroke(
            rect.expand(4.0),
            CornerRadius::same(radius + 4),
            Stroke::new(2.0, FOCUS_COLOR),
            StrokeKind::Outside,
        );
    }
}
