use eframe::egui::{self, Frame, Margin, Vec2};
use crate::models::{ModEntry, SortOption, Tab};
use crate::theme::{BG_DARK, TEXT_WHITE};
use crate::ui;

pub struct GooseModManager {
    pub active_tab: Tab,
    pub current_page: usize,
    pub mods: Vec<ModEntry>,
    pub sort_by: SortOption,
    pub sort_dropdown_open: bool,
    pub cols: usize,
    pub rows: usize,
    pub needs_initial_focus: bool,
}

impl Default for GooseModManager {
    fn default() -> Self {
        let mods = vec![
            ModEntry::new("Subaru Impreza WRX STI 2004", "Vehicle", "20.4MB"),
            ModEntry::new("Toyota Supra MK4 RZ 1997", "Vehicle", "18.7MB"),
            ModEntry::new("Nissan Skyline R34 GT-R V-Spec II", "Vehicle", "22.1MB"),
            ModEntry::new("Mazda RX-7 FD Spirit R Type A", "Vehicle", "19.3MB"),
            ModEntry::new("Honda NSX Type R 2002", "Vehicle", "17.8MB"),
            ModEntry::new("Mitsubishi Lancer Evolution IX MR", "Vehicle", "21.5MB"),
            ModEntry::new("BMW M3 E46 GTR 2001", "Vehicle", "23.0MB"),
            ModEntry::new("Ford Mustang Shelby GT500", "Vehicle", "24.2MB"),
            ModEntry::new("Porsche 911 GT3 RS 991.2", "Vehicle", "25.1MB"),
            ModEntry::new("Lamborghini Murcielago LP670-4 SV", "Vehicle", "26.8MB"),
            ModEntry::new("Ferrari F40 Competizione", "Vehicle", "19.9MB"),
            ModEntry::new("Chevrolet Corvette C6 ZR1", "Vehicle", "20.0MB"),
            ModEntry::new("Dodge Viper SRT-10 ACR", "Vehicle", "22.4MB"),
            ModEntry::new("Audi R8 V10 Plus 2016", "Vehicle", "21.7MB"),
            ModEntry::new("McLaren F1 LM 1995", "Vehicle", "18.2MB"),
            ModEntry::new("Aston Martin DB9 Volante", "Vehicle", "20.6MB"),
            ModEntry::new("Pagani Zonda R 2009", "Vehicle", "23.5MB"),
            ModEntry::new("Koenigsegg CCX 2006", "Vehicle", "24.8MB"),
            ModEntry::new("Bugatti Veyron Super Sport", "Vehicle", "27.3MB"),
            ModEntry::new("Lexus LFA Nürburgring Package", "Vehicle", "21.0MB"),
            ModEntry::new("Jaguar XJ220 1992", "Vehicle", "18.9MB"),
            ModEntry::new("Mercedes-Benz SLR McLaren", "Vehicle", "22.8MB"),
            ModEntry::new("Alfa Romeo 8C Competizione", "Vehicle", "19.5MB"),
            ModEntry::new("Maserati MC12 Stradale", "Vehicle", "23.1MB"),
        ];

        Self {
            active_tab: Tab::Browse,
            current_page: 0,
            mods,
            sort_by: SortOption::Vehicle,
            sort_dropdown_open: false,
            cols: 4,
            rows: 2,
            needs_initial_focus: true,
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
                if self.needs_initial_focus {
                    ui.ctx().memory_mut(|mem| mem.request_focus(egui::Id::new("tab_browse")));
                    self.needs_initial_focus = false;
                }

                // If an arrow key is pressed but absolutely nothing is focused, grab focus to start.
                let arrow_pressed = ui.input(|i| {
                    i.key_pressed(egui::Key::ArrowUp)
                        || i.key_pressed(egui::Key::ArrowDown)
                        || i.key_pressed(egui::Key::ArrowLeft)
                        || i.key_pressed(egui::Key::ArrowRight)
                });
                
                if arrow_pressed && ui.ctx().memory(|mem| mem.focused().is_none()) {
                    ui.ctx().memory_mut(|mem| mem.request_focus(egui::Id::new("tab_browse")));
                }

                // ── KEYBOARD / GAMEPAD ARROW NAVIGATION ───────────────────────────
                if ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp)) {
                    ui.ctx().memory_mut(|mem| mem.move_focus(egui::FocusDirection::Up));
                }
                if ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown)) {
                    ui.ctx().memory_mut(|mem| mem.move_focus(egui::FocusDirection::Down));
                }
                if ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowLeft)) {
                    ui.ctx().memory_mut(|mem| mem.move_focus(egui::FocusDirection::Left));
                }
                if ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowRight)) {
                    ui.ctx().memory_mut(|mem| mem.move_focus(egui::FocusDirection::Right));
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

                // ── GRID ─────────────────────────────────────────────
                ui::grid::render_grid(self, ui);
            });
    }
}
