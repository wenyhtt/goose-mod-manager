use eframe::egui::{self, Frame, Margin, TextureHandle, Vec2};
use crate::models::{ModEntry, SortOption, Tab};
use crate::theme::{BG_DARK, ITEMS_PER_PAGE, TEXT_WHITE};
use crate::ui;

pub struct GooseModManager {
    pub active_tab: Tab,
    pub current_page: usize,
    pub mods: Vec<ModEntry>,
    pub card_texture: Option<TextureHandle>,
    pub sort_by: SortOption,
    pub sort_dropdown_open: bool,
}

impl Default for GooseModManager {
    fn default() -> Self {
        let mods = vec![
            ModEntry::new("Subaru Impreza WRX STI...", "Vehicle", "20.4MB"),
            ModEntry::new("Toyota Supra MK4 RZ...", "Vehicle", "18.7MB"),
            ModEntry::new("Nissan Skyline R34 GT-R...", "Vehicle", "22.1MB"),
            ModEntry::new("Mazda RX-7 FD Spirit R...", "Vehicle", "19.3MB"),
            ModEntry::new("Honda NSX Type R...", "Vehicle", "17.8MB"),
            ModEntry::new("Mitsubishi Lancer Evo IX...", "Vehicle", "21.5MB"),
            ModEntry::new("BMW M3 E46 GTR...", "Vehicle", "23.0MB"),
            ModEntry::new("Ford Mustang GT500...", "Vehicle", "24.2MB"),
            ModEntry::new("Porsche 911 GT3 RS...", "Vehicle", "25.1MB"),
            ModEntry::new("Lamborghini Murcielago...", "Vehicle", "26.8MB"),
            ModEntry::new("Ferrari F40 Competizione...", "Vehicle", "19.9MB"),
            ModEntry::new("Chevrolet Corvette C6...", "Vehicle", "20.0MB"),
            ModEntry::new("Dodge Viper SRT-10...", "Vehicle", "22.4MB"),
            ModEntry::new("Audi R8 V10 Plus...", "Vehicle", "21.7MB"),
            ModEntry::new("McLaren F1 LM...", "Vehicle", "18.2MB"),
            ModEntry::new("Aston Martin DB9...", "Vehicle", "20.6MB"),
            ModEntry::new("Pagani Zonda R...", "Vehicle", "23.5MB"),
            ModEntry::new("Koenigsegg CCX...", "Vehicle", "24.8MB"),
            ModEntry::new("Bugatti Veyron SS...", "Vehicle", "27.3MB"),
            ModEntry::new("Lexus LFA Nürburgring...", "Vehicle", "21.0MB"),
            ModEntry::new("Jaguar XJ220...", "Vehicle", "18.9MB"),
            ModEntry::new("Mercedes SLR McLaren...", "Vehicle", "22.8MB"),
            ModEntry::new("Alfa Romeo 8C...", "Vehicle", "19.5MB"),
            ModEntry::new("Maserati MC12...", "Vehicle", "23.1MB"),
        ];

        Self {
            active_tab: Tab::Browse,
            current_page: 0,
            mods,
            card_texture: None,
            sort_by: SortOption::Vehicle,
            sort_dropdown_open: false,
        }
    }
}

impl GooseModManager {
    pub fn total_pages(&self) -> usize {
        (self.mods.len() + ITEMS_PER_PAGE - 1) / ITEMS_PER_PAGE
    }

    pub fn page_mods(&self) -> &[ModEntry] {
        let start = self.current_page * ITEMS_PER_PAGE;
        let end = (start + ITEMS_PER_PAGE).min(self.mods.len());
        &self.mods[start..end]
    }

    pub fn load_card_texture(&mut self, ctx: &egui::Context) {
        if self.card_texture.is_none() {
            let image_data = include_bytes!("../assets/card_image.png");
            let img = image::load_from_memory(image_data).expect("Failed to load card_image.png");
            let rgba = img.to_rgba8();
            let size = [rgba.width() as usize, rgba.height() as usize];
            let pixels = rgba.into_raw();
            let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
            self.card_texture =
                Some(ctx.load_texture("card_image", color_image, Default::default()));
        }
    }
}

impl eframe::App for GooseModManager {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        self.load_card_texture(&ctx);

        // Apply dark style
        ctx.all_styles_mut(|style| {
            style.visuals.override_text_color = Some(TEXT_WHITE);
            style.visuals.window_fill = BG_DARK;
            style.visuals.panel_fill = BG_DARK;
        });

        egui::CentralPanel::default()
            .frame(Frame {
                fill: BG_DARK,
                inner_margin: Margin::same(10),
                ..Default::default()
            })
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing = Vec2::new(10.0, 10.0);

                // ── TOP BAR ──────────────────────────────────────────
                ui::top_bar::render_top_bar(self, ui);

                ui.add_space(4.0);

                // ── GRID ─────────────────────────────────────────────
                ui::grid::render_grid(self, ui);
            });
    }
}
