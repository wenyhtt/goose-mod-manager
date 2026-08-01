use eframe::egui::{self, Align, Color32, CornerRadius, Layout, Pos2, Rect, Stroke, StrokeKind, Vec2};
use crate::app::GooseModManager;
use crate::models::{IconKind, SortOption, Tab};
use crate::theme::{CARD_BG, CARD_BG_HOVER, ICON_COLOR, PILL_HEIGHT, PILL_RADIUS, TEXT_WHITE, poppins, poppins_sm};
use crate::icons::paint_dropdown_arrow;
use crate::ui::widgets::{icon_button, pill_button};

pub fn render_top_bar(app: &mut GooseModManager, ui: &mut egui::Ui) {
    let top_bar_height = 110.0;
    ui.allocate_ui_with_layout(
        Vec2::new(ui.available_width(), top_bar_height),
        Layout::top_down(Align::LEFT),
        |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 10.0;

                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = 18.0;

                        // Tabs
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 10.0;
                            if pill_button(ui, "BROWSE", app.active_tab == Tab::Browse) {
                                app.active_tab = Tab::Browse;
                                app.current_page = 0;
                            }
                            if pill_button(ui, "INSTALLED", app.active_tab == Tab::Installed)
                            {
                                app.active_tab = Tab::Installed;
                                app.current_page = 0;
                            }
                        });

                        // Arrow navigation
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 10.0;
                            if icon_button(ui, IconKind::LeftArrow) && app.current_page > 0
                            {
                                app.current_page -= 1;
                            }
                            if icon_button(ui, IconKind::RightArrow)
                                && app.current_page < app.total_pages().saturating_sub(1)
                            {
                                app.current_page += 1;
                            }
                        });
                    });

                    // Page indicator
                    let page_text = format!(
                        "Page {} of {}",
                        app.current_page + 1,
                        app.total_pages()
                    );
                    let avail = ui.available_width();
                    ui.add_space((avail / 2.0 - 140.0).max(20.0));
                    ui.label(
                        egui::RichText::new(page_text)
                            .font(poppins())
                            .color(TEXT_WHITE),
                    );
                });
            });

            // Sort controls at top right
            let top_right =
                Pos2::new(ui.max_rect().right() - 10.0, ui.max_rect().top() + 10.0);
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
        Vec2::new(
            dropdown_width,
            PILL_HEIGHT * SortOption::all().len() as f32,
        ),
    );

    let mut item_states: Vec<(Rect, bool, bool, SortOption)> = Vec::new();

    if app.sort_dropdown_open {
        for (i, option) in SortOption::all().iter().enumerate() {
            let item_rect = Rect::from_min_size(
                Pos2::new(
                    menu_rect.left(),
                    menu_rect.top() + i as f32 * PILL_HEIGHT,
                ),
                Vec2::new(dropdown_width, PILL_HEIGHT),
            );
            let item_resp = ui.allocate_rect(item_rect, egui::Sense::click());
            let hovered = item_resp.hovered();
            let clicked = item_resp.clicked();
            if hovered {
                any_item_hovered = true;
            }
            if clicked {
                clicked_option = Some(*option);
            }
            item_states.push((item_rect, hovered, clicked, *option));
        }
    }

    // Now do all painting (immutable borrows via painter)
    let painter = ui.painter();

    // "SORT BY" label
    painter.text(
        Pos2::new(left + 10.0, top + PILL_HEIGHT / 2.0),
        egui::Align2::LEFT_CENTER,
        "SORT BY",
        poppins_sm(),
        TEXT_WHITE,
    );

    // Dropdown pill background
    painter.rect_filled(dropdown_rect, CornerRadius::same(PILL_RADIUS), CARD_BG);

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

    // Dropdown menu painting
    if app.sort_dropdown_open {
        painter.rect_filled(menu_rect, CornerRadius::same(12), CARD_BG);
        painter.rect_stroke(
            menu_rect,
            CornerRadius::same(12),
            Stroke::new(1.0, Color32::from_rgba_premultiplied(255, 255, 255, 30)),
            StrokeKind::Outside,
        );

        for (item_rect, hovered, _clicked, option) in &item_states {
            if *hovered {
                painter.rect_filled(*item_rect, CornerRadius::same(8), CARD_BG_HOVER);
            }
            if *option == app.sort_by {
                painter.rect_filled(
                    *item_rect,
                    CornerRadius::same(8),
                    Color32::from_rgba_premultiplied(255, 255, 255, 15),
                );
            }
            painter.text(
                Pos2::new(item_rect.left() + 22.0, item_rect.center().y),
                egui::Align2::LEFT_CENTER,
                option.label(),
                poppins_sm(),
                TEXT_WHITE,
            );
        }
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
