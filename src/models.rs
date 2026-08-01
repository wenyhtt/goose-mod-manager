#[derive(Clone)]
pub struct ModEntry {
    pub name: String,
    pub category: String,
    pub size: String,
}

impl ModEntry {
    pub fn new(name: &str, category: &str, size: &str) -> Self {
        Self {
            name: name.to_string(),
            category: category.to_string(),
            size: size.to_string(),
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
