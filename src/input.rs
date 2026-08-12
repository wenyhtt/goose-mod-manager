use crate::app::GooseModManager;
use eframe::egui;

pub fn consume_detail_key(input: &mut egui::InputState, key: egui::Key) -> bool {
    let initial_press = input.events.iter().any(|event| {
        matches!(
            event,
            egui::Event::Key {
                key: event_key,
                pressed: true,
                repeat: false,
                ..
            } if *event_key == key
        )
    });
    input.consume_key(egui::Modifiers::NONE, key);
    initial_press
}

pub fn handle_gamepad(app: &mut GooseModManager, ctx: &egui::Context) {
    while let Some(gilrs::Event { event, .. }) = app.gilrs.next_event() {
        if app.versions_dialog_open {
            match event {
                gilrs::EventType::ButtonPressed(gilrs::Button::DPadUp, _) => {
                    ctx.memory_mut(|mem| mem.move_focus(egui::FocusDirection::Up));
                }
                gilrs::EventType::ButtonPressed(gilrs::Button::DPadDown, _) => {
                    ctx.memory_mut(|mem| mem.move_focus(egui::FocusDirection::Down));
                }
                gilrs::EventType::ButtonPressed(gilrs::Button::DPadLeft, _) => {
                    ctx.memory_mut(|mem| mem.move_focus(egui::FocusDirection::Left));
                }
                gilrs::EventType::ButtonPressed(gilrs::Button::DPadRight, _) => {
                    ctx.memory_mut(|mem| mem.move_focus(egui::FocusDirection::Right));
                }
                gilrs::EventType::ButtonPressed(gilrs::Button::South, _) => ctx.input_mut(|i| {
                    i.events.push(egui::Event::Key {
                        key: egui::Key::Enter,
                        physical_key: None,
                        pressed: true,
                        repeat: false,
                        modifiers: egui::Modifiers::NONE,
                    })
                }),
                gilrs::EventType::ButtonPressed(gilrs::Button::East, _) => {
                    app.close_versions_dialog()
                }
                _ => {}
            }
            continue;
        }
        let details_open = app.selected_mod_url.is_some();
        match event {
            gilrs::EventType::ButtonPressed(gilrs::Button::DPadUp, _) => {
                if details_open {
                    GooseModManager::details_up(ctx);
                    continue;
                }
                if ctx.memory(|mem| mem.focused().is_none()) {
                    app.needs_initial_focus = true;
                } else {
                    ctx.memory_mut(|mem| mem.move_focus(egui::FocusDirection::Up));
                }
            }
            gilrs::EventType::ButtonPressed(gilrs::Button::DPadDown, _) => {
                if details_open {
                    app.details_down(ctx);
                    continue;
                }
                if ctx.memory(|mem| mem.focused().is_none()) {
                    app.needs_initial_focus = true;
                } else {
                    ctx.memory_mut(|mem| mem.move_focus(egui::FocusDirection::Down));
                }
            }
            gilrs::EventType::ButtonPressed(gilrs::Button::DPadLeft, _) => {
                if details_open {
                    app.details_left(ctx);
                    continue;
                }
                if ctx.memory(|mem| mem.focused().is_none()) {
                    app.needs_initial_focus = true;
                } else {
                    ctx.memory_mut(|mem| mem.move_focus(egui::FocusDirection::Left));
                }
            }
            gilrs::EventType::ButtonPressed(gilrs::Button::DPadRight, _) => {
                if details_open {
                    app.details_right(ctx);
                    continue;
                }
                if ctx.memory(|mem| mem.focused().is_none()) {
                    app.needs_initial_focus = true;
                } else {
                    ctx.memory_mut(|mem| mem.move_focus(egui::FocusDirection::Right));
                }
            }
            gilrs::EventType::ButtonPressed(gilrs::Button::South, _) => {
                ctx.input_mut(|i| {
                    i.events.push(egui::Event::Key {
                        key: egui::Key::Enter,
                        physical_key: None,
                        pressed: true,
                        repeat: false,
                        modifiers: egui::Modifiers::NONE,
                    })
                });
            }
            gilrs::EventType::ButtonReleased(gilrs::Button::South, _) => {
                ctx.input_mut(|i| {
                    i.events.push(egui::Event::Key {
                        key: egui::Key::Enter,
                        physical_key: None,
                        pressed: false,
                        repeat: false,
                        modifiers: egui::Modifiers::NONE,
                    })
                });
            }
            gilrs::EventType::ButtonPressed(gilrs::Button::East, _) => {
                ctx.input_mut(|i| {
                    i.events.push(egui::Event::Key {
                        key: egui::Key::Escape,
                        physical_key: None,
                        pressed: true,
                        repeat: false,
                        modifiers: egui::Modifiers::NONE,
                    })
                });
            }
            gilrs::EventType::ButtonReleased(gilrs::Button::East, _) => {
                ctx.input_mut(|i| {
                    i.events.push(egui::Event::Key {
                        key: egui::Key::Escape,
                        physical_key: None,
                        pressed: false,
                        repeat: false,
                        modifiers: egui::Modifiers::NONE,
                    })
                });
            }
            _ => {}
        }
    }
}

pub fn handle_keyboard(app: &mut GooseModManager, ui: &mut egui::Ui) {
    if app.versions_dialog_open {
        if ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape)) {
            app.close_versions_dialog();
        }
        if ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp)) {
            ui.ctx()
                .memory_mut(|mem| mem.move_focus(egui::FocusDirection::Up));
        }
        if ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown)) {
            ui.ctx()
                .memory_mut(|mem| mem.move_focus(egui::FocusDirection::Down));
        }
        if ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowLeft)) {
            ui.ctx()
                .memory_mut(|mem| mem.move_focus(egui::FocusDirection::Left));
        }
        if ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowRight)) {
            ui.ctx()
                .memory_mut(|mem| mem.move_focus(egui::FocusDirection::Right));
        }
        return;
    }
    let details_open = app.selected_mod_url.is_some();
    if details_open && ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape)) {
        app.close_details();
    }
    if details_open {
        if ui.input_mut(|i| consume_detail_key(i, egui::Key::ArrowUp)) {
            GooseModManager::details_up(ui.ctx());
        }
        if ui.input_mut(|i| consume_detail_key(i, egui::Key::ArrowDown)) {
            app.details_down(ui.ctx());
        }
        if ui.input_mut(|i| consume_detail_key(i, egui::Key::ArrowLeft)) {
            app.details_left(ui.ctx());
        }
        if ui.input_mut(|i| consume_detail_key(i, egui::Key::ArrowRight)) {
            app.details_right(ui.ctx());
        }
    }

    if !details_open {
        if ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp)) {
            if ui.ctx().memory(|mem| mem.focused().is_some()) {
                ui.ctx()
                    .memory_mut(|mem| mem.move_focus(egui::FocusDirection::Up));
            }
        }
        if ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown)) {
            if ui.ctx().memory(|mem| mem.focused().is_some()) {
                ui.ctx()
                    .memory_mut(|mem| mem.move_focus(egui::FocusDirection::Down));
            }
        }
        if ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowLeft)) {
            if ui.ctx().memory(|mem| mem.focused().is_some()) {
                ui.ctx()
                    .memory_mut(|mem| mem.move_focus(egui::FocusDirection::Left));
            }
        }
        if ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowRight)) {
            if ui.ctx().memory(|mem| mem.focused().is_some()) {
                ui.ctx()
                    .memory_mut(|mem| mem.move_focus(egui::FocusDirection::Right));
            }
        }
    }
}
