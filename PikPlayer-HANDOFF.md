# Pik Player Handoff

## Handoff — 2026-06-24 — Pro Review Player Concept Captured

### Current Phase
Captured Micky's new product direction for a professional review/player spinoff based on Pik Player's current playback foundation.

### Completed
- Added `SPINOFF-01：Pik Player Pro / Review Player` to `PikPlayer-PLAN.md`.
- Created a standalone new-project prompt at `PROFESSIONAL-PLAYER-PROMPT.md`.
- The concept is a pro music review / mix revision / approval player, not a consumer library player.

### Core Concept
- Drag in a track or mix version, play with hotkeys and timeline clicks.
- Right-side Notes panel supports General Notes and Timecoded Notes.
- Timecoded notes are created at current playback position via hotkey or timeline click, e.g. `01:23:14 小鼓太刺耳，可以小一點`.
- Top speaker-room visualization should look like a professional 7.1.4 / Dolby-style monitoring room, with real-time meter glow driven by playback volume.
- Speaker or group can be clicked for Solo / Mute.
- Version memory: V1 notes remain visible when V2 is imported/played; reviewer can mark notes `Done` / `OK` when fixed.
- Export targets: CSV, plain text, Markdown first; PDF / DAW marker export later.

### Next Action
When Micky wants to start the new project, use `PROFESSIONAL-PLAYER-PROMPT.md` as the opening prompt and fork/copy the current Pik Player codebase as the foundation.

---

## Handoff — 2026-06-24 20:52 — Codex Pullback & M4 Verification

### Current Phase
Pulled M4's latest GitHub work into this machine and verified the Phase 7 UI/cover-art/playback changes. Local `main` now contains the remote commits plus the existing local Cowart auto-backup commits via merge commit `36a992f`.

### Completed
- Fetched `origin/main` from GitHub; remote was `62ca3bb` with two M4 commits: UI/asset protocol polish and AlbumGrid click-to-play fix.
- Merged `origin/main` into local `main` without overwriting the seven local Cowart auto-backup commits.
- Confirmed M4's new handoff: 3-block PlayerBar, embedded cover art via Tauri asset protocol, AlbumGrid real covers, no auto-play on load, explicit `playback_play`, and MP3/WAV/M4A/AAC/AIFF routing through AtmosPlayer for vDSP metering.
- Removed two accidental committed scratch files from the M4 merge: `src-tauri/check.err` and `src-tauri/test_lofty.rs`.
- Preserved the pre-merge local Cowart timestamp-only dirty change in `stash@{0}` as `codex-temp-cowart-view-state`.

### Files Changed This Session
- `PikPlayer-HANDOFF.md`
- `src-tauri/check.err` deleted
- `src-tauri/test_lofty.rs` deleted
- `~/STATUS.md`

### Tests Run
- `rtk npm run build` ✅
- `rtk npm run tauri build -- --bundles app` ✅ after allowing Cargo to download new `http-range` dependency required by Tauri `protocol-asset`
- `rtk rg ... src src-tauri --glob '!target/**'` ✅ no project-side `AVPlayerView`, `AVPlayerLayer`, `AVKit`, `content_view_ptr`, `ns_window()`, `get_webview_window`, `addSubview`, or `setContentView`
- `rtk strings ... | rtk rg 'AVPlayerView|AVPlayerLayer|AVKit|/assets/index-|playback:state|asset://'` ✅ current Vite assets present; no AVPlayerView/AVKit strings found
- `rtk /usr/bin/otool -L ...` ✅ dependency inspection; AppKit/WebKit are still Tauri/Wry windowing, not project AV view code
- `rtk /usr/bin/plutil -lint .../Info.plist` ✅
- `rtk /usr/bin/codesign --force --deep --sign - ... && rtk /usr/bin/codesign --verify --deep --strict --verbose=2 ...` ✅

### Important Results
- Fresh app bundle exists at `src-tauri/target/release/bundle/macos/Pik Player.app` and passes ad-hoc codesign verification.
- First sandboxed Tauri build failed only because the new Tauri `protocol-asset` feature needed to fetch `http-range` from crates.io; escalated build succeeded.
- Worktree is clean except for this handoff/status update and the deliberate deletion of the two scratch files.

### Next Action
1. Run or open exactly `src-tauri/target/release/bundle/macos/Pik Player.app`.
2. Manually verify: MP3 cover art, AlbumGrid click-to-play, no auto-play on drag/open, vDSP meter movement, EQ toggle, output dropdown, normal WAV, Atmos MP4/WAV.
3. If manual tests pass, commit/push this merge + scratch-file cleanup so GitHub contains the verified combined history.

### Warning For Next Agent
- Do not reintroduce `ns_window`, `content_view_ptr`, `AVPlayerView`, or `AVPlayerLayer`.
- Do not pop `stash@{0}` unless Micky specifically wants the Cowart canvas timestamp restored; it only changes `canvas/cowart-view-state.json.updatedAt`.

---

## Handoff — 2026-06-24 — Antigravity UI Polish & Asset Protocol Fix

### Current Phase
M4 Iteration: Refined the UI to a 3-block layout, fixed the embedded cover art extraction and display by enabling Tauri v2 asset protocols, mapped MP3 playback to AtmosPlayer for vDSP metering, and disabled auto-play on load.

