# Graph Report - goose-mod-manager  (2026-08-03)

## Corpus Check
- 23 files · ~15,770 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 165 nodes · 232 edges · 20 communities (16 shown, 4 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 11 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `afd1eebf`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- ModEntry
- GooseModManager
- render_sort_controls
- What You Must Do When Invoked
- /graphify
- AI Agent Instructions for Goose Mod Manager
- graphify reference: extra exports and benchmark
- icons.rs
- paint_card
- graphify reference: query, path, explain
- 🏗️ Architecture & Codebase Overview
- graphify reference: add a URL and watch a folder
- graphify reference: commit hook and native CLAUDE.md integration
- graphify reference: incremental update and cluster-only
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- main
- extraction-spec.md

## God Nodes (most connected - your core abstractions)
1. `GooseModManager` - 25 edges
2. `ModEntry` - 14 edges
3. `What You Must Do When Invoked` - 12 edges
4. `run_page()` - 10 edges
5. `/graphify` - 10 edges
6. `paint_card()` - 9 edges
7. `render_sort_controls()` - 8 edges
8. `graphify reference: extra exports and benchmark` - 8 edges
9. `AI Agent Instructions for Goose Mod Manager` - 8 edges
10. `load_cached_mods()` - 7 edges

## Surprising Connections (you probably didn't know these)
- `icon_button()` --calls--> `paint_left_arrow()`  [INFERRED]
  src/ui/widgets.rs → src/icons.rs
- `icon_button()` --calls--> `paint_right_arrow()`  [INFERRED]
  src/ui/widgets.rs → src/icons.rs
- `render_sort_controls()` --calls--> `paint_dropdown_arrow()`  [INFERRED]
  src/ui/top_bar.rs → src/icons.rs
- `paint_card()` --calls--> `paint_download_icon()`  [INFERRED]
  src/ui/grid.rs → src/icons.rs
- `paint_card()` --calls--> `poppins_sm()`  [INFERRED]
  src/ui/grid.rs → src/theme.rs

## Import Cycles
- None detected.

## Communities (20 total, 4 thin omitted)

### Community 0 - "ModEntry"
Cohesion: 0.15
Nodes (23): Box, Bytes, ElementRef, Error, HashMap, PathBuf, Selector, ModEntry (+15 more)

### Community 1 - "GooseModManager"
Cohesion: 0.11
Nodes (14): App, Default, Frame, Gilrs, JoinHandle, GooseModManager, Option, Result (+6 more)

### Community 2 - "render_sort_controls"
Cohesion: 0.23
Nodes (13): FontId, Response, IconKind, poppins(), poppins_sm(), poppins_xs(), render_sort_controls(), render_top_bar() (+5 more)

### Community 3 - "What You Must Do When Invoked"
Cohesion: 0.13
Nodes (15): Part A - Structural extraction for code files, Part B - Semantic extraction (parallel subagents), Part C - Merge AST + semantic into final extraction, Step 0 - GitHub repos and multi-path merge (only if a URL or several paths), Step 1 - Ensure graphify is installed, Step 2.5 - Video and audio (only if video files detected), Step 2 - Detect files, Step 3 - Extract entities and relationships (+7 more)

### Community 4 - "/graphify"
Cohesion: 0.20
Nodes (9): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Usage (+1 more)

### Community 5 - "AI Agent Instructions for Goose Mod Manager"
Cohesion: 0.22
Nodes (8): 1. Architecture & State Management, 2. Layout & Responsiveness, 3. Assets & Images, 4. Iconography, 5. Navigation & Focus (10-Foot UI), 6. Philosophy (YAGNI), AI Agent Instructions for Goose Mod Manager, graphify

### Community 6 - "graphify reference: extra exports and benchmark"
Cohesion: 0.22
Nodes (8): graphify reference: extra exports and benchmark, Step 6b - Wiki (only if --wiki flag), Step 7 - Neo4j export (only if --neo4j or --neo4j-push flag), Step 7a - FalkorDB export (only if --falkordb or --falkordb-push flag), Step 7b - SVG export (only if --svg flag), Step 7c - GraphML export (only if --graphml flag), Step 7d - MCP server (only if --mcp flag), Step 8 - Token reduction benchmark (only if total_words > 5000)

### Community 7 - "icons.rs"
Cohesion: 0.57
Nodes (7): Color32, paint_download_icon(), paint_dropdown_arrow(), paint_left_arrow(), paint_right_arrow(), Painter, Pos2

### Community 8 - "paint_card"
Cohesion: 0.52
Nodes (6): Rect, paint_card(), paint_skeleton(), render_grid(), Painter, Ui

### Community 9 - "graphify reference: query, path, explain"
Cohesion: 0.33
Nodes (5): For /graphify explain, For /graphify path, graphify reference: query, path, explain, Step 0 — Constrained query expansion (REQUIRED before traversal), Step 1 — Traversal

### Community 10 - "🏗️ Architecture & Codebase Overview"
Cohesion: 0.33
Nodes (5): 🏗️ Architecture & Codebase Overview, 🗂️ Directory Structure, Goose Mod Manager, 🎮 Navigation & Focus, 🧩 UI Components (`src/ui/`)

### Community 11 - "graphify reference: add a URL and watch a folder"
Cohesion: 0.50
Nodes (3): For /graphify add, For --watch, graphify reference: add a URL and watch a folder

### Community 12 - "graphify reference: commit hook and native CLAUDE.md integration"
Cohesion: 0.50
Nodes (3): For git commit hook, For native CLAUDE.md integration, graphify reference: commit hook and native CLAUDE.md integration

### Community 13 - "graphify reference: incremental update and cluster-only"
Cohesion: 0.50
Nodes (3): For --cluster-only, For --update (incremental re-extraction), graphify reference: incremental update and cluster-only

## Knowledge Gaps
- **51 isolated node(s):** `Usage`, `What graphify is for`, `Step 0 - GitHub repos and multi-path merge (only if a URL or several paths)`, `Step 1 - Ensure graphify is installed`, `Step 2 - Detect files` (+46 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **4 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `GooseModManager` connect `GooseModManager` to `ModEntry`, `paint_card`, `render_sort_controls`?**
  _High betweenness centrality (0.124) - this node is a cross-community bridge._
- **Why does `ModEntry` connect `ModEntry` to `paint_card`, `GooseModManager`?**
  _High betweenness centrality (0.124) - this node is a cross-community bridge._
- **Why does `paint_card()` connect `paint_card` to `ModEntry`, `render_sort_controls`, `icons.rs`?**
  _High betweenness centrality (0.040) - this node is a cross-community bridge._
- **What connects `Usage`, `What graphify is for`, `Step 0 - GitHub repos and multi-path merge (only if a URL or several paths)` to the rest of the system?**
  _51 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `ModEntry` be split into smaller, more focused modules?**
  _Cohesion score 0.1452991452991453 - nodes in this community are weakly interconnected._
- **Should `GooseModManager` be split into smaller, more focused modules?**
  _Cohesion score 0.11384615384615385 - nodes in this community are weakly interconnected._
- **Should `What You Must Do When Invoked` be split into smaller, more focused modules?**
  _Cohesion score 0.13333333333333333 - nodes in this community are weakly interconnected._