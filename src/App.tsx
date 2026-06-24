import { createSignal, onMount, onCleanup } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { open } from '@tauri-apps/plugin-dialog';

import Sidebar from './components/Sidebar';
import SearchBox from './components/SearchBox';
import FilterRow from './components/FilterRow';
import AlbumGrid, { type TrackRow } from './components/AlbumGrid';
import PlayerBar from './components/PlayerBar';

interface PositionPayload {
  position: number;
  duration: number;
}

interface PlaybackState {
  engine: 'mpv' | 'atmos' | null;
  status: 'idle' | 'loading' | 'playing' | 'paused' | 'stopped' | 'error';
  filePath: string | null;
  fileName: string | null;
  isAtmosSource: boolean;
  playbackMode: string;
  position: number;
  duration: number;
  volume: number;
  error: string | null;
  eqEnabled: boolean;
}

interface ImportProgress {
  scanned: number;
  total: number;
  current: string;
}

interface AudioDevice {
  uid: string;
  name: string;
  isDefault: boolean;
}

interface MeterChannel {
  label: string;
  rms: number;
  peak: number;
}

interface MeterPayload {
  available: boolean;
  mode: 'stereo' | 'multichannel' | 'unavailable' | string;
  channels: MeterChannel[];
}

export default function App() {
  const [file, setFile] = createSignal('');
  const [filePath, setFilePath] = createSignal('');
  const [position, setPosition] = createSignal(0);
  const [duration, setDuration] = createSignal(0);
  const [status, setStatus] = createSignal<PlaybackState['status']>('idle');
  const [volume, setVolume] = createSignal(100);
  const [error, setError] = createSignal('');
  const [dragging, setDragging] = createSignal(false);
  const [isAtmos, setIsAtmos] = createSignal(false);
  const [playbackMode, setPlaybackMode] = createSignal('unknown');
  const [tracks, setTracks] = createSignal<TrackRow[]>([]);
  const [importing, setImporting] = createSignal(false);
  const [importProgress, setImportProgress] = createSignal<ImportProgress | null>(null);
  const [audioDevices, setAudioDevices] = createSignal<AudioDevice[]>([]);
  const [currentDeviceUid, setCurrentDeviceUid] = createSignal<string>('');
  const [eqEnabled, setEqEnabled] = createSignal(false);
  const [showEqPrompt, setShowEqPrompt] = createSignal(false);
  const [meter, setMeter] = createSignal<MeterPayload>({ available: false, mode: 'unavailable', channels: [] });

  const currentTrack = () => tracks().find((track) => track.path === filePath()) ?? null;

  async function refreshDevices() {
    try {
      const devices = await invoke<AudioDevice[]>('output_list_devices');
      setAudioDevices(devices);
      
      // Check headphone connection and prompt for EQ if not already enabled
      const isHeadphone = await invoke<boolean>('output_is_headphone_connected');
      if (isHeadphone && !eqEnabled()) {
        setShowEqPrompt(true);
      } else {
        setShowEqPrompt(false);
      }
    } catch {
      // non-fatal
    }
  }

  async function setOutputDevice(uid: string) {
    try {
      await invoke('output_set_device', { uid });
      setCurrentDeviceUid(uid);
    } catch {
      // non-fatal
    }
  }

  async function toggleEq() {
    const next = !eqEnabled();
    try {
      await invoke('playback_set_eq_enabled', { enabled: next });
      setEqEnabled(next);
    } catch (e) {
      setError(String(e));
    }
  }

  async function refreshLibrary() {
    try {
      const result = await invoke<TrackRow[]>('library_get_tracks');
      setTracks(result);
    } catch {
      // non-fatal
    }
  }

  async function handleSearch(query: string) {
    try {
      if (query.trim() === '') {
        refreshLibrary();
      } else {
        const result = await invoke<TrackRow[]>('library_search_tracks', { query });
        setTracks(result);
      }
    } catch {
      // non-fatal
    }
  }

  async function deleteTrack(id: number) {
    try {
      await invoke('library_remove_track', { id });
      refreshLibrary();
    } catch {
      // non-fatal
    }
  }

  async function importFolder() {
    try {
      const selected = await open({ directory: true, multiple: false });
      if (!selected || typeof selected !== 'string') return;
      setImporting(true);
      setImportProgress({ scanned: 0, total: 0, current: '' });
      await invoke('library_import_folder', { path: selected });
      refreshLibrary();
    } catch (e) {
      setError(String(e));
    } finally {
      setImporting(false);
      setImportProgress(null);
    }
  }

  function applyPlaybackState(state: PlaybackState) {
    setFile(state.fileName ?? '');
    setFilePath(state.filePath ?? '');
    setPosition(state.position ?? 0);
    setDuration(state.duration ?? 0);
    setStatus(state.status ?? 'idle');
    setVolume(state.volume ?? 100);
    setIsAtmos(state.isAtmosSource ?? false);
    setPlaybackMode(state.playbackMode ?? 'unknown');
    setError(state.error ?? '');
    setEqEnabled(state.eqEnabled ?? false);
  }

  onMount(async () => {
    invoke<PlaybackState>('playback_get_state')
      .then(applyPlaybackState)
      .catch((e: unknown) => setError(String(e)));

    refreshLibrary();
    refreshDevices();

    const unState = await listen<PlaybackState>('playback:state', (e) => {
      applyPlaybackState(e.payload);
    });
    const unPos = await listen<PositionPayload>('playback:position', (e) => {
      setPosition(e.payload.position ?? 0);
      setDuration(e.payload.duration ?? 0);
    });
    const unEnded = await listen('playback:ended', () => setStatus('stopped'));
    const unErr = await listen<string>('playback:error', (e) => setError(e.payload));
    const unMeter = await listen<MeterPayload>('playback:meter', (e) => {
      setMeter(e.payload);
    });

    const unImport = await listen<ImportProgress>('library:import-progress', (e) => {
      setImportProgress(e.payload);
    });

    const unDrop = await listen<{ paths: string[] }>('tauri://drag-drop', (e) => {
      setDragging(false);
      const paths = e.payload.paths;
      if (paths && paths.length > 0) loadFile(paths[0]);
    });
    const unDragEnter = await listen('tauri://drag-enter', () => setDragging(true));
    const unDragLeave = await listen('tauri://drag-leave', () => setDragging(false));

    onCleanup(() => {
      unState(); unPos(); unEnded(); unErr(); unMeter(); unImport();
      unDrop(); unDragEnter(); unDragLeave();
    });
  });

  async function loadFile(path: string, autoPlay: boolean = false) {
    setError('');
    setStatus('loading');
    try {
      await invoke('playback_load', { path });
      refreshLibrary();
      if (autoPlay) {
        await invoke('playback_play');
      }
    } catch (e: unknown) {
      setError(String(e));
      setStatus('error');
    }
  }

  function togglePlay() {
    invoke<void>('playback_toggle_play_pause').catch((e: unknown) => setError(String(e)));
  }

  function onSeek(val: number) {
    setPosition(val);
    invoke<void>('playback_seek', { pos: val }).catch(() => {});
  }

  function onVolume(val: number) {
    setVolume(val);
    invoke<void>('playback_set_volume', { vol: val }).catch(() => {});
  }

  async function openFilePicker() {
    try {
      const selected = await open({
        multiple: false,
        filters: [{ name: 'Audio', extensions: ['mp3','flac','wav','ogg','m4a','aac','opus','aiff','mka','mxf','mp4'] }]
      });
      if (selected && typeof selected === 'string') {
        loadFile(selected);
      }
    } catch (e) {
      setError(String(e));
    }
  }

  return (
    <div
      class="flex h-screen overflow-hidden relative"
      style={{
        "background-color": "var(--bg-primary)",
        background: "radial-gradient(circle at 52% 10%, rgba(244,241,234,0.055), transparent 36%), linear-gradient(180deg, #141519 0%, #101114 100%)",
      }}
    >
      {/* 1. Sidebar — buttons live here now */}
      <Sidebar
        onImportFolder={importFolder}
        onOpenFile={openFilePicker}
        importing={importing()}
      />

      {/* 2. Main Content */}
      <div class="flex-1 flex flex-col min-w-0 pb-[78px]">
        {/* Header */}
        <div
          class="px-7 pt-7 pb-4 flex items-center justify-between flex-shrink-0"
          style={{ "border-bottom": "1px solid rgba(244,241,234,0.1)", background: "rgba(16,17,20,0.72)" }}
        >
          <h1
            class="font-bold tracking-tight"
            style={{ "font-size": "22px", color: "var(--text-primary)", "letter-spacing": "-0.03em" }}
          >
            Recently Added
          </h1>
          <SearchBox onSearch={handleSearch} />
        </div>

        {/* Import progress */}
        {importing() && importProgress() && (
          <div
            class="mx-7 mt-4 rounded-xl px-4 py-3"
            style={{ background: "rgba(34,211,238,0.06)", border: "1px solid rgba(34,211,238,0.12)" }}
          >
            <div class="flex justify-between text-[11px] mb-2" style={{ color: "var(--text-secondary)" }}>
              <span style={{ color: "var(--accent)", opacity: "0.8" }}>Importing…</span>
              <span class="tabular-nums">{importProgress()!.scanned} / {importProgress()!.total || '?'}</span>
            </div>
            <div class="rounded-full overflow-hidden" style={{ height: "2px", background: "rgba(255,255,255,0.06)" }}>
              <div
                class="h-full rounded-full transition-all"
                style={{
                  width: importProgress()!.total > 0
                    ? `${(importProgress()!.scanned / importProgress()!.total) * 100}%`
                    : '100%',
                  background: "linear-gradient(90deg, #0891b2, #22d3ee)",
                  "box-shadow": "0 0 6px rgba(34,211,238,0.5)",
                }}
              />
            </div>
            {importProgress()!.current && (
              <div class="text-[10px] mt-1.5 truncate" style={{ color: "var(--text-muted)" }}>
                {importProgress()!.current}
              </div>
            )}
          </div>
        )}

        {/* Error */}
        {error() && !file() && (
          <div
            class="mx-7 mt-4 px-4 py-3 rounded-xl text-[12px]"
            style={{ background: "rgba(239,68,68,0.08)", border: "1px solid rgba(239,68,68,0.15)", color: "#fca5a5" }}
          >
            {error()}
          </div>
        )}

        {/* Filter row */}
        <div class="px-7 pt-4 pb-2 flex-shrink-0">
          <FilterRow />
        </div>

        {/* Scrollable track grid */}
        <div class="flex-1 overflow-y-auto px-7">
          {tracks().length > 0 ? (
            <AlbumGrid
              tracks={tracks()}
              currentPath={filePath()}
              onPlay={(path) => loadFile(path, true)}
              onDelete={deleteTrack}
            />
          ) : (
            <div class="flex flex-col items-center justify-center py-24">
              <div
                class="w-full max-w-md rounded-xl p-12 text-center transition-all"
                style={{
                  border: `1px solid ${dragging() ? 'rgba(158,216,196,0.45)' : 'rgba(244,241,234,0.11)'}`,
                  background: dragging() ? 'rgba(158,216,196,0.08)' : 'rgba(244,241,234,0.04)',
                  "box-shadow": dragging()
                    ? '0 22px 60px rgba(0,0,0,0.26), 0 0 34px rgba(158,216,196,0.1), inset 0 1px 0 rgba(244,241,234,0.06)'
                    : '0 18px 48px rgba(0,0,0,0.18), inset 0 1px 0 rgba(244,241,234,0.05)',
                  transition: 'all 0.2s ease',
                }}
              >
                <div
                  class="mx-auto mb-6 rounded-full"
                  style={{
                    width: "54px",
                    height: "54px",
                    border: "1px solid rgba(244,241,234,0.16)",
                    background: "radial-gradient(circle, rgba(158,216,196,0.18), rgba(244,241,234,0.04) 56%, transparent 58%)",
                    "box-shadow": "inset 0 0 22px rgba(0,0,0,0.24)",
                  }}
                />
                <div
                  class="font-semibold mb-2"
                  style={{ "font-size": "17px", color: "var(--text-primary)", "letter-spacing": "-0.02em" }}
                >
                  Build your listening room
                </div>
                <div class="mb-6 text-[13px]" style={{ color: "var(--text-secondary)" }}>
                  Drop audio here, or import a folder from the library rail.
                </div>
                <div
                  class="text-[11px] tracking-widest uppercase"
                  style={{ color: "var(--text-muted)", opacity: "0.9" }}
                >
                  MP3 / FLAC / WAV / AAC / OGG / Atmos MP4
                </div>
              </div>
            </div>
          )}
        </div>

        {/* Headphone EQ toast */}
        {showEqPrompt() && (
          <div
            class="absolute right-6 rounded-xl p-4 flex items-center gap-3 z-50"
            style={{
              bottom: "92px",
              background: "rgba(13,13,22,0.95)",
              border: "1px solid rgba(34,211,238,0.2)",
              "box-shadow": "0 8px 32px rgba(0,0,0,0.5), 0 0 20px rgba(34,211,238,0.08)",
              "backdrop-filter": "blur(20px)",
            }}
          >
            <span style={{ "font-size": "22px" }}>🎧</span>
            <div>
              <div class="text-[13px] font-medium" style={{ color: "var(--text-primary)" }}>Headphones detected</div>
              <div class="text-[11px]" style={{ color: "var(--text-muted)" }}>Enable AutoEQ?</div>
            </div>
            <div class="flex gap-2 ml-2">
              <button
                class="text-[11px] px-3 py-1.5 rounded-lg transition-all"
                style={{ background: "rgba(255,255,255,0.06)", color: "var(--text-secondary)", border: "1px solid rgba(255,255,255,0.06)" }}
                onClick={() => setShowEqPrompt(false)}
              >
                Dismiss
              </button>
              <button
                class="text-[11px] px-3 py-1.5 rounded-lg font-medium transition-all"
                style={{ background: "linear-gradient(90deg, #0891b2, #22d3ee)", color: "#fff", "box-shadow": "0 0 10px rgba(34,211,238,0.3)" }}
                onClick={() => { toggleEq(); setShowEqPrompt(false); }}
              >
                Enable EQ
              </button>
            </div>
          </div>
        )}
      </div>

      {/* 4. Player Bar */}
      <PlayerBar
        file={file()}
        isAtmos={isAtmos()}
        status={status()}
        playbackMode={playbackMode()}
        error={error()}
        position={position()}
        duration={duration()}
        volume={volume()}
        coverPath={currentTrack()?.coverPath ?? null}
        meter={meter()}
        audioDevices={audioDevices()}
        currentDeviceUid={currentDeviceUid()}
        onTogglePlay={togglePlay}
        onSeek={onSeek}
        onVolume={onVolume}
        onDeviceChange={setOutputDevice}
        eqEnabled={eqEnabled()}
        onToggleEq={toggleEq}
      />
    </div>
  );
}
