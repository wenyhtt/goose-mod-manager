use crate::models::{ModEntry, ModVersion, SortOption, Tab};
use crate::theme::{BG_DARK, FONT_EMPTY, TEXT_WHITE, noto_sans_bold};
use crate::ui;
use eframe::egui::{self, Frame, Margin, Vec2};
use std::thread::JoinHandle;

// Input module imported in main.rs

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
    pub versions_dialog_open: bool,
    pub versions_dialog_just_opened: bool,
    pub versions: Vec<ModVersion>,
    pub versions_error: Option<String>,
    pub download_status: Option<String>,
    scrape_task: Option<JoinHandle<Result<(usize, Vec<ModEntry>), String>>>,
    detail_task: Option<JoinHandle<(String, Result<crate::scraper::ModDetails, String>)>>,
    version_task: Option<JoinHandle<Result<Vec<ModVersion>, String>>>,
    download_task: Option<JoinHandle<Result<std::path::PathBuf, String>>>,
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
            versions_dialog_open: false,
            versions_dialog_just_opened: false,
            versions: Vec::new(),
            versions_error: None,
            download_status: None,
            scrape_task,
            detail_task: None,
            version_task: None,
            download_task: None,
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
        if let Ok(data) =
            std::fs::read_to_string(cached_path).or_else(|_| std::fs::read_to_string(legacy_path))
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
        let Some(task) = self.scrape_task.take() else {
            return;
        };
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

    fn refresh_versions_fetch(&mut self) {
        let Some(task) = self.version_task.take() else {
            return;
        };
        if !task.is_finished() {
            self.version_task = Some(task);
            return;
        }
        match task.join() {
            Ok(Ok(versions)) if versions.is_empty() => {
                self.versions_error = Some("No downloadable versions found.".to_string());
            }
            Ok(Ok(versions)) => self.versions = versions,
            Ok(Err(error)) => self.versions_error = Some(error),
            Err(_) => self.versions_error = Some("Fetching versions failed.".to_string()),
        }
    }

    fn refresh_version_download(&mut self) {
        let Some(task) = self.download_task.take() else {
            return;
        };
        if !task.is_finished() {
            self.download_task = Some(task);
            return;
        }
        self.download_status = Some(match task.join() {
            Ok(Ok(path)) => {
                eprintln!("Download task saved {}", path.display());
                format!("Saved to {}", path.display())
            }
            Ok(Err(error)) => {
                eprintln!("Download task failed: {error}");
                format!("Download failed: {error}")
            }
            Err(_) => {
                eprintln!("Download task panicked");
                "Download failed.".to_string()
            }
        });
    }

    pub fn open_versions_dialog(&mut self) {
        let Some(url) = self.selected_mod_url.clone() else {
            return;
        };
        self.versions_dialog_open = true;
        self.versions_dialog_just_opened = true;
        self.versions.clear();
        self.versions_error = None;
        self.download_status = None;
        self.version_task = Some(std::thread::spawn(move || {
            crate::scraper::fetch_versions(&url).map_err(|error| error.to_string())
        }));
    }

    pub fn close_versions_dialog(&mut self) {
        self.versions_dialog_open = false;
        self.versions_dialog_just_opened = false;
    }

    pub fn start_version_download(&mut self, version: ModVersion) {
        if self.download_task.is_some() {
            return;
        }
        eprintln!("Starting download task: {}", version.label);
        self.download_status = Some(format!("Downloading {}...", version.label));
        self.download_task = Some(std::thread::spawn(move || {
            crate::scraper::download_version(&version).map_err(|error| error.to_string())
        }));
    }

    pub fn versions_loading(&self) -> bool {
        self.version_task.is_some()
    }

    pub fn version_downloading(&self) -> bool {
        self.download_task.is_some()
    }

    pub fn open_details(&mut self, index: usize) {
        if let Some(mod_entry) = self.mods.get(index) {
            self.selected_mod_url = Some(mod_entry.url.clone());
            self.detail_image_offset = 0;
            self.details_just_opened = true;
        }
    }

    pub fn close_details(&mut self) {
        self.close_versions_dialog();
        self.selected_mod_url = None;
        self.detail_image_offset = 0;
    }

    pub fn carousel_prev(&mut self) {
        let count = self.selected_mod_preview_count();
        self.detail_image_offset = previous_carousel_offset(self.detail_image_offset, count);
    }

    pub fn carousel_next(&mut self) {
        let count = self.selected_mod_preview_count();
        self.detail_image_offset = next_carousel_offset(self.detail_image_offset, count);
    }

    fn selected_mod_preview_count(&self) -> usize {
        self.selected_mod()
            .map(|mod_entry| {
                if mod_entry.detail_image_bytes.is_empty() {
                    usize::from(mod_entry.image_bytes.is_some())
                } else {
                    mod_entry.detail_image_bytes.len()
                }
            })
            .unwrap_or(0)
    }

    fn details_focus_is(ctx: &egui::Context, id: &str) -> bool {
        ctx.memory(|mem| mem.focused() == Some(egui::Id::new(id)))
    }

    fn details_focus(ctx: &egui::Context, id: &str) {
        ctx.memory_mut(|mem| {
            mem.request_focus(egui::Id::new(id));
            mem.move_focus(egui::FocusDirection::None);
        });
    }

    pub(crate) fn details_up(ctx: &egui::Context) {
        Self::details_focus(ctx, ui::details::DETAIL_IMAGE_CURRENT_ID);
    }

    pub(crate) fn details_down(&self, ctx: &egui::Context) {
        if Self::details_focus_is(ctx, ui::details::DETAIL_VIEW_WEB_ID)
            && self.selected_mod_preview_count() > 1
        {
            Self::details_focus(ctx, ui::details::DETAIL_ARROW_LEFT_ID);
        } else {
            Self::details_focus(ctx, ui::details::DETAIL_VIEW_WEB_ID);
        }
    }

    pub(crate) fn details_left(&mut self, ctx: &egui::Context) {
        if Self::details_focus_is(ctx, ui::details::DETAIL_IMAGE_CURRENT_ID) {
            self.carousel_prev();
        } else if Self::details_focus_is(ctx, ui::details::DETAIL_ARROW_RIGHT_ID) {
            Self::details_focus(ctx, ui::details::DETAIL_ARROW_LEFT_ID);
        } else if Self::details_focus_is(ctx, ui::details::DETAIL_ARROW_LEFT_ID) {
            Self::details_focus(ctx, ui::details::DETAIL_VIEW_WEB_ID);
        } else {
            Self::details_focus(ctx, ui::details::DETAIL_IMAGE_CURRENT_ID);
        }
    }

    pub(crate) fn details_right(&mut self, ctx: &egui::Context) {
        if Self::details_focus_is(ctx, ui::details::DETAIL_IMAGE_CURRENT_ID)
            || Self::details_focus_is(ctx, ui::details::DETAIL_IMAGE_NEXT_ID)
            || Self::details_focus_is(ctx, ui::details::DETAIL_ARROW_RIGHT_ID)
        {
            self.carousel_next();
            Self::details_focus(ctx, ui::details::DETAIL_IMAGE_CURRENT_ID);
        } else if Self::details_focus_is(ctx, ui::details::DETAIL_VIEW_WEB_ID)
            && self.selected_mod_preview_count() > 1
        {
            Self::details_focus(ctx, ui::details::DETAIL_ARROW_LEFT_ID);
        } else if Self::details_focus_is(ctx, ui::details::DETAIL_ARROW_LEFT_ID) {
            Self::details_focus(ctx, ui::details::DETAIL_ARROW_RIGHT_ID);
        } else {
            Self::details_focus(ctx, ui::details::DETAIL_IMAGE_CURRENT_ID);
        }
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

fn previous_carousel_offset(offset: usize, count: usize) -> usize {
    if count > 1 {
        (offset + count - 1) % count
    } else {
        offset
    }
}

fn next_carousel_offset(offset: usize, count: usize) -> usize {
    if count > 1 {
        (offset + 1) % count
    } else {
        offset
    }
}

impl eframe::App for GooseModManager {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.refresh_after_scrape();
        self.refresh_scrape_if_needed();
        self.refresh_detail_fetch();
        self.fetch_selected_details_if_needed();
        self.refresh_versions_fetch();
        self.refresh_version_download();
        let ctx = ui.ctx().clone();

        // ── GAMEPAD INPUT HANDLING ─────────────────────────────────────────
        crate::input::handle_gamepad(self, &ctx);

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
                if !self.selected_mod_url.is_some()
                    && arrow_pressed
                    && ui.ctx().memory(|mem| mem.focused().is_none())
                {
                    self.needs_initial_focus = true;
                }

                let details_open = self.selected_mod_url.is_some();

                // ── KEYBOARD / GAMEPAD ARROW NAVIGATION ───────────────────────────
                crate::input::handle_keyboard(self, ui);
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
                                    .font(noto_sans_bold(FONT_EMPTY))
                                    .color(TEXT_WHITE),
                            );
                        });
                    }
                });

                ui::details::render_details(self, ui.ctx());
                ui::versions::render_versions(self, ui.ctx());
            });
    }
}

#[cfg(test)]
mod tests {
    use super::{next_carousel_offset, previous_carousel_offset};

    #[test]
    fn carousel_offsets_wrap_and_ignore_single_image() {
        assert_eq!(previous_carousel_offset(0, 3), 2);
        assert_eq!(next_carousel_offset(2, 3), 0);
        assert_eq!(previous_carousel_offset(0, 1), 0);
        assert_eq!(next_carousel_offset(0, 0), 0);
    }
}
