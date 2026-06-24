# Changes

- Added `src-tauri/capabilities/default.json` with required Tauri permissions (drag-drop events, IPC invoke commands, event listening, window operations).
- Added the `event` feature to the Tauri dependency in `src-tauri/Cargo.toml` to ensure runtime events function correctly.
- Updated the drag event listener in `src/components/Player.tsx` from `tauri://drag` to `tauri://drag-enter`.
- Added an explicit `set_property("pause", false)` in `src-tauri/src/player/mpv.rs` after the file load command to ensure auto-play.
