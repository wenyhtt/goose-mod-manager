# Graph Report - goose-mod-manager  (2026-08-08)

## Corpus Check
- 25 files · ~30,536 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 208 nodes · 353 edges · 19 communities (15 shown, 4 thin omitted)
- Extraction: 93% EXTRACTED · 7% INFERRED · 0% AMBIGUOUS · INFERRED: 23 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `c2021ed9`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- src/scraper.rs
- GooseModManager
- theme.rs
- What You Must Do When Invoked
- ModEntry
- AI Agent Instructions for Goose Mod Manager
- graphify reference: extra exports and benchmark
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
1. `GooseModManager` - 35 edges
2. `ModEntry` - 22 edges
3. `paint_card()` - 12 edges
4. `What You Must Do When Invoked` - 12 edges
5. `run_page()` - 10 edges
6. `/graphify` - 10 edges
7. `paint_footer()` - 9 edges
8. `paint_header()` - 8 edges
9. `paint_images()` - 8 edges
10. `paint_preview()` - 8 edges

## Surprising Connections (you probably didn't know these)
- `arrow_button()` --calls--> `paint_left_arrow()`  [INFERRED]
  src/ui/details.rs → src/icons.rs
- `arrow_button()` --calls--> `paint_right_arrow()`  [INFERRED]
  src/ui/details.rs → src/icons.rs
- `paint_card()` --calls--> `paint_download_icon()`  [INFERRED]
  src/ui/grid.rs → src/icons.rs
- `paint_header()` --calls--> `poppins()`  [INFERRED]
  src/ui/details.rs → src/theme.rs
- `dialog_button()` --calls--> `poppins_sm()`  [INFERRED]
  src/ui/details.rs → src/theme.rs

## Import Cycles
- None detected.

## Communities (19 total, 4 thin omitted)

### Community 0 - "src/scraper.rs"
Cohesion: 0.16
Nodes (27): Box, ElementRef, Error, HashMap, Html, PathBuf, Selector, absolute_url() (+19 more)

### Community 1 - "GooseModManager"
Cohesion: 0.10
Nodes (14): App, Default, Frame, Gilrs, JoinHandle, GooseModManager, Option, Result (+6 more)

### Community 2 - "theme.rs"
Cohesion: 0.17
Nodes (21): FontId, paint_download_icon(), paint_dropdown_arrow(), paint_heart_icon(), paint_left_arrow(), paint_right_arrow(), Color32, Painter (+13 more)

### Community 3 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 4 - "ModEntry"
Cohesion: 0.17
Nodes (23): Context, ModEntry, Bytes, Option, Self, String, Vec, arrow_button() (+15 more)

### Community 5 - "AI Agent Instructions for Goose Mod Manager"
Cohesion: 0.22
Nodes (8): 1. Architecture & State Management, 2. Layout & Responsiveness, 3. Assets & Images, 4. Iconography, 5. Navigation & Focus (10-Foot UI), 6. Philosophy (YAGNI), AI Agent Instructions for Goose Mod Manager, graphify

### Community 6 - "graphify reference: extra exports and benchmark"
Cohesion: 0.22
Nodes (8): graphify reference: extra exports and benchmark, Step 6b - Wiki (only if --wiki flag), Step 7 - Neo4j export (only if --neo4j or --neo4j-push flag), Step 7a - FalkorDB export (only if --falkordb or --falkordb-push flag), Step 7b - SVG export (only if --svg flag), Step 7c - GraphML export (only if --graphml flag), Step 7d - MCP server (only if --mcp flag), Step 8 - Token reduction benchmark (only if total_words > 5000)

### Community 8 - "paint_card"
Cohesion: 0.22
Nodes (14): CornerRadius, Image, paint_card(), paint_skeleton(), render_grid(), Painter, Rect, Ui (+6 more)

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

- **Why does `GooseModManager` connect `GooseModManager` to `src/scraper.rs`, `paint_card`, `theme.rs`, `ModEntry`?**
  _High betweenness centrality (0.156) - this node is a cross-community bridge._
- **Why does `ModEntry` connect `ModEntry` to `src/scraper.rs`, `GooseModManager`, `paint_card`?**
  _High betweenness centrality (0.149) - this node is a cross-community bridge._
- **Why does `paint_card()` connect `paint_card` to `theme.rs`, `ModEntry`?**
  _High betweenness centrality (0.049) - this node is a cross-community bridge._
- **Are the 6 inferred relationships involving `paint_card()` (e.g. with `paint_download_icon()` and `poppins_sm()`) actually correct?**
  _`paint_card()` has 6 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Usage`, `What graphify is for`, `Step 0 - GitHub repos and multi-path merge (only if a URL or several paths)` to the rest of the system?**
  _51 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `GooseModManager` be split into smaller, more focused modules?**
  _Cohesion score 0.0989247311827957 - nodes in this community are weakly interconnected._
- **Should `What You Must Do When Invoked` be split into smaller, more focused modules?**
  _Cohesion score 0.08 - nodes in this community are weakly interconnected._