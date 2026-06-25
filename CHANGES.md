# Phase 4 & 5 Changes

## Phase 4: Version Workflow & Note Resolution States
*   **Rust Backend**:
    *   Exposed and registered the `review_update_note_resolved_in` command in `src-tauri/src/lib.rs` to allow updating the `resolved_in_version_id` of a note.
*   **Frontend Data Layer** (`src/stores/notesStore.ts`):
    *   Added `updateNoteResolvedIn` function to optimistically update and invoke the backend command.
*   **Frontend UI** (`src/components/NotesPanel.tsx`):
    *   Updated the `NotesPanel` state logic to automatically resolve notes (`updateNoteResolvedIn`) to the current version when their status changes to `done` or `ok`, and unresolve them otherwise.
    *   Introduced a filter state (`All`, `Open`, `Done`, `Needs Check`, `Unresolved_prev`) to easily switch between note subsets.
    *   Modified the `NoteItem` component to show an explicit "Created In: V[X]" label and conditionally a "Resolved In: V[Y]" label with a green tick when resolved.
    *   Added styling (yellow border) for notes carried over from a previous version that are not yet resolved.
    *   Dynamically computed visible notes combining the current version notes, notes resolved within the current version, and unresolved notes from older versions.

## Phase 5: Export Functionality
*   **Rust Backend**:
    *   Added `review_export_csv`, `review_export_text`, and `review_export_markdown` commands in `src-tauri/src/lib.rs` to fetch and serialize notes to the respective formats (CSV, plain text, and Markdown).
    *   Registered the export commands into the Tauri invoke handler.
*   **Frontend Data Layer** (`src/stores/notesStore.ts`):
    *   Added `exportCsv`, `exportText`, and `exportMarkdown` functions to wrap the backend Tauri commands.
*   **Frontend UI** (`src/components/NotesPanel.tsx`):
    *   Added an "Export..." dropdown in the NotesPanel header.
    *   Implemented a clipboard copy mechanism directly pushing the fetched exported string to `navigator.clipboard.writeText`.
    *   Added a small temporary "Copied!" toast feedback.
- Phase 6: Implemented advanced meter glow, peak blink red, S/M tooltips, and Group selection for SpeakerRoom.

## Final Validation — 2026-06-25

- `cargo check`: PASS (0.24s, dev profile, 0 errors)
- `npm run build`: PASS (vite 6.4.3, 447ms, 0 errors, 0 warnings)
- `npm run tauri build -- --bundles app`: PASS (Rust release 51.56s, bundle at `src-tauri/target/release/bundle/macos/Pik Review.app`)
- All 6 phases confirmed in codebase:
  - Phase 1: SpeakerRoom, NotesPanel, PlayerBar, Transport UI
  - Phase 2: SQLite schema (projects/versions/notes) + CRUD in `src-tauri/src/database/mod.rs`
  - Phase 3: notesStore backend integration, auto project/version creation
  - Phase 4: V1/V2 note comparison, resolved_in_version_id, note filter states
  - Phase 5: CSV/Text/Markdown export commands + clipboard copy in NotesPanel
  - Phase 6: Solo/Mute, Group selection, meter glow, peak blink red in SpeakerRoom
