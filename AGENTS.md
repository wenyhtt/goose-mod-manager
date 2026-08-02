# AI Agent Instructions for Goose Mod Manager

This file outlines the architectural constraints and design patterns of the `goose-mod-manager` codebase. Agents modifying this code must strictly adhere to these rules to maintain performance and consistency.

## 1. Architecture & State Management
- **Immediate Mode**: The app uses `egui`. The UI is destroyed and rebuilt from scratch 60 times a second. There is no widget tree.
- **Single Source of Truth**: All persistent application state (active tab, current page, parsed mod lists, grid dimensions) lives in `src/app.rs` inside the `GooseModManager` struct. Do not spawn isolated state singletons.

## 2. Layout & Responsiveness
- **Dynamic Grids**: Do not hardcode grid columns or rows. The grid mathematically computes how many items fit based on `ui.available_size()` divided by `(min_width + gap)`.
- **Flow vs. Absolute**: Rely on `ui.horizontal` and `ui.vertical` for standard flow. Only use absolute positioning (e.g., `Pos2::new(x, y)`, `ui.put()`, or `painter.text()`) when elements must break flow, such as the absolute-centered page indicator or the floating "Sort By" dropdown overlay.

## 3. Assets & Images
- **Image Loading**: We do **not** use manual texture decoding. The `egui_extras` image loader is initialized in `main.rs`. 
- **Usage**: Load images strictly via `egui::ImageSource::Bytes` with `include_bytes!`. 
- **Cropping**: Do not stretch images. Apply CSS `object-fit: cover` behavior by dynamically calculating the image's natural aspect ratio vs the target `Rect` aspect ratio, and dynamically assigning the `.uv()` coordinates to crop the excess from the center.

## 4. Iconography
- **No SVGs**: Do not introduce SVG dependencies or files. They are too heavy for our needs.
- **Manual Painting**: All icons (chevrons, download buttons) are hand-coded mathematically in `src/icons.rs` using `egui::Shape::line` and `painter.line_segment()`. Always use open line paths for icons, never `convex_polygon` (which breaks on concave shapes).

## 5. Navigation & Focus (10-Foot UI)
- **Gamepad-First**: The app is designed for controllers and keyboards, not just mice. 
- **Focus Discovery**: Arrow keys are intercepted in `app.rs`. We trigger navigation via `ui.ctx().memory_mut(|mem| mem.move_focus(egui::FocusDirection::...))`.
- **Auto-Focus**: If an arrow key is pressed but nothing is currently focused, explicitly request focus on the primary navigation tab (`tab_browse`) so the user isn't stuck.
- **Visuals**: Disable egui's default focus ring globally. All interactive widgets (`src/ui/widgets.rs` and the main cards) manually check `response.has_focus()` and draw a custom neon border (`FOCUS_COLOR`).

## 6. Philosophy (YAGNI)
- **You Aren't Gonna Need It**: Build the absolute minimum that works. No avoidable dependencies, no heavy abstractions, no boilerplate.

## graphify

This project has a knowledge graph at graphify-out/ with god nodes, community structure, and cross-file relationships.

When the user types `/graphify`, use the installed graphify skill or instructions before doing anything else.

Rules:
- For codebase questions, first run `graphify query "<question>"` when graphify-out/graph.json exists. Use `graphify path "<A>" "<B>"` for relationships and `graphify explain "<concept>"` for focused concepts. These return a scoped subgraph, usually much smaller than GRAPH_REPORT.md or raw grep output.
- Dirty graphify-out/ files are expected after hooks or incremental updates; dirty graph files are not a reason to skip graphify. Only skip graphify if the task is about stale or incorrect graph output, or the user explicitly says not to use it.
- If graphify-out/wiki/index.md exists, use it for broad navigation instead of raw source browsing.
- Read graphify-out/GRAPH_REPORT.md only for broad architecture review or when query/path/explain do not surface enough context.
- After modifying code, run `graphify update .` to keep the graph current (AST-only, no API cost).
