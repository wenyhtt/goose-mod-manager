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
    pub selected_mod_url: Option<String>,
    pub detail_image_offset: usize,
    pub details_just_opened: bool,
    scrape_task: Option<JoinHandle<Result<(usize, Vec<ModEntry>), String>>>,
    detail_task: Option<JoinHandle<(String, Result<crate::scraper::ModDetails, String>)>>,
    failed_detail_url: Option<String>,
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
            selected_mod_url: None,
            detail_image_offset: 0,
            details_just_opened: false,
            scrape_task,
            detail_task: None,
            failed_detail_url: None,
            next_scrape_page: 2,
            has_more_scrape_pages: true,
        }
    }
}

impl GooseModManager {
    fn load_mods() -> Vec<ModEntry> {
        let cached_path = crate::scraper::cache_dir().join("mods.json");
        let legacy_path = format!("{}/mods.json", env!("CARGO_MANIFEST_DIR"));
        if let Ok(data) = std::fs::read_to_string(cached_path)
            .or_else(|_| std::fs::read_to_string(legacy_path))
        {
            let mut parsed: Vec<ModEntry> = serde_json::from_str(&data).unwrap_or_default();
            for m in parsed.iter_mut() {
                if let Some(ref path) = m.thumbnail_path {
                    if let Ok(bytes) = std::fs::read(path) {
                        m.image_bytes = Some(egui::load::Bytes::Shared(bytes.into()));
                    }
                }
                m.detail_image_bytes = m
                    .detail_image_paths
                    .iter()
                    .filter_map(|path| std::fs::read(path).ok())
                    .map(|bytes| egui::load::Bytes::Shared(bytes.into()))
                    .collect();
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

    fn refresh_detail_fetch(&mut self) {
        let Some(task) = self.detail_task.take() else {
            return;
        };
        if !task.is_finished() {
            self.detail_task = Some(task);
            return;
        }

        match task.join() {
            Ok((url, Ok(details))) => {
                if let Some(mod_entry) = self.mods.iter_mut().find(|mod_entry| mod_entry.url == url)
                {
                    if !details.likes.is_empty() {
                        mod_entry.likes = details.likes;
                    }
                    if !details.downloads.is_empty() {
                        mod_entry.downloads = details.downloads;
                    }
                    if !details.image_paths.is_empty() {
                        mod_entry.detail_image_paths = details.image_paths;
                    }
                    mod_entry.detail_image_bytes = mod_entry
                        .detail_image_paths
                        .iter()
                        .filter_map(|path| std::fs::read(path).ok())
                        .map(|bytes| egui::load::Bytes::Shared(bytes.into()))
                        .collect();
                }
                if let Err(error) = crate::scraper::save_mods(&self.mods) {
                    eprintln!("Saving mod details failed: {error}");
                }
            }
            Ok((url, Err(error))) => {
                self.failed_detail_url = Some(url);
                eprintln!("Fetching mod details failed: {error}");
            }
            Err(_) => eprintln!("Detail fetch thread panicked"),
        }
    }

    fn fetch_selected_details_if_needed(&mut self) {
        let Some(url) = self.selected_mod_url.clone() else {
            return;
        };
        if self.detail_task.is_some() || self.failed_detail_url.as_ref() == Some(&url) {
            return;
        }
        let needs_details = self
            .mods
            .iter()
            .find(|mod_entry| mod_entry.url == url)
            .is_some_and(|mod_entry| {
                mod_entry.likes.is_empty() || mod_entry.detail_image_paths.is_empty()
            });
        if needs_details {
            self.detail_task = Some(std::thread::spawn(move || {
                let result = crate::scraper::run_details(&url).map_err(|error| error.to_string());
                (url, result)
            }));
        }
    }

    pub fn open_details(&mut self, index: usize) {
        if let Some(mod_entry) = self.mods.get(index) {
            self.selected_mod_url = Some(mod_entry.url.clone());
            self.detail_image_offset = 0;
            self.details_just_opened = true;
        }
    }

    pub fn close_details(&mut self) {
        self.selected_mod_url = None;
        self.detail_image_offset = 0;
    }

    pub fn selected_mod(&self) -> Option<&ModEntry> {
        let url = self.selected_mod_url.as_ref()?;
        self.mods.iter().find(|mod_entry| &mod_entry.url == url)
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
        self.refresh_detail_fetch();
        self.fetch_selected_details_if_needed();
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

                let details_open = self.selected_mod_url.is_some();
                if details_open
                    && ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape))
                {
                    self.close_details();
                }

                // ── KEYBOARD / GAMEPAD ARROW NAVIGATION ───────────────────────────
                if !details_open {
                    if ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp)) {
                        if ui.ctx().memory(|mem| mem.focused().is_some()) {
                            ui.ctx()
                                .memory_mut(|mem| mem.move_focus(egui::FocusDirection::Up));
                        }
                    }
                    if ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown))
                    {
                        if ui.ctx().memory(|mem| mem.focused().is_some()) {
                            ui.ctx()
                                .memory_mut(|mem| mem.move_focus(egui::FocusDirection::Down));
                        }
                    }
                    if ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowLeft))
                    {
                        if ui.ctx().memory(|mem| mem.focused().is_some()) {
                            ui.ctx()
                                .memory_mut(|mem| mem.move_focus(egui::FocusDirection::Left));
                        }
                    }
                    if ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowRight))
                    {
                        if ui.ctx().memory(|mem| mem.focused().is_some()) {
                            ui.ctx()
                                .memory_mut(|mem| mem.move_focus(egui::FocusDirection::Right));
                        }
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

                ui.add_enabled_ui(!details_open, |ui| {
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

                ui::details::render_details(self, ui.ctx());
            });
    }
}
