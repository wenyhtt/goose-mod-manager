use eframe::egui::{Color32, FontFamily, FontId};

// ── Colour palette (from Figma) ──────────────────────────────────────────────
pub const BG_DARK: Color32 = Color32::from_rgb(0x27, 0x32, 0x3A);
pub const CARD_BG: Color32 = Color32::from_rgb(0x43, 0x50, 0x55);
pub const CARD_BG_HOVER: Color32 = Color32::from_rgb(0x4E, 0x5D, 0x63);
pub const TEXT_WHITE: Color32 = Color32::from_rgb(0xFF, 0xFF, 0xFF);
pub const ICON_COLOR: Color32 = Color32::from_rgb(0xE3, 0xE3, 0xE3);
pub const TAB_ACTIVE: Color32 = Color32::from_rgb(0x43, 0x50, 0x55);
pub const TAB_INACTIVE: Color32 = Color32::from_rgb(0x35, 0x42, 0x49);
pub const FOCUS_COLOR: Color32 = Color32::from_rgb(0xA3, 0xF7, 0xBF);

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
