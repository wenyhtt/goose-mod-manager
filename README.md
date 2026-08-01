# Goose Mod Manager

A premium, responsive, and gamepad-friendly Mod Manager UI built in Rust with `egui`.

## 🏗️ Architecture & Codebase Overview

This project is built using an immediate-mode GUI paradigm. The UI is completely redrawn every frame, which allows for highly dynamic responsive layouts and custom styling without the overhead of traditional DOM trees.

### 🗂️ Directory Structure

- **`src/main.rs`**  
  The entry point of the application. It initializes the `eframe` (egui framework) window with custom dimensions, dark theme settings, and bootstraps the `GooseModManager` app state.

- **`src/app.rs`**  
  Contains the `GooseModManager` struct which holds the global application state (active tab, current page, mock mod data, sort dropdown state, and dynamic grid row/col counts).
  - Handles initial focus and keyboard/gamepad 2D arrow navigation logic.
  - Dynamically calculates how many mod cards can fit on the screen based on window size.

- **`src/models.rs`**  
  Defines the core data structures used throughout the app:
  - `ModEntry`: Represents a single mod (name, category, size).
  - `SortOption`: Enums for sorting categories.
  - `Tab`: Enums for "Browse" vs "Installed" views.
  - `IconKind`: Enums for icon references.

- **`src/theme.rs`**  
  The design system. Contains constant definitions for all colors (`CARD_BG`, `FOCUS_COLOR`, etc.), UI measurements (pill sizes, border radii), and helper functions for loading the custom "Poppins" fonts.

- **`src/icons.rs`**  
  Contains custom rendering logic for iconography. Instead of loading complex SVG files that might render unpredictably, icons (like the left/right chevron arrows, dropdown, and download buttons) are hand-drawn natively using `egui::Shape::line` and `painter.line_segment()`.

### 🧩 UI Components (`src/ui/`)

The UI logic is cleanly separated into modular components:

- **`src/ui/top_bar.rs`**  
  Renders the top navigation area.
  - Contains the "Browse" and "Installed" tabs, alongside the page left/right arrows.
  - Implements the "Sort By" dropdown. The dropdown menu uses `egui::Area` to render as a foreground overlay, ensuring it perfectly overlaps the grid below it without shifting the layout.
  - Uses exact coordinate tracking to perfectly align elements horizontally and vertically.

- **`src/ui/grid.rs`**  
  Renders the main grid of mod cards.
  - Loops over the current page's `ModEntry` data and lays them out in a responsive grid.
  - Handles custom mesh rendering to apply border-radius clipping *only* to the top corners of the mod image thumbnails.
  - Applies hover and focus ring visuals to the cards.

- **`src/ui/widgets.rs`**  
  Contains highly reusable, low-level interactive widgets:
  - `pill_button`: A custom rounded button used for the main tabs.
  - `icon_button`: A custom circular button used for pagination.
  - Both widgets manually hook into `egui::Sense` to handle click detection, hover states, and drawing custom neon focus rings.

## 🎮 Navigation & Focus

The app is built to support "10-foot UI" experiences (like a Smart TV or Gamepad). 
- In `app.rs`, the app explicitly listens for Keyboard Arrow inputs (Up/Down/Left/Right).
- When an arrow is pressed, it triggers `ui.ctx().memory_mut(|mem| mem.move_focus(...))` to seamlessly jump focus between the cards, tabs, and buttons without ever needing a mouse.
