# SKILLs.md — “80s Defrag” Assisted Disk Cleanup (dragonfly)

## Experience goal
A tidy, full-screen terminal experience:
1) Retro “defrag blocks” animation while scanning
2) Clear, minimal summary
3) Guided drill-down + “cleanup tray” review
4) Explicit confirm before any deletion

## Terminal mode (full screen)
- Enter alternate screen buffer for a clean UI (no scrollback noise). [web:38][web:42]
- Hide cursor during animation; restore on exit. [web:38]
- Always restore terminal state even on Ctrl+C.

Suggested sequences:
- Enter alt screen: ESC[?1049h  (also clear screen)
- Exit alt screen:  ESC[?1049l
(Implementation may use terminfo/curses; the behavior is “alternate screen buffer”.) [web:38][web:42]

## Visual language (tidy retro)
- 80-column layout by default (fallback to current terminal width).
- Use a single block glyph set, e.g.:
  - Free block: "."
  - Used block: "#"
  - “Moving” block: "@"
  - Highlight selection: inverse video
- Keep animation in a fixed panel; keep logs out of the animated area.

## Workflow

### Skill: Animated scan (“Defrag Theater”)
Purpose: entertain while scanning, but still convey progress.
Animation:
- Render a grid of blocks representing allocation.
- Periodically “move” clusters to show “consolidation” while scan runs.
- Progress line: “Scanning ~/ … 42% | 18.3 GB indexed | 1.2M files”

Data sources:
- Use `dragonfly disk analyze <PATH>` as the real scanner. [attached_file:1]
- Prefer `--json` output for structured results: `dragonfly disk analyze <PATH> --json > report.json`. [attached_file:1]

Rules:
- Animation must not imply actual defragmentation (APFS doesn’t need/allow traditional defrag).
- It is only a progress visualization overlay for scanning.

### Skill: Post-scan “Top offenders” screen (minimal)
Render a table-like view (still in TUI):
- Biggest folders/files (top 20)
- Size + path
- A short hint tag per item: (Download), (Media), (Dev), (Archive), (Unknown)

Actions:
- [Enter] drill into selected path (re-run analyze on that path)
- [A] add to cleanup tray
- [R] rescan with higher min-size threshold
- [Q] quit

### Skill: Drill-down (one level at a time)
Command pattern:
- `dragonfly disk analyze <SELECTED_PATH> --min-size <SIZE>`
Optional:
- `--json` for stable UI rendering. [attached_file:1]

UI rule:
- Never show more than one screenful; paginate.
- Always show current breadcrumb: `~/Movies/Screen Recordings`

### Skill: Cleanup tray (review before delete)
A separate screen that lists chosen candidates:
- Path | Size | Intended action (Trash/Archive/Delete) | Notes

Rules:
- Default action is “Trash” (or “Archive”), not permanent delete.
- Require a final review step.

### Skill: Duplicates (high-value cleanup)
Commands:
- `dragonfly duplicates scan <PATH> --dry-run` (preview)
- `dragonfly duplicates scan <PATH> --interactive` (confirm per choice) [attached_file:1]

UI integration:
- Provide a “Duplicates” mode with the same tray/confirm flow.

### Skill: Cleanup execution (safe-first)
Commands:
- `dragonfly clean --dry-run` then (only after confirmation) run without dry-run. [attached_file:1]

Non-negotiables:
- Always support “dry-run preview” for every destructive action.
- Always show “About to remove: N items / X GB” and require YES.

## Guardrails (macOS)
- Never touch `/System` or other OS-critical locations by default.
- Treat `~/Library` as “advanced mode” only.
- Provide an “exclude patterns” list (node_modules, build outputs, etc.) as user-configurable.

## Output artifacts
- Save scan JSON reports to `./reports/<timestamp>-scan.json`
- Save cleanup tray plan to `./reports/<timestamp>-cleanup-plan.md`
