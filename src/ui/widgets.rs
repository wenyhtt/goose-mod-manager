use eframe::egui::{self, CornerRadius, Stroke, StrokeKind, Vec2};
use crate::theme::{CARD_BG, CARD_BG_HOVER, FOCUS_COLOR, ICON_BTN_SIZE, ICON_COLOR, PILL_HEIGHT, PILL_RADIUS, PILL_WIDTH, TAB_ACTIVE, TAB_INACTIVE, TEXT_WHITE, poppins};
use crate::models::IconKind;
use crate::icons::{paint_left_arrow, paint_right_arrow};

pub fn pill_button(ui: &mut egui::Ui, id_source: &str, text: &str, active: bool) -> egui::Response {
    let desired = Vec2::new(PILL_WIDTH, PILL_HEIGHT);
    let (rect, _resp) = ui.allocate_exact_size(desired, egui::Sense::hover());
    let response = ui.interact(rect, ui.id().with(id_source), egui::Sense::click());

    let bg = if active {
        TAB_ACTIVE
    } else if response.hovered() {
        CARD_BG_HOVER
    } else {
        TAB_INACTIVE
    };

    let painter = ui.painter();
    painter.rect_filled(rect, CornerRadius::same(PILL_RADIUS), bg);
    painter.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        text,
        poppins(),
        TEXT_WHITE,
    );

    if response.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }

    if response.has_focus() {
        painter.rect_stroke(
            rect.expand(4.0),
            CornerRadius::same(PILL_RADIUS + 4),
            Stroke::new(2.0, FOCUS_COLOR),
            StrokeKind::Outside,
        );
    }

    response
}

pub fn icon_button(ui: &mut egui::Ui, kind: IconKind) -> egui::Response {
    let desired = Vec2::new(ICON_BTN_SIZE, ICON_BTN_SIZE);
    let (rect, response) = ui.allocate_exact_size(desired, egui::Sense::click());

    let bg = if response.hovered() {
        CARD_BG_HOVER
    } else {
        CARD_BG
    };

    let painter = ui.painter();
    painter.rect_filled(rect, CornerRadius::same(30), bg);

    let center = rect.center();
    match kind {
        IconKind::LeftArrow => paint_left_arrow(painter, center, 20.0, ICON_COLOR),
        IconKind::RightArrow => paint_right_arrow(painter, center, 20.0, ICON_COLOR),
    }

    if response.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }

    if response.has_focus() {
        painter.rect_stroke(
            rect.expand(4.0),
            CornerRadius::same(30 + 4),
            Stroke::new(2.0, FOCUS_COLOR),
            StrokeKind::Outside,
        );
    }

    response
}
