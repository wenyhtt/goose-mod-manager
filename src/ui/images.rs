use eframe::egui::{self, CornerRadius, Pos2, Rect, Vec2};

pub fn bytes_image(bytes: egui::load::Bytes, path: &str) -> egui::Image<'static> {
    egui::Image::new(egui::ImageSource::Bytes {
        uri: format!("bytes://{}", path.replace('\\', "/")).into(),
        bytes,
    })
}

pub fn fallback_image() -> egui::Image<'static> {
    egui::Image::new(egui::ImageSource::Bytes {
        uri: "bytes://card_image_fallback.png".into(),
        bytes: egui::load::Bytes::Static(include_bytes!("../../assets/card_image_fallback.png")),
    })
}

pub fn paint_cover_image(
    ui: &mut egui::Ui,
    rect: Rect,
    rounding: CornerRadius,
    image: egui::Image<'static>,
) {
    let rect_aspect = rect.width() / rect.height();
    let cropped =
        if let Some(natural_size) = image.load_and_calc_size(ui, Vec2::splat(f32::INFINITY)) {
            let img_aspect = natural_size.x / natural_size.y;
            let uv = if img_aspect > rect_aspect {
                let visible = rect_aspect / img_aspect;
                let off = (1.0 - visible) / 2.0;
                Rect::from_min_max(Pos2::new(off, 0.0), Pos2::new(1.0 - off, 1.0))
            } else {
                let visible = img_aspect / rect_aspect;
                let off = (1.0 - visible) / 2.0;
                Rect::from_min_max(Pos2::new(0.0, off), Pos2::new(1.0, 1.0 - off))
            };
            image.uv(uv)
        } else {
            fallback_image()
        };

    cropped
        .corner_radius(rounding)
        .fit_to_exact_size(rect.size())
        .paint_at(ui, rect);
}
