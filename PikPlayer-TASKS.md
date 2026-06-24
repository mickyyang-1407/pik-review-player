# Pik Player Task Board

Statuses: `TODO`, `IN_PROGRESS`, `DONE`, `BLOCKED`.

## Phase 0 — Rotation Documents

| Status | Task | Owner Role | Files | Acceptance |
| --- | --- | --- | --- | --- |
| DONE | Create agent rules | Supervisor | `PikPlayer-AGENTS.md` | Codex/Claude/agy roles and handoff rules are documented. |
| DONE | Create task board | Supervisor | `PikPlayer-TASKS.md` | Phase tasks are visible and status-tracked. |
| DONE | Create handoff log | Supervisor | `PikPlayer-HANDOFF.md` | Next model can continue without chat history. |

## Phase 1 — Unified Playback Core

| Status | Task | Owner Role | Files | Acceptance |
| --- | --- | --- | --- | --- |
| DONE | Add backend playback facade | Backend Playback | `src-tauri/src/player/playback.rs`, `src-tauri/src/player/mod.rs`, `src-tauri/src/lib.rs` | Unified playback commands compile and wrap mpv/Atmos. |
| DONE | Emit unified playback events | Backend Playback | `src-tauri/src/player/playback.rs` | Frontend can listen only to `playback:*` events. |
| DONE | Wire frontend to unified commands | Frontend UX | `src/App.tsx` | No direct `player_*` or `atmos_*` invokes remain in `App.tsx`. |
| DONE | Update PlayerBar status UI | Frontend UX | `src/components/PlayerBar.tsx` | Loading/error/source/mode states are visible without a redesign. |
| DONE | Run build/check verification | Reviewer / QA | repo | `npm run build` and `cargo check` pass or failures are documented. |

## Phase 2 — Trustworthy Playback Status

| Status | Task | Owner Role | Files | Acceptance |
| --- | --- | --- | --- | --- |
| DONE | Add explicit playback mode labels | Frontend UX | `src/App.tsx`, `src/components/PlayerBar.tsx` | UI distinguishes Atmos source from actual/unknown playback mode. |
| DONE | Improve load/error states | Backend Playback, Frontend UX | `src-tauri/src/player/playback.rs` | Missing/unsupported/load failed states are user-readable. |
| DONE | Add empty state | Frontend UX | `src/App.tsx` | Empty library invites drag/drop or Open File. |
| DONE | Natural ended bridging | Backend Playback | `src-tauri/src/player/playback.rs` | `mpv:ended` and `av:ended` bridged to `playback:ended`. |

## Phase 3 — Library Alpha

| Status | Task | Owner Role | Files | Acceptance |
| --- | --- | --- | --- | --- |
| DONE | Persist opened tracks | Library & Metadata | `src-tauri/src/database/mod.rs`, `src-tauri/src/lib.rs` | `playback_load` scans metadata + inserts into DB. |
| DONE | Render real tracks | Frontend UX, Library & Metadata | `src/components/AlbumGrid.tsx`, `src/App.tsx` | Grid uses DB data; tracks() drives display vs empty state. |
| DONE | Search DB | Library & Metadata | `src/components/SearchBox.tsx`, `src/App.tsx`, `lib.rs` | SearchBox debounces `library_search_tracks`; empty query restores full list. |
| DONE | Delete track from library | Frontend UX | `AlbumGrid.tsx`, `App.tsx` | Hover reveals × button; calls `library_remove_track`. |

## Phase 4 — Import & ADM Sidecar

| Status | Task | Owner Role | Files | Acceptance |
| --- | --- | --- | --- | --- |
| DONE | Define sidecar schema | Library & Metadata | `src-tauri/src/scanner/mod.rs` | JSON schema comment documents ADM WAV `metadata.json` fields. |
| DONE | Folder import with progress | Library & Metadata, Frontend UX | `src-tauri/src/lib.rs`, `src/App.tsx` | "Import Folder…" button opens folder picker; progress bar shown; `library:import-progress` events drive scanned/total display. |
| DONE | Cover fallback | Library & Metadata | `src-tauri/src/scanner/mod.rs`, `src-tauri/src/database/mod.rs` | cover.jpg/folder.jpg/artwork.jpg detected; absolute path stored in `Track.cover_path`. |
| DONE | ADM sidecar reader | Library & Metadata | `src-tauri/src/scanner/mod.rs` | `<stem>.metadata.json` / `metadata.json` parsed; title/artist/album/cover override lofty tags for WAV files. |

## Bug Fixes (from code review)

