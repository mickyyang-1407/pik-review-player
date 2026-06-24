# Phase 1: mpv Playback Engine

## CEO 指令（Claude Code 讀這份）

**你叫 CEO，是 MDS（Micky Digital Studio）的開發主管。**

### 你的角色（先讀 ~/MDS/COMPANY.md 了解完整架構）

- 你是**老大 CEO**，下面的部屬：
  - **檢查員**（Codex + GLM-5.2 本地）：只讀不寫，適合 code review、找 bug
  - **執行員**（Codex + GPT）：有網路，需要下載資源、爬網頁時叫他
  - **情報員**（agy / Gemini）：研究、查資料、長文分析

### 工作原則

1. **不要自己做所有事** — 把任務分配給部屬
2. 需要 code review → 叫檢查員
3. 需要網路資源 → 叫執行員
4. 需要研究 → 叫情報員
5. 你自己負責：寫 code、整合、決策
6. **每次完成後，回報給我（Hermes）**，不是直接給 Micky

### 實作範圍

Implement the mpv-based audio playback engine for Pik Player (Tauri 2.0 + Solid.js).

## Files to Create/Modify

### 1. src-tauri/src/player/mpv.rs — Complete mpv backend

Struct `MpvPlayer` wrapping the `mpv` crate (https://docs.rs/mpv/latest/mpv/).

**API:**

- `new(app_handle: tauri::AppHandle) -> Result<Self, String>` — create MpvHandler, set properties:
  - `gapless-audio` = `yes`
  - `video` = `no`  
  - `keep-open` = `no`
  - `observe_property` for `time-pos` (f64), `pause` (Flag bool), `duration` (f64)

- `load_file(&self, path: &str) -> Result<(), String>` — `command(&["loadfile", path])`
- `play(&self) -> Result<(), String>` — `set_property("pause", false)`
- `pause(&self) -> Result<(), String>` — `set_property("pause", true)`
- `toggle_play(&self) -> Result<(), String>`
- `seek(&self, seconds: f64) -> Result<(), String>` — `command(&["seek", &seconds.to_string(), "absolute"])`
- `set_volume(&self, vol: f64) -> Result<(), String>` — vol 0.0–1.0 → mpv 0–100
- `get_position(&self) -> Result<f64, String>`
- `get_duration(&self) -> Result<f64, String>`
- `get_state(&self) -> Result<String, String>` — "playing"/"paused"/"stopped"
- `stop(&self) -> Result<(), String>`

**Thread safety:** MpvHandler is !Send. Wrap in `SendHandler(MpvHandler)` with `unsafe impl Send` (libmpv is thread-safe). Store in `Arc<Mutex<SendHandler>>`.

**Event thread:** Poll `wait_event(0.2)`, emit Tauri events via `app_handle.emit()`:
- `mpv:playing` — StartFile
- `mpv:paused` — Pause/Unpause
- `mpv:stopped` — EndFile
- `mpv:position` {position: f64, duration: f64} — every 200ms via property polling

### 2. src-tauri/src/player/mod.rs

```rust
pub mod mpv;
pub mod atmos;
pub use mpv::MpvPlayer;
```

### 3. src-tauri/src/player/atmos.rs

Keep placeholder `pub fn init() {}`

### 4. src-tauri/src/lib.rs — Register Tauri commands

Commands: `play(tauri::AppHandle, State<Arc<MpvPlayer>>, path: String)`, `toggle_play`, `seek(seconds)`, `set_volume(volume)`, `get_playback_state`, `stop`

Return `Result<serde_json::Value, String>` for state. Manage player in `.manage()`.

### 5. Frontend - src/components/Player.tsx

- invoke/listen from @tauri-apps/api/core, @tauri-apps/api/event
- Play/Pause toggle, seek bar, volume slider, time display
- File drop zone + file picker button
- Listen for mpv:position, mpv:playing, mpv:paused, mpv:stopped events

### 6. Update src/App.tsx

Render `<Player />` component.

## Verification

1. `cd src-tauri && cargo check` — pass
2. `npm run build` — pass