### Completed
- **3-Block Layout**: Modified `PlayerBar.tsx` to feature Left (Cover/Info), Center (Playback/Progress), and Right (EQ/Meter/Output) sections.
- **Embedded Cover Art Display**: Fixed the Tauri v2 asset protocol by adding `protocol-asset` to `Cargo.toml` and `assetProtocol: { enable: true, scope: ["**"] }` to `tauri.conf.json`, allowing the frontend to load covers from `~/Library/Caches/pik-player/covers`.
- **Album Grid Cover Art**: Modified `AlbumGrid.tsx` to display real cover art instead of generic discs.
- **Stop Auto-Play & Playlist Play Bug Fix**: Changed the default playback state on file load to `Paused`. To ensure clicking the `AlbumGrid` correctly plays the song instead of getting stuck in Paused (which caused the vDSP meter to stick at stereo), a new `playback_play` command was added and wired into `App.tsx`'s `loadFile(path, autoPlay)`.
- **MP3 Metering (vDSP)**: Routed MP3, WAV, M4A, AAC, and AIFF files to use `AtmosPlayer` via `detect_engine` in `playback.rs`, enabling accurate real-time metering.

### Files Changed
- `PikPlayer-TASKS.md`
- `src/components/PlayerBar.tsx`
- `src/components/AlbumGrid.tsx`
- `src-tauri/tauri.conf.json`
- `src-tauri/Cargo.toml`
- `src-tauri/capabilities/default.json`
- `src-tauri/src/player/playback.rs`

### Tests Run
- `npm run tauri build -- --bundles app` ✅
- Manual tests performed by the user to verify MP3 metering, auto-play prevention, and cover art display.

### Next Action
Wait for further user feedback on the album grid cover display and UI aesthetics.

---
## Handoff — 2026-06-24 00:10 — Codex Root-Cause Fix

### Current Phase
`.app` old AVPlayerView/black-screen investigation resolved at code/bundle level. The remaining visual evidence points to Tauri/Wry WebView internals only, not project-side native AV view attachment.

### Completed
- Removed the stale native-view API shape from Atmos playback: `atmos_create` no longer accepts a `content_view_ptr` / `ns_window` pointer.
- Removed Rust-side lookup of `window.ns_window()` before creating the Atmos player.
- Removed direct `AVKit` / `AppKit` imports and build-script framework links from the project Atmos wrapper path.
- Left Atmos playback as audio-only `AVPlayer` + `MTAudioProcessingTap` EQ/meter path.
- Cleaned release build warnings from unused Obj-C callback parameters.

### Files Changed
- `src-tauri/src/player/atmos.rs`
- `src-tauri/src/player/atmos_wrapper.m`
- `src-tauri/build.rs`
- `PikPlayer-HANDOFF.md`
- `~/STATUS.md`

### Tests Run
- `rtk npm run build` ✅
- `rtk npm run tauri build -- --bundles app` ✅ clean release build
- `rtk /usr/bin/codesign --force --deep --sign - 'src-tauri/target/release/bundle/macos/Pik Player.app'` ✅
- `rtk /usr/bin/codesign --verify --deep --strict --verbose=2 'src-tauri/target/release/bundle/macos/Pik Player.app'` ✅
- `rtk /usr/bin/otool -L 'src-tauri/target/release/bundle/macos/Pik Player.app/Contents/MacOS/pik-player'` ✅ dependency inspection
- `rtk strings ... | rtk rg 'AVPlayerView|AVPlayerLayer|AVKit|/assets/index-|playback:state'` ✅ no `AVPlayerView` / `AVPlayerLayer` / `AVKit`; current Vite asset refs present
- `rtk rg ... src src-tauri --glob '!target/**'` ✅ no project-side `AVPlayerView`, `AVPlayerLayer`, `content_view_ptr`, `ns_window()`, `get_webview_window`, `addSubview`, or `setContentView`
- Direct launch of exact bundle binary `./src-tauri/target/release/bundle/macos/Pik Player.app/Contents/MacOS/pik-player` ✅ stayed alive for smoke window launch, then stopped with Ctrl-C

### Results
- Fresh bundle binary timestamp: `Jun 24 00:06:33 2026`.
- Fresh signature identifier after ad-hoc sign: `com.mickydigitalstudio.pikplayer`.
- `otool -L` still lists `AppKit.framework`, but this is pulled by Tauri/Wry/WebKit windowing; project build no longer links `AVKit`, and source/binary checks show no project AVPlayerView path.
- `strings` still finds `addSubview` / `setContentView` only from Tauri/Wry/objc2 WebView internals, not from project code.
- `screencapture` validation was attempted but failed with `could not create image from display`, so visual screenshot proof could not be collected in this sandbox.

### Root Cause
The project no longer contained an actual AVPlayerView attachment, but the Atmos boundary still preserved the old native-view design: Rust passed `ns_window` into `atmos_create`, Obj-C accepted `content_view_ptr`, and the wrapper imported/linked AVKit/AppKit. That stale interface made future/native-view regressions easy and made bundle diagnostics ambiguous. The fix removes that root cause by making Atmos creation audio-only and window-agnostic.

### Next Action
1. Open exactly `src-tauri/target/release/bundle/macos/Pik Player.app` from Finder or run the exact binary above.
2. If a black/old window still appears, it is no longer from project-side AVPlayerView code; investigate LaunchServices opening another bundle or WebView asset load/runtime CSS instead.
3. Continue manual media tests: MP3, normal WAV, Atmos MP4/WAV, output device dropdown, EQ toggle.

### Warning For Next Agent
- Do not reintroduce `ns_window`, `content_view_ptr`, `AVPlayerView`, or `AVPlayerLayer` into Atmos playback. UI must remain Solid/Tauri; Atmos backend should stay audio-only.

---

## Handoff — 2026-06-23 — Codex AVPlayerView Diagnostic Pause

### Current Phase
Diagnosing why the latest built `.app` appears to show the old all-black AVPlayerView-style screen instead of the redesigned Solid/Tauri frontend. Work paused by Micky to restart Codex with `--ask-for-approval never`.

