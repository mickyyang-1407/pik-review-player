# Pik Review Plan

## Product

- **Product name**: Pik Review
- **Repository / folder**: `pik-review-player`
- **Bundle identifier**: `com.mickydigitalstudio.pikreview`
- **Package name**: `pik-review-player`
- **Rust crate / binary**: `pik-review`

Pik Review is a professional mix review player for timecoded notes, version approvals, and immersive playback feedback.

## Phase 0 Scope

Phase 0 is limited to project fork, identity rename, clean repository setup, build verification, GitHub setup, and handoff documentation.

Phase 0 must not implement the Phase 1–6 product features.

## Foundation

- Base code copied from `/Users/mickyyang/Projects/pik-player`.
- Keep Tauri v2 + SolidJS + Rust playback foundation.
- Keep audio-only AVFoundation / AtmosPlayer / vDSP meter / output device / EQ foundation.
- Do not reintroduce `AVPlayerView`, `AVPlayerLayer`, `ns_window`, `content_view_ptr`, or native video view attachment.
- UI remains Solid/Tauri; no native AppKit UI rewrite.

## App Data Isolation

- Tauri identifier is changed to `com.mickydigitalstudio.pikreview`, creating separate app data from Pik Player.
- SQLite library file is renamed to `pik-review-library.db`.
- Embedded cover cache path is renamed to `pik-review/covers`.

## MVP Direction

- Local-first SQLite project/version/note workflow.
- Timecoded notes and general notes.
- Version review workflow for V1/V2/V3 approvals.
- CSV, plain text, and Markdown export.
- 7.1.4-style speaker room visualization with solo/mute controls in a later phase.

## Deferred Phases

- **Phase 1**: Review shell UI with speaker room, transport/timeline, and notes panel.
- **Phase 2**: Project / Version / Note SQLite schema and CRUD.
- **Phase 3**: Timecoded note workflow and keyboard shortcuts.
- **Phase 4**: Version workflow and note resolution states.
- **Phase 5**: CSV, plain text, and Markdown export.
- **Phase 6**: Speaker room v1 with meter glow and solo/mute controls.