| Status | Bug | Severity | Fix |
| --- | --- | --- | --- |
| DONE | `find_audio_files` symlink loop → stack overflow | HIGH | Use `entry.metadata()` (no symlink follow) instead of `p.is_dir()` |
| DONE | `library_import_folder` blocks Tauri thread | MEDIUM | Made `async` + `tauri::async_runtime::spawn_blocking` |
| DONE | `AlbumGrid` `.map()` breaks hover reset + active reactivity | MEDIUM | Replaced with `<For each={props.tracks}>` |
| DONE | `SearchBox` debounce timer leaks on unmount | LOW | Added `onCleanup(() => clearTimeout(debounce))` |
| DEFERRED | `playback.rs:201-213` concurrent `load()` TOCTOU | MEDIUM | Both engines use Arc so no crash; documented in HANDOFF |

## Phase 5 — Output Device Selection

| Status | Task | Owner Role | Files | Acceptance |
| --- | --- | --- | --- | --- |
| DONE | CoreAudio device list (Obj-C) | Backend Playback | `atmos_wrapper.m` | `audio_list_output_devices()` returns JSON array of output-capable devices |
| DONE | mpv output device switch | Backend Playback | `player/mpv.rs` | `set_audio_device(uid)` sets `audio-device coreaudio/<uid>` |
| DONE | AtmosPlayer output device switch | Backend Playback | `player/atmos.rs`, `atmos_wrapper.m` | `set_output_device(uid)` sets `AVPlayer.audioOutputDeviceUniqueID` |
| DONE | PlaybackPlayer facade | Backend Playback | `player/playback.rs` | `set_output_device(uid)` routes to active engine; re-applied on each track load; stored in `PlaybackState.outputDeviceUid` |
| DONE | Tauri commands | Backend Playback | `lib.rs` | `output_list_devices` + `output_set_device` registered and compile |
| DONE | Frontend device selector | Frontend UX | `App.tsx`, `components/PlayerBar.tsx` | Device `<select>` in PlayerBar right section; "System Default" option; changes take effect immediately |
| TODO | Manual test | QA | — | Open app → device dropdown lists real CoreAudio output devices → select headphones → audio routes correctly |

## Manual Media Test Matrix

- TODO MP3
- TODO AAC / M4A
- TODO FLAC
- TODO OGG
- TODO normal WAV
- TODO ADM / Atmos WAV
- TODO Atmos MP4
- DONE missing file → shows "File not found" in PlayerBar error area
- TODO damaged file
- TODO CJK filename
- DONE unsupported format (.xyz) → shows "Unsupported format (.xyz)"

## Phase 6 — AutoEQ Integration

| Status | Task | Owner Role | Files | Acceptance |
| --- | --- | --- | --- | --- |
| DONE | Headphone detection logic | Backend | `atmos_wrapper.m` | Can query if current CoreAudio output is a headphone. |
| DONE | EQ UI Toggle | Frontend | `PlayerBar.tsx` | EQ ON/OFF button is visible in PlayerBar. |
| DONE | MpvPlayer EQ support | Backend | `mpv.rs` | Can inject `af=lavfi=[anequalizer=...]` based on EQ parameters. |
| DONE | AtmosPlayer EQ support | Backend | `atmos.rs`, `atmos_wrapper.m` | Can apply EQ via `MTAudioProcessingTap` and vDSP. |
| DONE | AutoEQ JSON Parser | Backend | `eq.rs` | Can read and parse AutoEQ json files for frequency, Q, gain. |

## Phase 7 — M4 Iteration & UI Polish

| Status | Task | Owner Role | Files | Acceptance |
| --- | --- | --- | --- | --- |
| DONE | 3-Block Layout | Frontend UX | `PlayerBar.tsx` | Layout reorganized to Left, Center, Right with inline Meter. |
| DONE | Embedded Cover Art Extract | Library & Metadata | `scanner/mod.rs` | Uses `lofty` to extract embedded cover art into local cache. |
| DONE | Fix Tauri Asset Protocol | Backend | `tauri.conf.json`, `capabilities/default.json` | Asset protocol enabled to load cover art from cache dir. |
| DONE | AlbumGrid Cover Art | Frontend UX | `AlbumGrid.tsx` | Grid correctly displays `track.coverPath` instead of default disc. |
| DONE | Stop Auto-Play on Drop | Backend Playback | `playback.rs` | Loading tracks enters `Paused` state instead of auto-playing. |
| DONE | Route MP3 to AVPlayer | Backend Playback | `playback.rs` | Detects MP3/WAV/M4A to use `AtmosPlayer`, granting vDSP metering to MP3. |
