use crate::app::GooseModManager;
use crate::icons::paint_download_icon;
use crate::models::ModVersion;
use crate::theme::{
    noto_sans_bold, noto_sans_light, noto_sans_regular, BG_DARK, CARD_BG, CARD_BG_HOVER,
    FOCUS_COLOR, FONT_MD, FONT_SM, FONT_XS, ICON_COLOR, TEXT_WHITE,
};
use eframe::egui::{self, Color32, CornerRadius, Id, Pos2, Rect, Sense, Stroke, StrokeKind, Vec2};

const VERSION_ROW_HEIGHT: f32 = 76.0;

pub fn render_versions(app: &mut GooseModManager, ctx: &egui::Context) {
    if !app.versions_dialog_open {
        return;
    }

    let viewport = ctx.content_rect();
    let size = Vec2::new(
        392.0_f32.min((viewport.width() - 24.0).max(320.0)),
        520.0_f32.min((viewport.height() - 24.0).max(300.0)),
    );
    let dialog = Rect::from_center_size(viewport.center(), size);
    let versions = app.versions.clone();
    let loading = app.versions_loading();
    let downloading = app.version_downloading();

    egui::Area::new(Id::new("mod_versions"))
        .order(egui::Order::Tooltip)
        .fixed_pos(viewport.min)
        .show(ctx, |ui| {
            ui.set_min_size(viewport.size());
            ui.painter()
                .rect_filled(viewport, 0.0, Color32::from_black_alpha(100));

            if !app.versions_dialog_just_opened
                && ctx.input(|input| input.pointer.any_click())
                && ctx.input(|input| {
                    input
                        .pointer
                        .interact_pos()
                        .is_some_and(|position| !dialog.contains(position))
                })
            {
                app.close_versions_dialog();
                return;
            }

            ui.painter()
                .rect_filled(dialog, CornerRadius::same(12), BG_DARK);
            ui.painter().text(
                Pos2::new(dialog.left() + 14.0, dialog.top() + 28.0),
                egui::Align2::LEFT_CENTER,
                "All Versions",
                noto_sans_bold(FONT_MD),
                TEXT_WHITE,
            );

            let content = Rect::from_min_max(
                Pos2::new(dialog.left() + 12.0, dialog.top() + 56.0),
                Pos2::new(dialog.right() - 12.0, dialog.bottom() - 12.0),
            );
            if loading {
                status(ui, content, "Fetching versions...");
            } else if let Some(error) = &app.versions_error {
                status(ui, content, error);
            } else {
                paint_version_list(ui, app, &versions, content, downloading);
            }

            if let Some(message) = &app.download_status {
                ui.painter().text(
                    Pos2::new(content.center().x, dialog.bottom() - 18.0),
                    egui::Align2::CENTER_CENTER,
                    message,
                    noto_sans_light(FONT_XS),
                    TEXT_WHITE.gamma_multiply(0.7),
                );
            }
        });

    app.versions_dialog_just_opened = false;
}

fn paint_version_list(
    ui: &mut egui::Ui,
    app: &mut GooseModManager,
    versions: &[ModVersion],
    rect: Rect,
    downloading: bool,
) {
    let status_height = f32::from(app.download_status.is_some()) * 24.0;
    ui.scope_builder(
        egui::UiBuilder::new().max_rect(Rect::from_min_max(
            rect.min,
            Pos2::new(rect.right(), rect.bottom() - status_height),
        )),
        |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    for (index, version) in versions.iter().enumerate() {
                        let (row, response) = ui.allocate_exact_size(
                            Vec2::new(ui.available_width(), VERSION_ROW_HEIGHT),
                            if downloading {
                                Sense::hover()
                            } else {
                                Sense::click()
                            },
                        );
                        paint_version_row(ui, row, version, response.has_focus(), downloading);
                        if response.hovered() && !downloading {
                            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                        }
                        if (app.versions_dialog_just_opened
                            || ui.ctx().memory(|mem| mem.focused().is_none()))
                            && index == 0
                        {
                            response.request_focus();
                        }
                        if response.clicked() && !downloading {
                            app.start_version_download(version.clone());
                        }
                    }
                });
        },
    );
}

fn paint_version_row(
    ui: &egui::Ui,
    rect: Rect,
    version: &ModVersion,
    focused: bool,
    downloading: bool,
) {
    let background = if !downloading && ui.rect_contains_pointer(rect) {
        CARD_BG_HOVER
    } else {
        CARD_BG
    };
    ui.painter()
        .rect_filled(rect, CornerRadius::same(12), background);
    let file_icon = Rect::from_center_size(
        Pos2::new(rect.left() + 21.0, rect.top() + 24.0),
        Vec2::new(14.0, 18.0),
    );
    ui.painter()
        .rect_filled(file_icon, CornerRadius::same(2), ICON_COLOR);
    ui.painter().text(
        Pos2::new(rect.left() + 38.0, rect.top() + 25.0),
        egui::Align2::LEFT_CENTER,
        &version.label,
        noto_sans_regular(FONT_SM),
        TEXT_WHITE,
    );
    let label_width = ui
        .painter()
        .layout_no_wrap(
            version.label.clone(),
            noto_sans_regular(FONT_SM),
            TEXT_WHITE,
        )
        .size()
        .x;
    let size_left = (rect.left() + 46.0 + label_width).min(rect.right() - 94.0);
    let size = ui
        .painter()
        .layout_no_wrap(version.size.clone(), noto_sans_regular(FONT_XS), TEXT_WHITE)
        .size();
    let size_rect = Rect::from_min_size(
        Pos2::new(size_left, rect.top() + 12.0),
        size + Vec2::new(10.0, 4.0),
    );
    ui.painter()
        .rect_filled(size_rect, CornerRadius::same(4), CARD_BG_HOVER);
    ui.painter().text(
        size_rect.center(),
        egui::Align2::CENTER_CENTER,
        &version.size,
        noto_sans_regular(FONT_XS),
        TEXT_WHITE.gamma_multiply(0.65),
    );
    let metadata = match (
        &version.downloads.is_empty(),
        &version.published_at.is_empty(),
    ) {
        (false, false) => format!("{} downloads - {}", version.downloads, version.published_at),
        (false, true) => format!("{} downloads", version.downloads),
        (true, false) => version.published_at.clone(),
        (true, true) => String::new(),
    };
    ui.painter().text(
        Pos2::new(rect.left() + 10.0, rect.bottom() - 16.0),
        egui::Align2::LEFT_CENTER,
        metadata,
        noto_sans_light(FONT_XS),
        TEXT_WHITE.gamma_multiply(0.65),
    );
    paint_download_icon(
        ui.painter(),
        Pos2::new(rect.right() - 22.0, rect.top() + 26.0),
        18.0,
        ICON_COLOR,
    );
    if focused {
        ui.painter().rect_stroke(
            rect.expand(3.0),
            CornerRadius::same(15),
            Stroke::new(2.0, FOCUS_COLOR),
            StrokeKind::Outside,
        );
    }
}

fn status(ui: &egui::Ui, rect: Rect, message: &str) {
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        message,
        noto_sans_regular(FONT_SM),
        TEXT_WHITE.gamma_multiply(0.7),
    );
}
