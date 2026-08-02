use crate::models::{ModEntry, SortOption, Tab};
use crate::theme::{BG_DARK, TEXT_WHITE};
use crate::ui;
use eframe::egui::{self, Frame, Margin, Vec2};
use std::thread::JoinHandle;

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
    scrape_task: Option<JoinHandle<Result<(usize, Vec<ModEntry>), String>>>,
    next_scrape_page: usize,
    has_more_scrape_pages: bool,
}

impl Default for GooseModManager {
    fn default() -> Self {
        let mods = Self::load_mods();
        let scrape_task = Some(std::thread::spawn(|| {
            crate::scraper::run_page(1)
                .map_err(|error| error.to_string())
                .map(|mods| (1, mods))
        }));

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
            scrape_task,
            next_scrape_page: 2,
            has_more_scrape_pages: true,
        }
    }
}

impl GooseModManager {
    fn load_mods() -> Vec<ModEntry> {
        let json_path = format!("{}/mods.json", env!("CARGO_MANIFEST_DIR"));
        if let Ok(data) = std::fs::read_to_string(json_path) {
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
            Vec::new()
        }
    }

    fn refresh_after_scrape(&mut self) {
        let Some(task) = self.scrape_task.take() else { return };
        if task.is_finished() {
            match task.join() {
                Ok(Ok((page, mods))) => {
                    let found_mods = !mods.is_empty();
                    if page == 1 {
                        self.mods = mods;
                        self.current_page = 0;
                    } else {
                        let existing = self
                            .mods
                            .iter()
                            .map(|m| m.url.clone())
                            .collect::<std::collections::HashSet<_>>();
                        self.mods
                            .extend(mods.into_iter().filter(|m| !existing.contains(&m.url)));
                    }
                    self.next_scrape_page = page + 1;
                    self.has_more_scrape_pages = found_mods;
                    if let Err(error) = crate::scraper::save_mods(&self.mods) {
                        eprintln!("Saving scraped mods failed: {error}");
                    } else {
                        self.mods = Self::load_mods();
                    }
                }
                Ok(Err(error)) => {
                    self.has_more_scrape_pages = false;
                    eprintln!("Scraping failed: {error}");
                }
                Err(_) => {
                    self.has_more_scrape_pages = false;
                    eprintln!("Scraping thread panicked");
                }
            }
        } else {
            self.scrape_task = Some(task);
        }
    }

    fn refresh_scrape_if_needed(&mut self) {
        if self.scrape_task.is_some()
            || !self.has_more_scrape_pages
            || self.current_page + 1 >= self.total_pages()
            || self.current_page + 3 < self.total_pages()
        {
            return;
        }

        let page = self.next_scrape_page;
        self.scrape_task = Some(std::thread::spawn(move || {
            crate::scraper::run_page(page)
                .map(|mods| (page, mods))
                .map_err(|error| error.to_string())
        }));
    }

    pub fn total_pages(&self) -> usize {
        let items_per_page = (self.cols * self.rows).max(1);
        let loaded_pages = (self.mods.len() + items_per_page - 1) / items_per_page;
        loaded_pages + usize::from(self.scrape_task.is_some())
    }

    pub fn is_loading(&self) -> bool {
        self.scrape_task.is_some()
    }

    pub fn page_mods(&self) -> &[ModEntry] {
        let items_per_page = (self.cols * self.rows).max(1);
        let loaded_pages = (self.mods.len() + items_per_page - 1) / items_per_page;
        if self.current_page >= loaded_pages {
            return &[];
        }
        let start = self.current_page * items_per_page;
        let end = (start + items_per_page).min(self.mods.len());
        &self.mods[start..end]
    }
}

impl eframe::App for GooseModManager {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.refresh_after_scrape();
        self.refresh_scrape_if_needed();
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
                self.current_page = self.current_page.min(self.total_pages().saturating_sub(1));

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
