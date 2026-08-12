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
pub const FONT_XS: f32 = 14.0;
pub const FONT_SM: f32 = 20.0;
pub const FONT_MD: f32 = 24.0;
pub const FONT_EMPTY: f32 = 36.0;

pub const FONT_WEIGHT_LIGHT: f32 = 300.0;
pub const FONT_WEIGHT_REGULAR: f32 = 400.0;
pub const FONT_WEIGHT_BOLD: f32 = 700.0;
pub const FONT_WEIGHT_AXIS: &[u8; 4] = b"wght";

pub const NOTO_SANS_LIGHT_FACE: &str = "NotoSans-Light";
pub const NOTO_SANS_REGULAR_FACE: &str = "NotoSans-Regular";
pub const NOTO_SANS_BOLD_FACE: &str = "NotoSans-Bold";
pub const NOTO_SANS_LIGHT: &str = "NotoSansLight";
pub const NOTO_SANS_REGULAR: &str = "NotoSansRegular";
pub const NOTO_SANS_BOLD: &str = "NotoSansBold";

pub fn noto_sans_light(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(NOTO_SANS_LIGHT.into()))
}

pub fn noto_sans_regular(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(NOTO_SANS_REGULAR.into()))
}

pub fn noto_sans_bold(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(NOTO_SANS_BOLD.into()))
}
