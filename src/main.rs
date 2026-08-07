use eframe::egui::{FontData, FontDefinitions, FontFamily};
use std::sync::Arc;

mod app;
mod icons;
mod models;
mod scraper;
mod theme;
mod ui;

use app::GooseModManager;

// ── Entry point ──────────────────────────────────────────────────────────────
fn main() -> eframe::Result<()> {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([852.0, 480.0])
            .with_min_inner_size([852.0, 480.0])
            .with_title("Goose Mod Manager"),
        ..Default::default()
    };

    eframe::run_native(
        "Goose Mod Manager",
        options,
        Box::new(|cc| {
            let mut fonts = FontDefinitions::default();

            fonts.font_data.insert(
                "Poppins-Medium".to_owned(),
                Arc::new(FontData::from_static(include_bytes!(
                    "../assets/fonts/Poppins-Medium.ttf"
                ))),
            );

            fonts.font_data.insert(
                "Poppins-Regular".to_owned(),
                Arc::new(FontData::from_static(include_bytes!(
                    "../assets/fonts/Poppins-Regular.ttf"
                ))),
            );

            fonts.families.insert(
                FontFamily::Name("Poppins".into()),
                vec!["Poppins-Medium".to_owned(), "Poppins-Regular".to_owned()],
            );

            fonts
                .families
                .get_mut(&FontFamily::Proportional)
                .unwrap()
                .insert(0, "Poppins-Regular".to_owned());

            cc.egui_ctx.set_fonts(fonts);

            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::new(GooseModManager::default()))
        }),
    )
}
