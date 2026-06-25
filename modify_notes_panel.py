import re

with open("src/components/NotesPanel.tsx", "r") as f:
    content = f.read()

# Replace Note imports and Add Project/Version
content = content.replace("import { Note, generalNotes, timecodedNotes } from '../stores/notesStore';", "import { Note, generalNotes, timecodedNotes, Project, Version } from '../stores/notesStore';")

# Update NotesPanelProps
props_new = """interface NotesPanelProps {
  notes: Note[];
  onAddNote: (body: string, timecodeMs: number | null, authorRole: Note['authorRole']) => void;
  onUpdateStatus: (id: number, status: Note['status']) => void;
  onDeleteNote: (id: number) => void;
  onSeekToNote: (timecodeMs: number) => void;
  currentPosition: number; // seconds
  currentProject: Project | null;
  currentVersion: Version | null;
  versions: Version[];
  onVersionSelect: (v: Version) => void;
  onCreateVersion: () => void;
}"""
content = re.sub(r'interface NotesPanelProps \{.*?\n\}', props_new, content, flags=re.DOTALL)

# Update NoteItem props
content = content.replace("const NoteItem: Component<{ note: Note, currentPosition: number, onUpdateStatus: (id: string, status: Note['status']) => void, onDeleteNote: (id: string) => void, onSeekToNote: (ms: number) => void }> = (props) => {", 
                          "const NoteItem: Component<{ note: Note, currentPosition: number, onUpdateStatus: (id: number, status: Note['status']) => void, onDeleteNote: (id: number) => void, onSeekToNote: (ms: number) => void, versions: Version[] }> = (props) => {")

# Show version tag in NoteItem
note_item_old = """          <span class="text-[10px] uppercase tracking-wider px-1.5 py-0.5 rounded border border-[var(--border-soft)] text-[var(--text-muted)]">
            {props.note.authorRole}
          </span>"""
note_item_new = """          <span class="text-[10px] uppercase tracking-wider px-1.5 py-0.5 rounded border border-[var(--border-soft)] text-[var(--text-muted)]">
            {props.note.authorRole}
          </span>
          <Show when={props.note.createdInVersionId !== null}>
            <span class="text-[10px] tracking-wider px-1.5 py-0.5 rounded bg-[var(--bg-hover)] text-[var(--text-secondary)]">
              {props.versions.find(v => v.id === props.note.createdInVersionId)?.label || 'V?'}
            </span>
          </Show>"""
content = content.replace(note_item_old, note_item_new)

# Update header with version selector
header_old = """      <div class="p-4 border-b border-[var(--border-soft)] flex-shrink-0 flex justify-between items-center">
        <h2 class="font-bold text-lg">Review Notes</h2>
      </div>"""
header_new = """      <div class="p-4 border-b border-[var(--border-soft)] flex-shrink-0 flex flex-col gap-2">
        <div class="flex justify-between items-center">
          <h2 class="font-bold text-lg truncate" title={props.currentProject?.title || "Review Notes"}>
            {props.currentProject?.title || "Review Notes"}
          </h2>
          <button 
            class="text-xs bg-[var(--accent)] text-[var(--bg-primary)] px-2 py-1 rounded"
            onClick={props.onCreateVersion}
          >
            New Ver
          </button>
        </div>
        <Show when={props.versions.length > 0}>
          <select 
            class="bg-[var(--bg-primary)] text-[var(--text-primary)] border border-[var(--border-soft)] rounded px-2 py-1 text-xs"
            value={props.currentVersion?.id || ''}
            onChange={(e) => {
              const id = parseInt(e.currentTarget.value, 10);
              const v = props.versions.find(ver => ver.id === id);
              if (v) props.onVersionSelect(v);
            }}
          >
            <For each={props.versions}>
              {(v) => <option value={v.id}>{v.label} - {new Date(v.importedAt).toLocaleTimeString()}</option>}
            </For>
          </select>
        </Show>
      </div>"""
content = content.replace(header_old, header_new)

# Pass versions to NoteItem
content = content.replace("onSeekToNote={props.onSeekToNote}", "onSeekToNote={props.onSeekToNote}\n                  versions={props.versions}")

with open("src/components/NotesPanel.tsx", "w") as f:
    f.write(content)
