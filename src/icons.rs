use eframe::egui::{self, Color32, Pos2, Stroke};

pub fn paint_left_arrow(painter: &egui::Painter, center: Pos2, size: f32, color: Color32) {
    let s = size / 2.0;
    let stroke = Stroke::new(3.0, color);
    let points = vec![
        Pos2::new(center.x + s * 0.3, center.y - s * 0.5),
        Pos2::new(center.x - s * 0.3, center.y),
        Pos2::new(center.x + s * 0.3, center.y + s * 0.5),
    ];
    painter.add(egui::Shape::line(points, stroke));
}

pub fn paint_right_arrow(painter: &egui::Painter, center: Pos2, size: f32, color: Color32) {
    let s = size / 2.0;
    let stroke = Stroke::new(3.0, color);
    let points = vec![
        Pos2::new(center.x - s * 0.3, center.y - s * 0.5),
        Pos2::new(center.x + s * 0.3, center.y),
        Pos2::new(center.x - s * 0.3, center.y + s * 0.5),
    ];
    painter.add(egui::Shape::line(points, stroke));
}

pub fn paint_dropdown_arrow(painter: &egui::Painter, center: Pos2, size: f32, color: Color32) {
    let s = size / 2.0;
    let points = vec![
        Pos2::new(center.x, center.y + s * 0.4),
        Pos2::new(center.x - s * 0.7, center.y - s * 0.3),
        Pos2::new(center.x + s * 0.7, center.y - s * 0.3),
    ];
    painter.add(egui::Shape::convex_polygon(points, color, Stroke::NONE));
}

pub fn paint_download_icon(painter: &egui::Painter, center: Pos2, size: f32, color: Color32) {
    let s = size / 2.0;
    let stroke = Stroke::new(2.0, color);

    // Arrow shaft
    painter.line_segment(
        [
            Pos2::new(center.x, center.y - s * 0.6),
            Pos2::new(center.x, center.y + s * 0.2),
        ],
        stroke,
    );
    // Arrow head
    painter.line_segment(
        [
            Pos2::new(center.x - s * 0.4, center.y - s * 0.1),
            Pos2::new(center.x, center.y + s * 0.3),
        ],
        stroke,
    );
    painter.line_segment(
        [
            Pos2::new(center.x + s * 0.4, center.y - s * 0.1),
            Pos2::new(center.x, center.y + s * 0.3),
        ],
        stroke,
    );
    // Tray
    painter.line_segment(
        [
            Pos2::new(center.x - s * 0.7, center.y + s * 0.3),
            Pos2::new(center.x - s * 0.7, center.y + s * 0.7),
        ],
        stroke,
    );
    painter.line_segment(
        [
            Pos2::new(center.x - s * 0.7, center.y + s * 0.7),
            Pos2::new(center.x + s * 0.7, center.y + s * 0.7),
        ],
        stroke,
    );
    painter.line_segment(
        [
            Pos2::new(center.x + s * 0.7, center.y + s * 0.7),
            Pos2::new(center.x + s * 0.7, center.y + s * 0.3),
        ],
        stroke,
    );
}

pub fn paint_heart_icon(painter: &egui::Painter, center: Pos2, size: f32, color: Color32) {
    let s = size / 2.0;
    let stroke = Stroke::new(2.0, color);
    let points = vec![
        Pos2::new(center.x, center.y + s * 0.55),
        Pos2::new(center.x - s * 0.75, center.y - s * 0.1),
        Pos2::new(center.x - s * 0.45, center.y - s * 0.65),
        Pos2::new(center.x, center.y - s * 0.35),
        Pos2::new(center.x + s * 0.45, center.y - s * 0.65),
        Pos2::new(center.x + s * 0.75, center.y - s * 0.1),
        Pos2::new(center.x, center.y + s * 0.55),
    ];
    painter.add(egui::Shape::line(points, stroke));
}
