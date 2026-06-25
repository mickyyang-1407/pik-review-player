import { Component, createSignal, For, Show } from 'solid-js';
import { Note, Project, Version, updateNoteResolvedIn, exportCsv, exportText, exportMarkdown } from '../stores/notesStore';

interface NotesPanelProps {
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
}

function formatTimecode(ms: number) {
  const totalSeconds = Math.floor(ms / 1000);
  const minutes = Math.floor(totalSeconds / 60);
  const seconds = totalSeconds % 60;
  const milli = Math.floor((ms % 1000) / 10);
  return `${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}.${milli.toString().padStart(2, '0')}`;
}

const STATUS_COLORS: Record<Note['status'], string> = {
  open: 'rgba(244,241,234,0.4)',
  done: 'rgba(94,196,140,0.6)',
  ok: 'rgba(34,211,238,0.6)',
  needs_check: 'rgba(251,191,36,0.6)',
  rejected: 'rgba(239,68,68,0.6)'
};

const NoteItem: Component<{ note: Note, currentPosition: number, currentVersion: Version | null, onUpdateStatus: (id: number, status: Note['status']) => void, onDeleteNote: (id: number) => void, onSeekToNote: (ms: number) => void, versions: Version[] }> = (props) => {
  const isNearby = () => {
    if (props.note.timecodeMs === null) return false;
    const diff = Math.abs(props.note.timecodeMs / 1000 - props.currentPosition);
    return diff <= 2;
  };

  const isUnresolvedPrev = () => props.note.createdInVersionId !== props.currentVersion?.id && props.note.resolvedInVersionId === null;
  const isResolved = () => props.note.resolvedInVersionId !== null;

  const nextStatus = (current: Note['status']): Note['status'] => {
    const order: Note['status'][] = ['open', 'done', 'ok', 'needs_check', 'rejected'];
    const idx = order.indexOf(current);
    return order[(idx + 1) % order.length];
  };

  return (
    <div class="p-3 mb-2 rounded-lg border text-sm" style={{
      "background-color": "var(--bg-card)",
      "border-color": isNearby() ? "var(--accent)" : (isUnresolvedPrev() ? "rgba(251,191,36,0.8)" : "var(--border-soft)"),
      "box-shadow": isNearby() ? "var(--shadow-glow)" : "none"
    }}>
      <div class="flex justify-between items-start mb-2">
        <div class="flex items-center gap-2">
          <Show when={props.note.timecodeMs !== null}>
            <button 
              class="text-xs font-mono px-1.5 py-0.5 rounded bg-[var(--bg-hover)] text-[var(--text-secondary)] hover:text-[var(--accent)]"
              onClick={() => props.onSeekToNote(props.note.timecodeMs!)}
            >
              {formatTimecode(props.note.timecodeMs!)}
            </button>
          </Show>
          <span class="text-[10px] uppercase tracking-wider px-1.5 py-0.5 rounded border border-[var(--border-soft)] text-[var(--text-muted)]">
            {props.note.authorRole}
          </span>
          <Show when={props.note.createdInVersionId !== null}>
            <span class="text-[10px] tracking-wider px-1.5 py-0.5 rounded bg-[var(--bg-hover)] text-[var(--text-secondary)]">
              Created In: {props.versions.find(v => v.id === props.note.createdInVersionId)?.label || 'V?'}
            </span>
          </Show>
          <Show when={isResolved()}>
            <span class="text-[10px] tracking-wider px-1.5 py-0.5 rounded bg-[var(--bg-hover)] text-[var(--text-secondary)] flex items-center gap-1">
              <span class="text-green-400">✓</span>
              Resolved In: {props.versions.find(v => v.id === props.note.resolvedInVersionId)?.label || 'V?'}
            </span>
          </Show>
        </div>
        <div class="flex items-center gap-2">
          <button 
            class="text-[10px] uppercase px-1.5 py-0.5 rounded"
            style={{ "background-color": STATUS_COLORS[props.note.status], color: "var(--bg-primary)", "font-weight": "bold" }}
            onClick={() => props.onUpdateStatus(props.note.id, nextStatus(props.note.status))}
          >
            {props.note.status.replace('_', ' ')}
          </button>
          <button 
            class="text-[var(--text-muted)] hover:text-red-400"
            onClick={() => props.onDeleteNote(props.note.id)}
          >
            ×
          </button>
        </div>
      </div>
      <div class="text-[var(--text-primary)]">
        {props.note.body}
      </div>
    </div>
  );
};