### Evidence Collected
- Read `~/STATUS.md` and continued the active `pik-player` goal.
- Rebuilt frontend with `rtk npm run build` ✅.
- Rebuilt release app bundle with `rtk npm run tauri build -- --bundles app` ✅.
- Fresh bundle path: `src-tauri/target/release/bundle/macos/Pik Player.app`.
- Fresh binary timestamp after rebuild: `Jun 23 23:15:22 2026`.
- `rtk strings` on fresh app binary found **no `AVPlayerView` / `AVPlayerLayer`**.
- `rtk strings` did find `/assets/index-Bcly-RvD.js` and `/assets/index-jyDoLAeL.css`, meaning the rebuilt binary embeds the current Vite frontend asset references.
- `rtk strings` still finds `addSubview`, `contentView`, `setContentView`, and `WKWebView`, but only from Tauri/Wry/objc2 WebKit/AppKit internals, not project AVPlayer code.
- Source search found no project `AVPlayerView`, `AVPlayerLayer`, `addSubview`, or `setContentView` usage outside Tauri internals.
- `src-tauri/src/player/atmos_wrapper.m` currently has `atmos_create(void* content_view_ptr, ...)` but immediately `(void)content_view_ptr;` and only creates an `AVPlayer`; it does **not** attach any native view to the window.
- Spotlight/project search found only one `Pik Player.app` under this repo: `src-tauri/target/release/bundle/macos/Pik Player.app`.
- Process checks via `rtk ps aux | rtk rg ...` are unreliable in sandbox: non-escalated calls return `Operation not permitted`; one escalated simple check returned no matching stale `pik-player/tauri/vite` process at that moment.

### Important Current Observation
- `git status --short` currently shows only `canvas/cowart-view-state.json` modified. The expected uncommitted changes listed in the active goal (`src-tauri/src/player/atmos_wrapper.m`, `src/components/PlayerBar.tsx`) are not currently dirty in this worktree, so the next session must treat the worktree as source of truth and verify whether those edits were already committed, overwritten, or never present in this checkout.

### Commands Run This Pause Session
- `rtk npm run build` ✅
- `rtk npm run tauri build -- --bundles app` ✅
- `rtk strings 'src-tauri/target/release/bundle/macos/Pik Player.app/Contents/MacOS/pik-player' | rtk rg ...` ✅
- `rtk rg ... src src-tauri --glob '!target/**'` ✅
- `rtk /usr/bin/find ... -name 'Pik Player.app' ...` ✅

### Next Best Actions After Restart
1. Start Codex with `codex --ask-for-approval never` if Micky wants unattended execution.
2. Re-run process checks with sufficient permission: `rtk ps -axo pid,ppid,comm,args | rtk rg -i 'pik-player|Pik Player|tauri|vite|MacOS/pik'`.
3. Kill any stale Pik Player/Tauri/Vite process if found.
4. Re-sign the freshly rebuilt app bundle if needed: `rtk codesign --force --deep --sign - 'src-tauri/target/release/bundle/macos/Pik Player.app'`.
5. Verify signature: `rtk codesign --verify --deep --strict --verbose=2 'src-tauri/target/release/bundle/macos/Pik Player.app'`.
6. Launch the exact binary or app bundle and visually verify the redesigned frontend, not the old black native AVPlayer view.
7. If visual verification still shows black/old UI, inspect whether WebView failed to load embedded assets or whether launch is using a different executable via LaunchServices/cache.

### Current Root-Cause Hypothesis
The available evidence currently argues **against** project AVPlayerView/native-view code being present in the fresh binary. The likely remaining causes are: stale/running process, LaunchServices/cache opening a different executable, WebView asset load failure presenting a blank/black window, or mismatch between the app Micky launched and the freshly rebuilt bundle.

---

## Handoff — 2026-06-23 — Codex Verification

### Current Phase
UI Redesign verification. Code-level and bundle-level checks pass; GUI playback manual test still not completed because this session did not expose a Computer Use control tool and shell GUI launch escalation was rejected.

### Completed This Session
- Read `~/STATUS.md` and latest handoff.
- Verified repository state was clean before checks.
- Re-ran frontend and Rust checks.
- Built a release `.app` bundle successfully with `npm run tauri build -- --bundles app`.
- Confirmed generated app bundle path: `src-tauri/target/release/bundle/macos/Pik Player.app`.
- Re-signed the local app bundle with ad-hoc signature: `codesign --force --deep --sign - ...`.
- Verified bundle signature after re-sign: `codesign --verify --deep --strict --verbose=2 ...` passes.
- Checked app binary dependencies with `otool -L`; app links to Homebrew mpv at `/opt/homebrew/opt/mpv/lib/libmpv.2.dylib` plus system frameworks.
- Confirmed `Info.plist` is valid.

### Tests Run
- `rtk npm run build` ✅
- `rtk cargo check` in `src-tauri` ✅
- `rtk npm run tauri build -- --bundles app` ✅
- `rtk codesign --verify --deep --strict --verbose=2 'src-tauri/target/release/bundle/macos/Pik Player.app'` ✅ after ad-hoc re-sign
- `rtk plutil -lint 'src-tauri/target/release/bundle/macos/Pik Player.app/Contents/Info.plist'` ✅
- `rtk otool -L 'src-tauri/target/release/bundle/macos/Pik Player.app/Contents/MacOS/pik-player'` ✅ dependency inspection

### Important Results
- `.app` bundle is available and passes code-sign verification after local ad-hoc re-sign.
- Full `npm run tauri build` with default `targets: "all"` still fails only at DMG packaging (`bundle_dmg.sh`), after the `.app` has already been built. Use `npm run tauri build -- --bundles app` for a clean app-bundle build until DMG packaging is investigated.
- Project contains no sample media files, so automated media import/playback could not be exercised from local fixtures.

### Still Not Done
- Real GUI launch test.
- Real MP3 playback test.
- Real Atmos MP4 playback test.
- Import Folder through native dialog.
- Output device dropdown visual confirmation.
- EQ ON/OFF runtime test.

### Next Action
Use Computer Use or run locally:
```bash
cd ~/Projects/pik-player
npm run tauri dev
```
Then manually test the checklist from the previous entry.

