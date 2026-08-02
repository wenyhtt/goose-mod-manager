use eframe::egui;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ModEntry {
    pub name: String,
    pub category: String,
    pub url: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub downloads: String,
    pub thumbnail_path: Option<String>,
    #[serde(skip)]
    pub image_bytes: Option<egui::load::Bytes>,
}

impl ModEntry {
    pub fn new(
        name: &str,
        category: &str,
        url: &str,
        author: &str,
        version: &str,
        downloads: &str,
        thumbnail_path: Option<&str>,
    ) -> Self {
        Self {
            name: name.to_string(),
            category: category.to_string(),
            url: url.to_string(),
            author: author.to_string(),
            version: version.to_string(),
            downloads: downloads.to_string(),
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

#[cfg(test)]
mod tests {
    use super::ModEntry;

    #[test]
    fn old_cache_entries_get_empty_metadata() {
        let entry: ModEntry = serde_json::from_str(
            r#"{"name":"Test","category":"Car","url":"https://example.com"}"#,
        )
        .unwrap();

        assert!(entry.author.is_empty());
        assert!(entry.version.is_empty());
    }
}
