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
| TODO | Push to GitHub | Push `main` after Phase 0 commit. |
| BLOCKED | Update global status | Waiting for permission to write `/Users/mickyyang/STATUS.md` outside the workspace. |

## Future Planning Only — Do Not Implement In Phase 0

| Phase | Status | Scope |
| --- | --- | --- |
| Phase 1 | FUTURE | Review shell UI: speaker room, professional transport/timeline, right notes panel. |
| Phase 2 | FUTURE | Project / Version / Note SQLite schema and workflow. |
| Phase 3 | FUTURE | Hotkeys, timeline note creation, note seek, playback highlight. |
| Phase 4 | FUTURE | V1/V2 version review, Done/OK states, note filters. |
| Phase 5 | FUTURE | CSV, plain text, and Markdown export. |
| Phase 6 | FUTURE | 7.1.4 speaker room visualization with meter glow and solo/mute. |
