use crate::models::{ModEntry, SortOption, Tab};
use crate::theme::{BG_DARK, TEXT_WHITE};
use crate::ui;
use eframe::egui::{self, Frame, Margin, Vec2};

pub struct GooseModManager {
    pub active_tab: Tab,
    pub current_page: usize,
    pub mods: Vec<ModEntry>,
    pub sort_by: SortOption,
    pub sort_dropdown_open: bool,
    pub cols: usize,
    pub rows: usize,
    pub needs_initial_focus: bool,
    pub gilrs: gilrs::Gilrs,
}

impl Default for GooseModManager {
    fn default() -> Self {
        let mut mods = if let Ok(data) = std::fs::read_to_string("mods.json") {
            let mut parsed: Vec<ModEntry> = serde_json::from_str(&data).unwrap_or_default();
            for m in parsed.iter_mut() {
                if let Some(ref path) = m.thumbnail_path {
                    if let Ok(bytes) = std::fs::read(path) {
                        m.image_bytes = Some(egui::load::Bytes::Shared(bytes.into()));
                    }
                }
            }
            parsed
        } else {
            Vec::new() // Fallback if no database exists
        };

        Self {
            active_tab: Tab::Browse,
            current_page: 0,
            mods,
            sort_by: SortOption::Vehicle,
            sort_dropdown_open: false,
            cols: 4,
            rows: 2,
            needs_initial_focus: true,
            gilrs: gilrs::Gilrs::new().unwrap(),
        }
    }
}

impl GooseModManager {
    pub fn total_pages(&self) -> usize {
        let items_per_page = (self.cols * self.rows).max(1);
        (self.mods.len() + items_per_page - 1) / items_per_page
    }

    pub fn page_mods(&self) -> &[ModEntry] {
        let items_per_page = (self.cols * self.rows).max(1);
        let start = self.current_page * items_per_page;
        let end = (start + items_per_page).min(self.mods.len());
        &self.mods[start..end]
    }
}

impl eframe::App for GooseModManager {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        
        // ── GAMEPAD INPUT HANDLING ─────────────────────────────────────────
        while let Some(gilrs::Event { event, .. }) = self.gilrs.next_event() {
            match event {
                gilrs::EventType::ButtonPressed(gilrs::Button::DPadUp, _) => {
                    if ctx.memory(|mem| mem.focused().is_none()) {
                        self.needs_initial_focus = true;
                    } else {
                        ctx.memory_mut(|mem| mem.move_focus(egui::FocusDirection::Up));
                    }
                }
                gilrs::EventType::ButtonPressed(gilrs::Button::DPadDown, _) => {
                    if ctx.memory(|mem| mem.focused().is_none()) {
                        self.needs_initial_focus = true;
                    } else {
                        ctx.memory_mut(|mem| mem.move_focus(egui::FocusDirection::Down));
                    }
                }
                gilrs::EventType::ButtonPressed(gilrs::Button::DPadLeft, _) => {
                    if ctx.memory(|mem| mem.focused().is_none()) {
                        self.needs_initial_focus = true;
                    } else {
                        ctx.memory_mut(|mem| mem.move_focus(egui::FocusDirection::Left));
                    }
                }
                gilrs::EventType::ButtonPressed(gilrs::Button::DPadRight, _) => {
                    if ctx.memory(|mem| mem.focused().is_none()) {
                        self.needs_initial_focus = true;
                    } else {
                        ctx.memory_mut(|mem| mem.move_focus(egui::FocusDirection::Right));
                    }
                }
                gilrs::EventType::ButtonPressed(gilrs::Button::South, _) => {
                    ctx.input_mut(|i| i.events.push(egui::Event::Key {
                        key: egui::Key::Enter,
                        physical_key: None,
                        pressed: true,
                        repeat: false,
                        modifiers: egui::Modifiers::NONE,
                    }));
                }
                gilrs::EventType::ButtonReleased(gilrs::Button::South, _) => {
                    ctx.input_mut(|i| i.events.push(egui::Event::Key {
                        key: egui::Key::Enter,
                        physical_key: None,
                        pressed: false,
                        repeat: false,
                        modifiers: egui::Modifiers::NONE,
                    }));
                }
                gilrs::EventType::ButtonPressed(gilrs::Button::East, _) => {
                    ctx.input_mut(|i| i.events.push(egui::Event::Key {
                        key: egui::Key::Escape,
                        physical_key: None,
                        pressed: true,
                        repeat: false,
                        modifiers: egui::Modifiers::NONE,
                    }));
                }
                gilrs::EventType::ButtonReleased(gilrs::Button::East, _) => {
                    ctx.input_mut(|i| i.events.push(egui::Event::Key {
                        key: egui::Key::Escape,
                        physical_key: None,
                        pressed: false,
                        repeat: false,
                        modifiers: egui::Modifiers::NONE,
                    }));
                }
                _ => {}
            }
        }
        
