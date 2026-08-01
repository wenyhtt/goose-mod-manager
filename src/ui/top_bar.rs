use crate::app::GooseModManager;
use crate::icons::paint_dropdown_arrow;
use crate::models::{IconKind, SortOption, Tab};
use crate::theme::{
    BG_DARK, CARD_BG, CARD_BG_HOVER, FOCUS_COLOR, ICON_COLOR, PILL_HEIGHT, PILL_RADIUS, TEXT_WHITE, poppins,
    poppins_sm,
};
use crate::ui::widgets::{icon_button, pill_button};
use eframe::egui::{
    self, Align, Color32, CornerRadius, Layout, Pos2, Rect, Stroke, StrokeKind, Vec2,
};

pub fn render_top_bar(app: &mut GooseModManager, ui: &mut egui::Ui) {
    let top_bar_height = 110.0;

    // We'll capture the exact vertical position of the tabs to align the sort dropdown
    let mut tabs_top_y = ui.max_rect().top() + 10.0;

    ui.allocate_ui_with_layout(
        Vec2::new(ui.available_width(), top_bar_height),
        Layout::top_down(Align::LEFT),
        |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 10.0;
                ui.spacing_mut().item_spacing.y = 10.0;

                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = 18.0;

                        // Tabs
                        let tabs_resp = ui
                            .horizontal(|ui| {
                                ui.spacing_mut().item_spacing.x = 10.0;
                                let browse_resp = pill_button(
                                    ui,
                                    "tab_browse",
                                    "BROWSE",
                                    app.active_tab == Tab::Browse,
                                );
                                
                                if app.needs_initial_focus {
                                    browse_resp.request_focus();
                                    app.needs_initial_focus = false;
                                }

                                if browse_resp.clicked() {
                                    app.active_tab = Tab::Browse;
                                    app.current_page = 0;
                                }
                                if pill_button(
                                    ui,
                                    "tab_installed",
                                    "INSTALLED",
                                    app.active_tab == Tab::Installed,
                                ).clicked() {
                                    app.active_tab = Tab::Installed;
                                    app.current_page = 0;
                                }
                            })
                            .response;
                        tabs_top_y = tabs_resp.rect.top();

                        // Arrow navigation
                        let arrows_resp = ui
                            .horizontal(|ui| {
                                ui.spacing_mut().item_spacing.x = 10.0;
                                if icon_button(ui, IconKind::LeftArrow).clicked() && app.current_page > 0 {
                                    app.current_page -= 1;
                                }
                                if icon_button(ui, IconKind::RightArrow).clicked()
                                    && app.current_page < app.total_pages().saturating_sub(1)
                                {
                                    app.current_page += 1;
                                }
                            })
                            .response;

                        // Page indicator (Centered perfectly on screen, vertically aligned with arrows)
                        let page_text =
                            format!("Page {} of {}", app.current_page + 1, app.total_pages());

                        ui.painter().text(
                            Pos2::new(ui.max_rect().center().x, arrows_resp.rect.center().y),
                            egui::Align2::CENTER_CENTER,
                            page_text,
                            poppins(),
                            TEXT_WHITE,
                        );
                    });
                });
            });

            // Sort controls at top right
            let top_right = Pos2::new(ui.max_rect().right() - 10.0, tabs_top_y);
            render_sort_controls(app, ui, top_right);
        },
    );
}

