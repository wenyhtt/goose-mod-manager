# Graph Report - /home/diaz/Projects/goose-mod-manager  (2026-08-02)

## Corpus Check
- cluster-only mode — file stats not available

## Summary
- 72 nodes · 119 edges · 9 communities (7 shown, 2 thin omitted)
- Extraction: 91% EXTRACTED · 9% INFERRED · 0% AMBIGUOUS · INFERRED: 11 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `e4018745`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Community 0
- Community 1
- Community 2
- Community 3
- Community 4
- Community 5
- Community 6

## God Nodes (most connected - your core abstractions)
1. `GooseModManager` - 23 edges
2. `ModEntry` - 10 edges
3. `paint_card()` - 8 edges
4. `render_sort_controls()` - 8 edges
5. `render_top_bar()` - 7 edges
6. `icon_button()` - 7 edges
7. `paint_left_arrow()` - 5 edges
8. `paint_right_arrow()` - 5 edges
9. `paint_dropdown_arrow()` - 5 edges
10. `paint_download_icon()` - 5 edges

## Surprising Connections (you probably didn't know these)
- `GooseModManager` --references--> `Tab`  [EXTRACTED]
  src/app.rs → src/models.rs
- `icon_button()` --calls--> `paint_left_arrow()`  [INFERRED]
  src/ui/widgets.rs → src/icons.rs
- `icon_button()` --calls--> `paint_right_arrow()`  [INFERRED]
  src/ui/widgets.rs → src/icons.rs
- `render_sort_controls()` --calls--> `paint_dropdown_arrow()`  [INFERRED]
  src/ui/top_bar.rs → src/icons.rs
- `paint_card()` --calls--> `paint_download_icon()`  [INFERRED]
  src/ui/grid.rs → src/icons.rs

## Import Cycles
- None detected.

## Communities (9 total, 2 thin omitted)

### Community 0 - "Community 0"
Cohesion: 0.15
Nodes (12): App, Default, Frame, Gilrs, JoinHandle, GooseModManager, Option, Result (+4 more)

### Community 1 - "Community 1"
Cohesion: 0.26
Nodes (12): FontId, Response, IconKind, poppins(), poppins_sm(), render_sort_controls(), render_top_bar(), Pos2 (+4 more)

### Community 2 - "Community 2"
Cohesion: 0.21
Nodes (10): Bytes, Rect, ModEntry, Option, Self, String, paint_card(), render_grid() (+2 more)

### Community 3 - "Community 3"
Cohesion: 0.57
Nodes (7): Color32, paint_download_icon(), paint_dropdown_arrow(), paint_left_arrow(), paint_right_arrow(), Painter, Pos2

### Community 4 - "Community 4"
Cohesion: 0.38
Nodes (6): Box, Error, PathBuf, project_root(), Result, run()

## Knowledge Gaps
- **2 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `GooseModManager` connect `Community 0` to `Community 1`, `Community 2`, `Community 5`?**
  _High betweenness centrality (0.387) - this node is a cross-community bridge._
- **Why does `ModEntry` connect `Community 2` to `Community 0`, `Community 5`?**
  _High betweenness centrality (0.139) - this node is a cross-community bridge._
- **Why does `render_sort_controls()` connect `Community 1` to `Community 0`, `Community 3`?**
  _High betweenness centrality (0.109) - this node is a cross-community bridge._
- **Are the 2 inferred relationships involving `paint_card()` (e.g. with `paint_download_icon()` and `poppins_sm()`) actually correct?**
  _`paint_card()` has 2 INFERRED edges - model-reasoned connections that need verification._
- **Are the 3 inferred relationships involving `render_sort_controls()` (e.g. with `paint_dropdown_arrow()` and `poppins()`) actually correct?**
  _`render_sort_controls()` has 3 INFERRED edges - model-reasoned connections that need verification._
- **Are the 3 inferred relationships involving `render_top_bar()` (e.g. with `poppins()` and `icon_button()`) actually correct?**
  _`render_top_bar()` has 3 INFERRED edges - model-reasoned connections that need verification._