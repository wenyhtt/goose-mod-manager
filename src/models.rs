use eframe::egui;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ModEntry {
    pub name: String,
    pub category: String,
    pub url: String,
    pub thumbnail_path: Option<String>,
    #[serde(skip)]
    pub image_bytes: Option<egui::load::Bytes>,
}

impl ModEntry {
    pub fn new(name: &str, category: &str, url: &str, thumbnail_path: Option<&str>) -> Self {
        Self {
            name: name.to_string(),
            category: category.to_string(),
            url: url.to_string(),
            thumbnail_path: thumbnail_path.map(|s| s.to_string()),
            image_bytes: None,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum SortOption {
    Vehicle,
    Visual,
    Player,
}

impl SortOption {
    pub fn label(&self) -> &str {
        match self {
            SortOption::Vehicle => "VEHICLE",
            SortOption::Visual => "VISUAL",
            SortOption::Player => "PLAYER",
        }
    }
    pub fn all() -> &'static [SortOption] {
        &[SortOption::Vehicle, SortOption::Visual, SortOption::Player]
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Tab {
    Browse,
    Installed,
}

#[derive(Clone, Copy)]
pub enum IconKind {
    LeftArrow,
    RightArrow,
}
