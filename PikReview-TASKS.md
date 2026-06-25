# Pik Review Task Board

## Phase 0 — Project Fork And Rename

| Status | Task | Notes |
| --- | --- | --- |
| DONE | Confirm target folder | `/Users/mickyyang/Projects/pik-review-player` exists and was empty before copy. |
| DONE | Read global status | `~/STATUS.md` was read at session start. |
| DONE | Read product prompt | Read `/Users/mickyyang/Projects/pik-player/PROFESSIONAL-PLAYER-PROMPT.md`. |
| DONE | Copy clean baseline | Copied from `pik-player` excluding `.git`, `node_modules`, `dist`, `src-tauri/target`, build artifacts, logs, and temporary files. |
| DONE | Rename app identity | Product name, window title, package name, Tauri identifier, Rust crate, and binary renamed for Pik Review. |
| DONE | Rename app data paths | Tauri app data isolation, SQLite filename, and cover cache path renamed away from Pik Player. |
| DONE | Initialize fresh git repo | Fresh `.git` created in `pik-review-player`; remote set to `git@github.com:mickyyang-1407/pik-review-player.git`. |
| DONE | Install dependencies | `npm install` succeeded; 0 vulnerabilities. |
| DONE | Verify frontend build | `npm run build` succeeded. |
| DONE | Verify Tauri app build | `npm run tauri build -- --bundles app` succeeded; app at `src-tauri/target/release/bundle/macos/Pik Review.app`. |
| DONE | Check native video regressions | Source and app binary scans found no `AVPlayerView`, `AVPlayerLayer`, `ns_window`, `content_view_ptr`, `get_webview_window`, or `AVKit`. |
| DONE | Create private GitHub repo | Micky created private GitHub repo `pik-review-player`. |
| DONE | Push to GitHub | Pushed `main` to `git@github.com:mickyyang-1407/pik-review-player.git`; commit `65e51dfa5b1cce9f01ac2d08e7a556f080aed5ca`. |
| DONE | Update global status | Updated `/Users/mickyyang/STATUS.md` with Phase 0 results and laptop handoff notes. |

## Completed Phases 1–6

| Phase | Status | Scope |
| --- | --- | --- |
| Phase 1 | DONE | Review shell UI: speaker room, professional transport/timeline, right notes panel. |
| Phase 2 | DONE | Project / Version / Note SQLite schema and workflow. |
| Phase 3 | DONE | Hotkeys, timeline note creation, note seek, playback highlight. |
| Phase 4 | DONE | V1/V2 version review, Done/OK states, note filters. |
| Phase 5 | DONE | CSV, plain text, and Markdown export. |
| Phase 6 | DONE | 7.1.4 speaker room visualization with meter glow and solo/mute. |