fn render_sort_controls(app: &mut GooseModManager, ui: &mut egui::Ui, top_right: Pos2) {
    let sort_label_width = 80.0;
    let dropdown_width = 196.0;
    let total_width = sort_label_width + 10.0 + dropdown_width;
    let left = top_right.x - total_width;
    let top = top_right.y;

    // Calculate rects first
    let dropdown_rect = Rect::from_min_size(
        Pos2::new(left + sort_label_width + 10.0, top),
        Vec2::new(dropdown_width, PILL_HEIGHT),
    );

    // Allocate interaction rects first (mutable borrows)
    let dropdown_response = ui.allocate_rect(dropdown_rect, egui::Sense::click());

    // Collect dropdown menu interactions
    let mut clicked_option: Option<SortOption> = None;
    let mut any_item_hovered = false;

    let menu_rect = Rect::from_min_size(
        Pos2::new(dropdown_rect.left(), dropdown_rect.bottom() + 4.0),
        Vec2::new(dropdown_width, PILL_HEIGHT * SortOption::all().len() as f32),
    );

    let painter = ui.painter();

    // "SORT BY" label
    painter.text(
        Pos2::new(left, top + PILL_HEIGHT / 2.0),
        egui::Align2::LEFT_CENTER,
        "SORT BY",
        poppins_sm(),
        TEXT_WHITE,
    );

    // Dropdown pill background
    painter.rect_filled(dropdown_rect, CornerRadius::same(PILL_RADIUS), CARD_BG);

    if dropdown_response.has_focus() {
        painter.rect_stroke(
            dropdown_rect.expand(4.0),
            CornerRadius::same(PILL_RADIUS + 4),
            Stroke::new(2.0, FOCUS_COLOR),
            StrokeKind::Outside,
        );
    }

    // Dropdown text
    painter.text(
        Pos2::new(dropdown_rect.left() + 22.0, dropdown_rect.center().y),
        egui::Align2::LEFT_CENTER,
        app.sort_by.label(),
        poppins(),
        TEXT_WHITE,
    );

    // Dropdown arrow
    let arrow_center = Pos2::new(dropdown_rect.right() - 30.0, dropdown_rect.center().y);
    paint_dropdown_arrow(painter, arrow_center, 24.0, ICON_COLOR);

    // Dropdown menu painting as an OVERLAY
    if app.sort_dropdown_open {
        egui::Area::new(egui::Id::new("sort_dropdown_menu"))
            .fixed_pos(menu_rect.min)
            .order(egui::Order::Foreground)
            .show(ui.ctx(), |area_ui| {
                let mut local_states = Vec::new();
                for (i, option) in SortOption::all().iter().enumerate() {
                    let item_rect = Rect::from_min_size(
                        Pos2::new(menu_rect.left(), menu_rect.top() + i as f32 * PILL_HEIGHT),
                        Vec2::new(dropdown_width, PILL_HEIGHT),
                    );
                    let item_resp = area_ui.allocate_rect(item_rect, egui::Sense::click());
                    local_states.push((
                        item_rect,
                        item_resp.hovered(),
                        item_resp.clicked(),
                        item_resp.has_focus(),
                        *option,
                    ));
                }

                let area_painter = area_ui.painter();
                area_painter.rect_filled(menu_rect, CornerRadius::same(12), CARD_BG);
                area_painter.rect_stroke(
                    menu_rect,
                    CornerRadius::same(12),
                    Stroke::new(1.0, Color32::from_rgba_premultiplied(255, 255, 255, 30)),
                    StrokeKind::Outside,
                );

                for (item_rect, hovered, clicked, has_focus, option) in local_states {
                    let (bg, text_color) = if option == app.sort_by {
                        (TEXT_WHITE, BG_DARK)
                    } else if hovered {
                        any_item_hovered = true;
                        (CARD_BG_HOVER, TEXT_WHITE)
                    } else {
                        (Color32::TRANSPARENT, TEXT_WHITE)
                    };

                    // TWEAK THIS: Increase to make hover background smaller, decrease to make it larger!
                    let shrink_amount = 6.0;
                    let visual_rect = item_rect.shrink(shrink_amount);

                    if bg != Color32::TRANSPARENT {
                        area_painter.rect_filled(visual_rect, CornerRadius::same(6), bg);
                    }

                    if has_focus {
                        area_painter.rect_stroke(
                            visual_rect.expand(2.0),
                            CornerRadius::same(6 + 2),
                            Stroke::new(2.0, FOCUS_COLOR),
                            StrokeKind::Outside,
                        );
                    }
                    area_painter.text(
                        Pos2::new(item_rect.left() + 22.0, item_rect.center().y),
                        egui::Align2::LEFT_CENTER,
                        option.label(),
                        poppins_sm(),
                        text_color,
                    );
                    if clicked {
                        clicked_option = Some(option);
                    }
                }
            });
    }

    // Handle cursor icons
    if dropdown_response.hovered() || any_item_hovered {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }

    // Handle state changes
    if dropdown_response.clicked() {
        app.sort_dropdown_open = !app.sort_dropdown_open;
    }

    if let Some(option) = clicked_option {
        app.sort_by = option;
        app.sort_dropdown_open = false;
    }

    // Close dropdown when clicking elsewhere
    if app.sort_dropdown_open && ui.input(|i| i.pointer.any_click()) {
        let pointer = ui.input(|i| i.pointer.interact_pos());
        if let Some(pos) = pointer {
            if !dropdown_rect.contains(pos) && !menu_rect.contains(pos) {
                app.sort_dropdown_open = false;
            }
        }
    }
}