### Warning
- This session's tool list did not include actual Computer Use control calls despite the skill being available, so GUI clicks/screenshots could not be performed here.
- Direct shell GUI launch with escalated permissions was rejected by policy in this environment.
- The `.app` depends on Homebrew mpv dylibs under `/opt/homebrew/opt/...`; it is suitable for this machine, but not yet a fully self-contained distributable app.

---

## Handoff — 2026-06-23 — Claude Code

### Current Phase
UI Redesign + EQ Bug Fixes (COMPLETE at code level — NEVER manually tested)

### Completed This Session

**agy bug fixes（agy 寫的 EQ 有兩個 critical bug，已修）：**
- `build.rs`：補 `MediaToolbox` + `Accelerate` framework link — 沒有這兩條 `cargo check` 過但 `tauri dev` link 階段會 undefined symbol crash
- `atmos_wrapper.m` line 119：`vDSP_biquad` delay buffer 傳了 `vDSP_biquad_Setup *` 當 `float *`，型別錯誤 runtime 會 crash/memory corruption。修法：`EQBandContext` 加 `float delay[8][4]`（per-channel delay state），改傳 `context->bands[i].delay[ch]`

**UI Redesign（全面換新）：**
- `src/styles/theme.css`：新色系 deep navy-black `#08080f` + cyan accent `#22d3ee`，加 glow/shadow CSS 變數
- `src/components/PlayerBar.tsx`：完全重寫 — 高度 78px；進度條移到最頂端 flush + gradient glow；Play 用 SVG icon + 發光陰影；backdrop-filter blur 玻璃質感；timestamps 在按鈕下方
- `src/components/AlbumGrid.tsx`：完全重寫 — hover `translateY(-2px)` 上浮 + 陰影；封面用格式專屬漸層（Atmos 藍紫、FLAC 深綠、WAV 深棕、MP3 深紫）；active glow border
- `src/components/Sidebar.tsx`：Import Folder + Open File 移入側邊欄（解決遮擋 soundtrack 問題）；按鈕用 accent 色設計
- `src/App.tsx`：移除 header 的按鈕行；Sidebar 接受 `onImportFolder/onOpenFile/importing` props；import progress 重新設計；player bar 高度改 `pb-[78px]`；empty state 更精緻

### Files Changed
- `src-tauri/build.rs`
- `src-tauri/src/player/atmos_wrapper.m`
- `src/styles/theme.css`
- `src/components/PlayerBar.tsx`
- `src/components/AlbumGrid.tsx`
- `src/components/Sidebar.tsx`
- `src/App.tsx`

### Tests Run
- `cargo check` ✅
- `npm run build` ✅

### CRITICAL: 從未手動測試
這個 app 從 Phase 1 到現在從來沒有跑過。所有功能都是 code-level 驗證，不代表 runtime 正確。

### Next Action（下一棒第一件事）
```bash
cd ~/Projects/pik-player
npm run tauri dev
```

**手動測試清單：**
1. App 開起來不 crash
2. 拖 MP3 → grid 出現 → 點擊 → PlayerBar 顯示 → 有聲音
3. 拖 Atmos MP4 → ATMOS badge → 有聲音
4. Import Folder（Sidebar 按鈕）→ progress → 曲目出現
5. SearchBox 篩選
6. Hover → × → 刪除
7. Output device dropdown 有真實裝置
8. EQ ON/OFF 不 crash
9. 進度條拖曳 seek 正確

**常見 crash 方向：**
- AVFoundation crash → `atmos_wrapper.m` 的 EQ tap 在 asset 未 load 完時初始化
- mpv 無聲音 → mpv dylib 路徑問題（build env 需 `LIBRARY_PATH=/opt/homebrew/opt/mpv/lib`）

### Warning
- `Sidebar` 現在需要 3 個 props（`onImportFolder`, `onOpenFile`, `importing`）
- PlayerBar 高度 78px → App content `pb-[78px]`
- `EQBandContext.delay[8][4]` 是 per-channel delay state

---

## Handoff — 2026-06-22 — Claude Code

### Current Phase
Phase 5 — Output Device Selection (COMPLETE at code level)