        ctx.request_repaint(); // Keep polling inputs

        // Apply dark style
        ctx.all_styles_mut(|style| {
            style.visuals.override_text_color = Some(TEXT_WHITE);
            style.visuals.window_fill = BG_DARK;
            style.visuals.panel_fill = BG_DARK;
            style.visuals.selection.stroke = eframe::egui::Stroke::NONE; // Disable default focus ring
        });

        egui::CentralPanel::default()
            .frame(Frame {
                fill: BG_DARK,
                inner_margin: Margin::same(10),
                ..Default::default()
            })
            .show(ui, |ui| {
                // Initial focus is now handled directly inside top_bar.rs where the widget is rendered
                let arrow_pressed = ui.input(|i| {
                    i.key_pressed(egui::Key::ArrowUp)
                        || i.key_pressed(egui::Key::ArrowDown)
                        || i.key_pressed(egui::Key::ArrowLeft)
                        || i.key_pressed(egui::Key::ArrowRight)
                });
                if arrow_pressed && ui.ctx().memory(|mem| mem.focused().is_none()) {
                    self.needs_initial_focus = true;
                }

                // ── KEYBOARD / GAMEPAD ARROW NAVIGATION ───────────────────────────
                if ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp)) {
                    if ui.ctx().memory(|mem| mem.focused().is_some()) {
                        ui.ctx().memory_mut(|mem| mem.move_focus(egui::FocusDirection::Up));
                    }
                }
                if ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown)) {
                    if ui.ctx().memory(|mem| mem.focused().is_some()) {
                        ui.ctx().memory_mut(|mem| mem.move_focus(egui::FocusDirection::Down));
                    }
                }
                if ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowLeft)) {
                    if ui.ctx().memory(|mem| mem.focused().is_some()) {
                        ui.ctx().memory_mut(|mem| mem.move_focus(egui::FocusDirection::Left));
                    }
                }
                if ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowRight)) {
                    if ui.ctx().memory(|mem| mem.focused().is_some()) {
                        ui.ctx().memory_mut(|mem| mem.move_focus(egui::FocusDirection::Right));
                    }
                }
                ui.spacing_mut().item_spacing = Vec2::new(10.0, 10.0);

                // Dynamically compute responsive grid sizing
                let available = ui.available_size();
                let padding = 10.0;
                let gap = 10.0;
                // Minimum card width/height
                let min_w = 260.0;
                let min_h = 240.0;
                let content_w = (available.x - padding * 2.0).max(0.0);
                let content_h = (available.y - padding * 2.0 - 130.0).max(0.0); // 130 is approx top_bar height

                self.cols = ((content_w + gap) / (min_w + gap)).floor().max(1.0) as usize;
                self.rows = ((content_h + gap) / (min_h + gap)).floor().max(1.0) as usize;

                // ── TOP BAR ──────────────────────────────────────────
                ui::top_bar::render_top_bar(self, ui);

                ui.add_space(4.0);

                // ── GRID OR EMPTY STATE ─────────────────────────────────────────────
                if self.sort_by == SortOption::Vehicle {
                    ui::grid::render_grid(self, ui);
                } else {
                    ui.centered_and_justified(|ui| {
                        ui.label(
                            egui::RichText::new("Oops.. we ain't ready yet")
                                .font(egui::FontId::new(
                                    36.0,
                                    egui::FontFamily::Name("Poppins".into()),
                                ))
                                .color(TEXT_WHITE),
                        );
                    });
                }
            });
    }
}