const NotesPanel: Component<NotesPanelProps> = (props) => {
  const [newBody, setNewBody] = createSignal('');
  const [isGeneral, setIsGeneral] = createSignal(false);
  const [author, setAuthor] = createSignal<Note['authorRole']>('producer');
  const [showGeneral, setShowGeneral] = createSignal(false);
  const [filter, setFilter] = createSignal<'all' | 'open' | 'done' | 'needs_check' | 'unresolved_prev'>('all');
  const [showToast, setShowToast] = createSignal(false);

  const visibleNotes = () => {
    return props.notes.filter(n => {
      const isCurrentVersion = n.createdInVersionId === props.currentVersion?.id;
      const isUnresolvedPrev = !isCurrentVersion && n.resolvedInVersionId === null;
      const isResolvedCurrent = !isCurrentVersion && n.resolvedInVersionId === props.currentVersion?.id;

      if (!isCurrentVersion && !isUnresolvedPrev && !isResolvedCurrent) {
          return false;
      }

      if (filter() === 'open' && n.status !== 'open') return false;
      if (filter() === 'done' && n.status !== 'done') return false;
      if (filter() === 'needs_check' && n.status !== 'needs_check') return false;
      if (filter() === 'unresolved_prev' && !isUnresolvedPrev) return false;

      return true;
    });
  };

  const localGeneralNotes = () => visibleNotes().filter(n => n.timecodeMs === null);
  const localTimecodedNotes = () => visibleNotes().filter(n => n.timecodeMs !== null).sort((a, b) => a.timecodeMs! - b.timecodeMs!);

  const handleUpdateStatus = (id: number, status: Note['status']) => {
    props.onUpdateStatus(id, status);
    if ((status === 'done' || status === 'ok') && props.currentVersion) {
      updateNoteResolvedIn(id, props.currentVersion.id);
    } else {
      updateNoteResolvedIn(id, null);
    }
  };

  const handleSubmit = (e: Event) => {
    e.preventDefault();
    if (!newBody().trim()) return;
    props.onAddNote(newBody().trim(), isGeneral() ? null : props.currentPosition * 1000, author());
    setNewBody('');
  };

  return (
    <div class="w-[320px] h-full flex flex-col border-l border-[var(--border-soft)] bg-[var(--bg-secondary)] relative">
      <div class="p-4 border-b border-[var(--border-soft)] flex-shrink-0 flex flex-col gap-2">
        <div class="flex justify-between items-center relative">
          <h2 class="font-bold text-lg truncate" title={props.currentProject?.title || "Review Notes"}>
            {props.currentProject?.title || "Review Notes"}
          </h2>
          <Show when={showToast()}>
            <div class="absolute top-0 right-0 -mt-2 bg-green-500 text-white text-xs px-2 py-1 rounded shadow z-10">Copied!</div>
          </Show>
          <div class="flex items-center gap-2">
            <select 
              class="bg-[var(--bg-primary)] text-[var(--text-primary)] border border-[var(--border-soft)] rounded px-2 py-1 text-xs"
              onChange={async (e) => {
                const val = e.currentTarget.value;
                e.currentTarget.value = ""; // reset
                if (!val || !props.currentProject) return;
                try {
                  let text = "";
                  if (val === 'csv') text = await exportCsv(props.currentProject.id);
                  if (val === 'text') text = await exportText(props.currentProject.id);
                  if (val === 'md') text = await exportMarkdown(props.currentProject.id);
                  await navigator.clipboard.writeText(text);
                  
                  setShowToast(true);
                  setTimeout(() => setShowToast(false), 2000);
                } catch (err) {
                  console.error(err);
                }
              }}
            >
              <option value="">Export...</option>
              <option value="csv">Export CSV</option>
              <option value="text">Export Text</option>
              <option value="md">Export Markdown</option>
            </select>
            <button 
              class="text-xs bg-[var(--accent)] text-[var(--bg-primary)] px-2 py-1 rounded"
              onClick={props.onCreateVersion}
            >
              New Ver
            </button>
          </div>
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
      </div>

      <div class="flex-1 overflow-y-auto p-4 pb-32">
        <div class="mb-4 flex gap-2 text-xs overflow-x-auto pb-2 scrollbar-hide">
          <For each={['all', 'open', 'done', 'needs_check', 'unresolved_prev']}>
            {(f) => (
              <button 
                class={`px-2 py-1 rounded whitespace-nowrap ${filter() === f ? 'bg-[var(--accent)] text-[var(--bg-primary)]' : 'bg-[var(--bg-hover)] text-[var(--text-secondary)]'}`}
                onClick={() => setFilter(f as any)}
              >
                {f.replace('_', ' ').toUpperCase()}
              </button>
            )}
          </For>
        </div>

        <div class="mb-4">
          <button 
            class="w-full text-left font-semibold text-[var(--text-secondary)] py-2 flex justify-between items-center"
            onClick={() => setShowGeneral(!showGeneral())}
          >
            <span>General Notes ({localGeneralNotes().length})</span>
            <span>{showGeneral() ? '▼' : '▶'}</span>
          </button>
          <Show when={showGeneral()}>
            <For each={localGeneralNotes()}>
              {(note) => (
                <NoteItem 
                  note={note} 
                  currentPosition={props.currentPosition}
                  currentVersion={props.currentVersion}
                  onUpdateStatus={handleUpdateStatus}
                  onDeleteNote={props.onDeleteNote}
                  onSeekToNote={props.onSeekToNote}
                  versions={props.versions}
                />
              )}
            </For>
          </Show>
        </div>

        <div>
          <h3 class="font-semibold text-[var(--text-secondary)] py-2 mb-2">Timecoded Notes ({localTimecodedNotes().length})</h3>
          <For each={localTimecodedNotes()}>
            {(note) => (
              <NoteItem 
                note={note} 
                currentPosition={props.currentPosition}
                currentVersion={props.currentVersion}
                onUpdateStatus={handleUpdateStatus}
                onDeleteNote={props.onDeleteNote}
                onSeekToNote={props.onSeekToNote}
                versions={props.versions}
              />
            )}
          </For>
        </div>
      </div>

      {/* Input Area */}
      <div class="absolute bottom-0 left-0 right-0 p-4 bg-[var(--bg-tertiary)] border-t border-[var(--border-soft)]">
        <form onSubmit={handleSubmit} class="flex flex-col gap-2">
          <div class="flex justify-between items-center">
            <select 
              class="bg-[var(--bg-primary)] text-[var(--text-primary)] border border-[var(--border-soft)] rounded px-2 py-1 text-xs"
              value={author()}
              onChange={(e) => setAuthor(e.currentTarget.value as Note['authorRole'])}
            >
              <option value="producer">Producer</option>
              <option value="artist">Artist</option>
              <option value="manager">Manager</option>
              <option value="mixer">Mixer</option>
              <option value="client">Client</option>
            </select>
            <label class="flex items-center gap-1 text-xs text-[var(--text-secondary)]">
              <input 
                type="checkbox" 
                checked={isGeneral()} 
                onChange={(e) => setIsGeneral(e.currentTarget.checked)} 
              />
              General
            </label>
          </div>
          <div class="relative">
            <input 
              id="note-input"
              type="text" 
              placeholder={isGeneral() ? "Add general note..." : `Add note at ${formatTimecode(props.currentPosition * 1000)}`}
              class="w-full bg-[var(--bg-primary)] border border-[var(--border-soft)] rounded p-2 text-sm text-[var(--text-primary)] focus:outline-none focus:border-[var(--accent)]"
              value={newBody()}
              onInput={(e) => setNewBody(e.currentTarget.value)}
            />
          </div>
        </form>
      </div>
    </div>
  );
};

export default NotesPanel;