### Completed
- **atmos_wrapper.m**: Added `#import <CoreAudio/CoreAudio.h>`; new functions: `audio_list_output_devices()` (returns malloc'd JSON of all output-capable devices with uid/name/isDefault), `free_audio_devices_json()`, `atmos_set_output_device()` (sets `AVPlayer.audioOutputDeviceUniqueID`).
- **build.rs**: Added `cargo:rustc-link-lib=framework=CoreAudio`.
- **player/atmos.rs**: Added `atmos_set_output_device`, `audio_list_output_devices`, `free_audio_devices_json` to `extern "C"` block (all `pub`). Added `AtmosPlayer::set_output_device(&self, uid: &str)`.
- **player/mpv.rs**: Added `MpvPlayer::set_audio_device(&self, uid: &str)` — sets `audio-device coreaudio/<uid>` via mpv command, or `auto` when uid is empty.
- **player/playback.rs**: Added `output_device_uid: Option<String>` to `PlaybackState`. Added `PlaybackPlayer::set_output_device(uid)` — routes to active engine, stores uid for persistence. On `load()` success, re-applies stored device uid to new engine instance.
- **lib.rs**: Added `AudioDevice { uid, name, isDefault }` struct. Added `output_list_devices` and `output_set_device` commands. Both registered in `invoke_handler`.
- **App.tsx**: Added `AudioDevice` interface, `audioDevices`/`currentDeviceUid` signals, `refreshDevices()` + `setOutputDevice()` functions called on mount. Passed as props to PlayerBar.
- **PlayerBar.tsx**: Added `For` import from solid-js. Extended `PlayerBarProps` with device props. Added `<select>` dropdown in the right section: "System Default" + `<For>` over devices; triggers `onDeviceChange` on change.
- `cargo check` ✅  `npm run build` ✅

### Files Changed
- `src-tauri/src/player/atmos_wrapper.m`
- `src-tauri/build.rs`
- `src-tauri/src/player/atmos.rs`
- `src-tauri/src/player/mpv.rs`
- `src-tauri/src/player/playback.rs`
- `src-tauri/src/lib.rs`
- `src/App.tsx`
- `src/components/PlayerBar.tsx`
- `PikPlayer-TASKS.md`
- `PikPlayer-HANDOFF.md`

### Design Decisions
- CoreAudio device listing in Obj-C (not Rust FFI crate) — consistent with existing atmos_wrapper.m pattern, no new crates needed.
- Device uid is stored in `PlaybackState.outputDeviceUid` and re-applied on every track load — so switching device mid-session persists across tracks.
- mpv uses `coreaudio/<CoreAudio_UID>` format; AtmosPlayer uses `AVPlayer.audioOutputDeviceUniqueID` directly. Same UID string works for both.
- "System Default" option = uid `""`, which sends `None` / `auto` to both engines.

### Blockers
- Needs manual test in running app: device list should show real CoreAudio output devices (built-in, headphones, etc.); selecting one should route audio immediately.

### Next Action
1. `npm run tauri dev` → check device dropdown shows real devices.
2. Connect headphones / USB DAC → verify they appear in dropdown.
3. Select device while a track is playing → verify audio routes to new device.
4. Phase 5 ideas next: cover art rendering in AlbumGrid, keyboard shortcuts (space = play/pause), Now Playing info panel.

### Warning For Next Agent
- `audio_list_output_devices()` is `pub extern "C"` in `atmos.rs` — it's the C function from the Obj-C file, NOT a Rust fn. Call it via `use player::atmos::audio_list_output_devices`.
- The Obj-C file uses `malloc`/`strdup` for the JSON string; must call `free_audio_devices_json(ptr)` after reading it, or you leak.
- `AVPlayer.audioOutputDeviceUniqueID` requires macOS 10.9+ (we target 11.0+, so safe).
- mpv `audio-device coreaudio/<uid>` format: the UID string from CoreAudio must match exactly.

## Current Status

- Current owner: Codex
- Current phase: Phase 2 — Trustworthy Playback Status
- Phase 0 rotation documents: complete
- Phase 1 unified playback core: complete at build/check level
- Main objective: make playback states/error/output labels more trustworthy and manually verify real media behavior.

## Decision Log

- 2026-06-22: Keep `MpvPlayer` and `AtmosPlayer` intact for Phase 1. Add a facade instead of rewriting audio engines.
- 2026-06-22: Keep legacy `player_*` / `atmos_*` commands registered until a later cleanup.
- 2026-06-22: Treat Spatial Audio actual status as `unknown` for now. Do not pretend the app can verify it yet.
- 2026-06-22: Frontend now uses only `playback_*` commands and `playback:*` events. Legacy events still exist internally for compatibility but are not UI dependencies.

## Handoff — 2026-06-22 Start — Codex

### Current Phase
Phase 1 — Unified Playback Core

### Completed
- Read `~/STATUS.md`.
- Checked repo state.
- Created `PikPlayer-AGENTS.md`.
- Created `PikPlayer-TASKS.md`.
- Created `PikPlayer-HANDOFF.md`.

### In Progress
- Backend playback facade.

### Files Changed
- `PikPlayer-AGENTS.md`
- `PikPlayer-TASKS.md`
- `PikPlayer-HANDOFF.md`

### Tests Run
- None yet in this handoff block.

### Results
- Phase 0 is documented.

### Blockers
- None.

### Decisions Made
- First implementation target is unified playback, not Library or UI redesign.

### Next Action
1. Add `src-tauri/src/player/playback.rs`.
2. Register unified playback commands in `src-tauri/src/lib.rs`.
3. Update `src/App.tsx` to use unified commands/events.

### Warning For Next Agent
- `pik-player-planning-notes.md` was already modified before this implementation session. Do not revert it.

## Handoff — 2026-06-22 — Codex

### Current Phase
Phase 2 — Trustworthy Playback Status

### Completed
- Phase 0 rotation docs created:
  - `PikPlayer-AGENTS.md`
  - `PikPlayer-TASKS.md`
  - `PikPlayer-HANDOFF.md`
- Added backend unified playback facade:
  - `src-tauri/src/player/playback.rs`
  - commands: `playback_load`, `playback_toggle_play_pause`, `playback_stop`, `playback_seek`, `playback_set_volume`, `playback_get_state`
  - events: `playback:state`, `playback:position`, `playback:error`, `playback:ended`
- Kept legacy `player_*` and `atmos_*` commands registered.
- Updated `MpvPlayer` with clone support, explicit `play`, `pause`, `position`, and `is_paused`.
- Updated `AtmosPlayer` with clone support, `position`, and `is_playing`.
- Updated `App.tsx` to call only unified playback commands and listen only to unified playback events.
- Updated `PlayerBar` to show status and playback mode labels.
- Added a simple empty state below the current demo grid.
- Verified frontend no longer directly invokes old playback commands/events with `rg`.

### In Progress
- Phase 2 status polish and manual playback verification.

### Files Changed
- `PikPlayer-AGENTS.md`
- `PikPlayer-TASKS.md`
- `PikPlayer-HANDOFF.md`
- `src-tauri/src/player/playback.rs`
- `src-tauri/src/player/mod.rs`
- `src-tauri/src/player/mpv.rs`
- `src-tauri/src/player/atmos.rs`
- `src-tauri/src/lib.rs`
- `src/App.tsx`
- `src/components/PlayerBar.tsx`
- Existing earlier change: `pik-player-planning-notes.md`

### Tests Run
- `rtk cargo check` in `src-tauri`
- `rtk npm run build`
- `rtk rg "invoke<.*\\('(player_|atmos_)|invoke\\('(player_|atmos_)|listen<.*\\('(mpv:|av:)|listen\\('(mpv:|av:)" src`

### Results
- `cargo check` passed.
- `npm run build` passed.
- `rg` found no frontend direct calls/listeners for legacy playback commands/events.

### Blockers
- No real audio files were manually tested in the GUI during this turn.
- `playback:ended` currently emits on explicit `playback_stop`; natural end detection still depends on future facade polish, because legacy `mpv:ended` / `av:ended` are not bridged into the facade yet.
- Spatial Audio actual output detection is still intentionally `unknown`.

### Decisions Made
- Phase 1 is considered complete for code structure and build/check verification.
- Next phase should focus on trustworthy status, manual playback tests, and bridging natural ended/error events if needed.

### Next Action
1. Manually test MP3 and Atmos MP4/ADM WAV in the app.
2. Bridge natural engine-ended events into `playback:ended` or add observer-based ended detection in `PlaybackPlayer`.
3. Improve the empty state so it does not appear below the hardcoded demo grid once Library work begins.
4. Start Phase 2 task: better error categories and clearer mode/status copy.

### Warning For Next Agent
- Do not remove legacy commands yet; they are intentionally kept for compatibility.
- `AlbumGrid` is still hardcoded demo data. Do not mistake it for Library progress.
- `playback_mode = "unknown"` for Atmos is deliberate until Web Research proves reliable Spatial Audio detection.

## Handoff — 2026-06-22 — Claude Code

### Current Phase
Phase 2 — Trustworthy Playback Status

### Completed
- Natural ended bridging: `mpv:ended` and `av:ended` are now bridged into `playback:ended` via `bridge_engine_events()` in `PlaybackPlayer::init`.
- Error classification: three distinct categories — "File not found", "Unsupported format (.ext)", "Load failed" — replacing raw engine errors.
- Empty state improvement: `AlbumGrid` (hardcoded demo data) removed from main layout; empty state is now the primary UI, showing drag target + file format list.
- Dragging highlight moved into the empty state card itself (border + bg tint on `tauri://drag-enter`).
- `cargo check` ✅  `npm run build` ✅

### In Progress
- Phase 2 is functionally complete at code level. Manual media tests not yet done in running app.

### Files Changed
- `src-tauri/src/player/playback.rs` — bridge_engine_events, improved load errors
- `src/App.tsx` — empty state as primary UI, AlbumGrid commented out
- `PikPlayer-TASKS.md` — Phase 2 tasks marked DONE
- `PikPlayer-HANDOFF.md` — this entry

### Tests Run
- `/Users/mickyyang/.cargo/bin/cargo check` in `src-tauri`
- `npm run build` in project root

### Results
- `cargo check` passed.
- `npm run build` passed (15 modules, 379ms).

### Blockers
- Manual playback test with real MP3/Atmos files still needed (requires running GUI app).
- `AtmosPlayer::start_observer` emits `av:ended` when `pos >= dur - 0.1 && is_playing`. This should work but has not been confirmed on real Atmos content.

### Decisions Made
- Phase 2 is considered complete for code. Manual test matrix is the remaining acceptance step.
- AlbumGrid component is NOT deleted — just commented-out import. Phase 3 will un-comment and wire real DB.

### Next Action
1. Run `npm run tauri dev` and manually test:
   a. Open MP3 → verify plays, progress moves, ends cleanly → PlayerBar shows "Stopped"
   b. Open Atmos MP4 → verify Atmos label appears, plays, ends cleanly
   c. Open non-existent path (via terminal invoke test) → "File not found"
   d. Drag .xyz file → "Unsupported format (.xyz)"
2. If manual tests pass, update Manual Media Test Matrix in `PikPlayer-TASKS.md`.
3. Start Phase 3: persist opened tracks to DB, render real tracks in grid.

### Warning For Next Agent
- `AlbumGrid` import in `App.tsx` is commented out (`// import AlbumGrid from './components/AlbumGrid';`). Phase 3 will restore it.
- Legacy `player_*` and `atmos_*` Tauri commands remain registered — do not remove.
- `playback_mode` for Atmos is still "unknown" — deliberate.
- `AtmosPlayer::bridge_engine_events` uses `app.listen("av:ended", ...)` which listens to events emitted by `AtmosPlayer::start_observer`. Both run concurrently — this is fine but be aware of the double-observer pattern if debugging.

## Handoff — 2026-06-22 — Claude Code

### Current Phase
Phase 3 — Library Alpha (partial)

### Completed
- `database/mod.rs`: Added `#[derive(Clone, Serialize)]` + `#[serde(rename_all = "camelCase")]` to Track; query now orders by `added_at DESC`.
- `lib.rs`: `playback_load` now scans metadata via `scanner::scan_file()` and inserts into DB before loading into player. Metadata scan failure uses filename as title (non-fatal).
- `lib.rs`: Added 3 new Tauri commands: `library_get_tracks`, `library_search_tracks`, `library_remove_track`.
- `AlbumGrid.tsx`: Completely rewritten from hardcoded demo data to props-driven (`tracks`, `currentPath`, `onPlay`). Highlights active track, Atmos badge for mp4/adm/atmos files, duration display, emoji cover by format.
- `App.tsx`: Added `tracks` signal + `refreshLibrary()`. Library loaded on mount; refreshed after each `loadFile`. Grid shown when tracks exist; empty state shown when empty. Active track highlighted via `filePath` signal.
- `cargo check` ✅ `npm run build` ✅ (16 modules)

### In Progress
- Search (SearchBox not yet wired to `library_search_tracks`)
- Track deletion from UI
- Manual playback test still needed in running GUI

### Files Changed
- `src-tauri/src/database/mod.rs`
- `src-tauri/src/lib.rs`
- `src/components/AlbumGrid.tsx`
- `src/App.tsx`
- `PikPlayer-TASKS.md`
- `PikPlayer-HANDOFF.md`

### Tests Run
- `/Users/mickyyang/.cargo/bin/cargo check` ✅
- `npm run build` ✅

### Blockers
- Manual test with real audio files not yet done (needs running GUI app).

### Decisions Made
- `playback_load` adds to DB before loading player. If scan fails, uses filename as title with `duration=0`. DB duration is display-only; real duration comes from player during playback.
- `library_get_tracks` returns tracks `ORDER BY added_at DESC` ("Recently Added" UX).
- Atmos detection in grid: format=mp4 OR path contains "adm"/"atmos" → ATMOS badge.

### Next Action
1. Wire SearchBox to `library_search_tracks`: on input, call the command and update `tracks()`.
2. Add right-click context menu or inline delete button on AlbumGrid items calling `library_remove_track(id)`.
3. Run `npm run tauri dev` and manually test the full flow:
   a. Drop MP3 → appears in grid → click → plays
   b. Drop Atmos MP4 → ATMOS badge in grid → plays
   c. SearchBox filters results
4. Update manual test matrix after confirming.

### Warning For Next Agent
- `Track.id` in DB starts at 1 (SQLite AUTOINCREMENT). The `id=0` passed to `insert()` is a placeholder — SQLite ignores it.
- `INSERT OR IGNORE` on same path means re-opening the same file does NOT update its metadata. This is intentional for Phase 3; update logic can come later.
- SearchBox component is now wired — it has `onSearch` prop and 250ms debounce.
- Do NOT change the Rust Track struct field names — they are `snake_case` in Rust, `camelCase` in JSON (via `#[serde(rename_all = "camelCase")]`), and `camelCase` in the TS `TrackRow` interface.

## Handoff — 2026-06-22 — Claude Code

### Current Phase
Phase 4 — Import & ADM Sidecar (partial)

### Completed
- **Delete track**: AlbumGrid now accepts `onDelete` prop; hover reveals red × button; `e.stopPropagation()` prevents accidental play; `App.tsx` calls `library_remove_track(id)` + `refreshLibrary()`.
- **Folder import backend**: `library_import_folder(path)` command walks directory recursively, filters audio extensions, scans each file, inserts into DB, emits `library:import-progress` per file.
- **Folder import frontend**: "Import Folder…" button opens folder picker; `importing` + `importProgress` signals drive progress bar (scanned/total + current filename); button disabled while importing; bar hidden when done.
- **ADM sidecar schema**: documented as JSON comment in `scanner/mod.rs` (fields: title, artist, album, year, track, cover, atmos, bed_channels, object_count, binaural_render).
- `cargo check` ✅ `npm run build` ✅

### In Progress
- Cover fallback (cover.jpg / folder.jpg detection)
- ADM sidecar reader (parsing metadata.json to override lofty tags)
- Manual playback test still pending (needs running GUI)

### Files Changed
- `src/components/AlbumGrid.tsx` — delete button on hover
- `src/App.tsx` — importFolder(), deleteTrack(), progress state, listener
- `src-tauri/src/lib.rs` — library_import_folder + find_audio_files + ImportProgress struct
- `src-tauri/src/scanner/mod.rs` — ADM sidecar schema comment
- `PikPlayer-TASKS.md` — Phase 3 + Phase 4 tasks updated
- `PikPlayer-HANDOFF.md` — this entry

### Tests Run
- `/Users/mickyyang/.cargo/bin/cargo check` ✅
- `npm run build` ✅ (16 modules, 28.10 kB JS)

### Blockers
- `library_import_folder` is synchronous (blocks Tauri thread until all files scanned). For large folders (1000+ files), this will freeze the UI. Phase 5 should move it to `async` with `tauri::async_runtime::spawn`.
- No manual test with real audio yet.

### Decisions Made
- `library_import_folder` emits `library:import-progress` per file (not batched). Fine for typical music folders (<500 files).
- Cover fallback and ADM sidecar reader deferred — no visible UI impact yet since covers are emoji.

### Next Action
1. Run `npm run tauri dev` and manually test:
   a. Import Folder → progress bar appears → library fills with tracks
   b. Search → filters tracks
   c. Hover card → × appears → click → track removed
   d. Open File + drag-drop still work
2. If folder import is too slow for large folders, make `library_import_folder` async:
   ```rust
   #[tauri::command]
   async fn library_import_folder(...) -> Result<u32, String> { ... }
   ```
3. Implement ADM sidecar reader: in `scan_file`, check for `<stem>.metadata.json` or `metadata.json` in same dir, parse JSON, override title/artist/album.
4. Implement cover fallback: scan for `cover.jpg`, `folder.jpg`, `artwork.jpg` in same dir; store relative path in a new `cover_path` field in Track.

### Warning For Next Agent
- `library_import_folder` is currently synchronous — safe for small folders, but block-prone for large ones.
- AlbumGrid uses `.map()` instead of Solid's `<For>` — fine for library sizes up to a few hundred tracks. Switch to `<For>` when approaching 500+ tracks.
- ADM sidecar schema is defined in `scanner/mod.rs` comments only — not yet implemented as code.

## Handoff — 2026-06-22 — Claude Code (autonomous review session)

### Current Phase
Phase 4 — Import & ADM Sidecar (COMPLETE) + Bug Fixes

### Completed
- Spawned 2 code-review subagents + 1 clippy background job in parallel.
- **Clippy**: 3 `new_without_default` warnings only (style, not bugs).
- **Rust review findings fixed**:
  - `find_audio_files` symlink loop → `entry.metadata()` (no symlink follow), skips symlinks silently.
  - `library_import_folder` sync blocking → `async fn` + `tauri::async_runtime::spawn_blocking`; `Arc<Database>` cloned before move.
  - TOCTOU in `playback.rs:201-213` → deferred; noted that Arc keeps both engines alive, no crash risk.
- **Frontend review findings fixed**:
  - `AlbumGrid.tsx`: `.map()` → `<For each={props.tracks}>` — fixes hoveredId reset on refresh and `active()` reactivity when `currentPath` changes independently.
  - `SearchBox.tsx`: added `onCleanup(() => clearTimeout(debounce))`.
- **Scanner enhancements**:
  - `scan_file` now returns `cover_path: Option<String>` (cover.jpg / folder.jpg / artwork.jpg detection).
  - ADM WAV sidecar reader: reads `<stem>.metadata.json` or `metadata.json`, overrides title/artist/album/cover.
- **Database**: added `cover_path TEXT` column; `ALTER TABLE ... ADD COLUMN` migration runs on startup (idempotent).
- `cargo check` ✅ `npm run build` ✅

### Files Changed
- `src-tauri/src/lib.rs` — symlink fix, async import, cover_path in Track construction
- `src-tauri/src/scanner/mod.rs` — SidecarMeta, read_sidecar(), find_cover(), Metadata.cover_path
- `src-tauri/src/database/mod.rs` — cover_path field + migration
- `src/components/AlbumGrid.tsx` — .map() → <For>, coverPath in TrackRow interface
- `src/components/SearchBox.tsx` — onCleanup debounce
- `PikPlayer-TASKS.md` — Phase 4 + Bug Fixes section
- `~/STATUS.md` — updated to Phase 4 complete

### Deferred
- `playback.rs:201-213` concurrent `load()` TOCTOU — low risk (Arc-safe), defer to Phase 5.
- `AlbumGrid` cover image display — `coverPath` is now in `TrackRow` but UI still uses emoji. Phase 5: render actual `<img>` when `coverPath` is non-null using Tauri's asset protocol.

### Next Action
1. Run `npm run tauri dev` and test manually:
   - Drop MP3 → appears in grid with title/artist → click → plays → progress bar moves → ends → Stopped
   - Drop Atmos MP4 → ATMOS badge → plays
   - Import Folder → progress bar → all tracks appear
   - SearchBox → filters tracks reactively (now using <For>, should be smooth)
   - Hover card → × button → click → track removed from grid
   - Error: non-existent path → "File not found" in PlayerBar
2. Phase 5 ideas: cover art rendering, Now Playing info panel, keyboard shortcuts (space = play/pause), drag-to-seek scrubbing.

### Warning For Next Agent
- `library_import_folder` is now async — the handler is declared `async fn`. If you add new sync commands, keep them `fn` (not `async fn`) unless they do I/O.
- `Track.cover_path` column was added via `ALTER TABLE ... ADD COLUMN` migration — existing DBs get the column added automatically on next app start.
- `AlbumGrid` now uses `<For>` — do NOT revert to `.map()`. The <For> pattern is required for correct Solid reactivity.
- The sidecar reader only runs for `.wav` files. If you add support for other formats, add sidecar logic in `scan_file` for those extensions too.

## Handoff — 2026-06-22 — Claude Code (AutoEQ Start)

### Current Phase
Phase 6 — AutoEQ Integration (partial)

### Completed
- Added `eq.rs` module to define `EqProfile` and `EqBand` structs, with JSON deserialization logic.
- Implemented `to_mpv_af_string()` to convert EqProfile to `af=lavfi=[anequalizer=...]` format.
- Modified `PlaybackState` in `playback.rs` to track `eq_profile` and `eq_enabled`.
- Added `playback_set_eq_enabled` and `playback_set_eq_profile` Tauri commands.
- Updated `PlayerBar.tsx` and `App.tsx` to include an EQ toggle button. State is managed by SolidJS signals and synchronized with Tauri backend.
- `cargo check` ✅ `npm run build` ✅

### In Progress
- Headphone detection logic in `atmos_wrapper.m`.
- `AtmosPlayer` EQ support via `AVAudioUnitEQ` or `MTAudioProcessingTap`.

### Files Changed
- `src-tauri/src/player/eq.rs` (new)
- `src-tauri/src/player/mod.rs`
- `src-tauri/src/player/mpv.rs`
- `src-tauri/src/player/playback.rs`
- `src-tauri/src/lib.rs`
- `src/App.tsx`
- `src/components/PlayerBar.tsx`
- `PikPlayer-TASKS.md`
- `PikPlayer-HANDOFF.md`

### Next Action
1. Test MPV EQ by manually loading a test JSON profile.
2. Figure out how to attach `AVAudioUnitEQ` to an `AVPlayer` in Objective-C.

## Handoff — 2026-06-22 — Claude Code (AutoEQ Continued)

### Current Phase
Phase 6 — AutoEQ Integration (COMPLETE)

### Completed
- Added headphone detection logic in `atmos_wrapper.m` via `audio_is_headphone_connected()`.
- Exposed headphone detection in `atmos.rs` and added `output_is_headphone_connected` Tauri command in `lib.rs`.
- Implemented automatic EQ prompting toast in `App.tsx` which appears when headphones are detected and EQ is currently disabled.
- Implemented `AtmosPlayer` EQ support using `MTAudioProcessingTap` and Apple's vDSP framework for biquad filtering in `atmos_wrapper.m`.
- Exposed `atmos_set_eq` in `atmos.rs` to allow passing the parsed EQ JSON directly to the Objective-C tap context.
- Wired `playback.rs` to correctly pass EQ state changes to `AtmosPlayer`.
- Marked all Phase 6 tasks as DONE in `PikPlayer-TASKS.md`.

### In Progress
- Phase 6 code is fully complete. Remaining items are manual tests.

### Next Action
1. Test MPV EQ by manually loading a test JSON profile via UI.
2. Test AtmosPlayer EQ by playing an Atmos source, toggling EQ, and verifying the vDSP tap works without crashes or latency.
3. Proceed to Phase 7 or any UI polishes.
