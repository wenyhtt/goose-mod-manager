use eframe::egui::epaint::text::VariationCoords;
use eframe::egui::{FontData, FontDefinitions, FontFamily, FontTweak};
use std::sync::Arc;

mod app;
mod icons;
mod input;
mod models;
mod scraper;
mod theme;
mod ui;

use app::GooseModManager;
use theme::{
    FONT_WEIGHT_AXIS, FONT_WEIGHT_BOLD, FONT_WEIGHT_LIGHT, FONT_WEIGHT_REGULAR, NOTO_SANS_BOLD,
    NOTO_SANS_BOLD_FACE, NOTO_SANS_LIGHT, NOTO_SANS_LIGHT_FACE, NOTO_SANS_REGULAR,
    NOTO_SANS_REGULAR_FACE,
};

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

            for (name, weight) in [
                (NOTO_SANS_LIGHT_FACE, FONT_WEIGHT_LIGHT),
                (NOTO_SANS_REGULAR_FACE, FONT_WEIGHT_REGULAR),
                (NOTO_SANS_BOLD_FACE, FONT_WEIGHT_BOLD),
            ] {
                fonts.font_data.insert(
                    name.to_owned(),
                    Arc::new(
                        FontData::from_static(include_bytes!(
                            "../assets/fonts/NotoSans-Variable.ttf"
                        ))
                        .tweak(FontTweak {
                            coords: VariationCoords::new([(FONT_WEIGHT_AXIS, weight)]),
                            ..Default::default()
                        }),
                    ),
                );
            }

            for (family, font) in [
                (NOTO_SANS_LIGHT, NOTO_SANS_LIGHT_FACE),
                (NOTO_SANS_REGULAR, NOTO_SANS_REGULAR_FACE),
                (NOTO_SANS_BOLD, NOTO_SANS_BOLD_FACE),
            ] {
                fonts
                    .families
                    .insert(FontFamily::Name(family.into()), vec![font.to_owned()]);
            }

            fonts
                .families
                .get_mut(&FontFamily::Proportional)
                .unwrap()
                .insert(0, NOTO_SANS_REGULAR_FACE.to_owned());

            cc.egui_ctx.set_fonts(fonts);

            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::new(GooseModManager::default()))
        }),
    )
}
