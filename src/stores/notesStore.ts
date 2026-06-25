import { createSignal } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';

export interface Note {
  id: number;
  projectId: number;
  createdInVersionId: number | null;
  resolvedInVersionId: number | null;
  timecodeMs: number | null; // null = general note
  body: string;
  status: 'open' | 'done' | 'ok' | 'needs_check' | 'rejected';
  authorRole: 'producer' | 'artist' | 'manager' | 'mixer' | 'client';
  createdAt: string;
  updatedAt: string;
}

export interface Project {
  id: number;
  title: string;
  client: string | null;
  artist: string | null;
  album: string | null;
  cue: string | null;
  createdAt: string;
  updatedAt: string;
}

export interface Version {
  id: number;
  projectId: number;
  label: string;
  filePath: string;
  duration: number | null;
  importedAt: string;
  checksum: string | null;
}

const [notes, setNotes] = createSignal<Note[]>([]);

export const generalNotes = () => notes().filter(n => n.timecodeMs === null);
export const timecodedNotes = () => notes().filter(n => n.timecodeMs !== null).sort((a, b) => a.timecodeMs! - b.timecodeMs!);

export async function loadNotes(projectId: number) {
  try {
    const fetched = await invoke<Note[]>('review_get_notes', { projectId });
    setNotes(fetched);
  } catch (e) {
    console.warn('Failed to load notes', e);
  }
}

export async function addNote(projectId: number, createdInVersionId: number | null, body: string, timecodeMs: number | null, authorRole: Note['authorRole'] = 'producer') {
  try {
    const newNote = await invoke<Note>('review_create_note', {
      projectId,
      createdInVersionId,
      timecodeMs,
      body,
      authorRole,
      status: 'open'
    });
    setNotes(prev => [...prev, newNote]);
  } catch (e) {
    console.warn('Failed to add note', e);
  }
}

export async function updateNoteStatus(id: number, status: Note['status']) {
  // Optimistic update
  setNotes(prev => prev.map(n => n.id === id ? { ...n, status } : n));
  try {
    await invoke('review_update_note_status', { id, status });
  } catch (e) {
    console.warn('Failed to update note status', e);
  }
}

export async function updateNoteResolvedIn(id: number, resolvedInVersionId: number | null) {
  setNotes(prev => prev.map(n => n.id === id ? { ...n, resolvedInVersionId } : n));
  try {
    await invoke('review_update_note_resolved_in', { id, resolvedInVersionId });
  } catch (e) {
    console.warn('Failed to update note resolved in', e);
  }
}

export async function deleteNote(id: number) {
  // Optimistic update
  setNotes(prev => prev.filter(n => n.id !== id));
  try {
    await invoke('review_delete_note', { id });
  } catch (e) {
    console.warn('Failed to delete note', e);
  }
}

export async function exportCsv(projectId: number): Promise<string> {
  return await invoke('review_export_csv', { projectId });
}

export async function exportText(projectId: number): Promise<string> {
  return await invoke('review_export_text', { projectId });
}

export async function exportMarkdown(projectId: number): Promise<string> {
  return await invoke('review_export_markdown', { projectId });
}

export { notes, setNotes };
