use eframe::egui::{Color32, FontFamily, FontId};

// ── Colour palette ──────────────────────────────────────────────────────────
pub const BG_DARK: Color32 = Color32::from_rgb(0x0F, 0x0A, 0x05);
pub const CARD_BG: Color32 = Color32::from_rgb(0x1F, 0x15, 0x0C);
pub const CARD_BG_HOVER: Color32 = Color32::from_rgb(0x41, 0x2D, 0x15);
pub const TEXT_WHITE: Color32 = Color32::from_rgb(0xE1, 0xDC, 0xC9);
pub const ICON_COLOR: Color32 = Color32::from_rgb(0xE1, 0xDC, 0xC9);
pub const TAB_ACTIVE: Color32 = Color32::from_rgb(0x41, 0x2D, 0x15);
pub const TAB_INACTIVE: Color32 = Color32::from_rgb(0x1F, 0x15, 0x0C);
pub const FOCUS_COLOR: Color32 = Color32::from_rgb(0xE1, 0xDC, 0xC9);

// ── Dimensions ───────────────────────────────────────────────────────────────
pub const PILL_HEIGHT: f32 = 52.0;
pub const PILL_WIDTH: f32 = 186.0;
pub const ICON_BTN_SIZE: f32 = 52.0;
pub const CARD_RADIUS: u8 = 24;
pub const PILL_RADIUS: u8 = 50;

// ── Fonts ────────────────────────────────────────────────────────────────────
pub fn poppins() -> FontId {
    FontId::new(24.0, FontFamily::Name("Poppins".into()))
}

pub fn poppins_sm() -> FontId {
    FontId::new(20.0, FontFamily::Name("Poppins".into()))
}

pub fn poppins_xs() -> FontId {
    FontId::new(14.0, FontFamily::Name("Poppins".into()))
}
