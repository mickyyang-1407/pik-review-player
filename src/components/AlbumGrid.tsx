import { createSignal, For } from 'solid-js';

export interface TrackRow {
  id: number;
  path: string;
  title: string | null;
  artist: string | null;
  album: string | null;
  duration: number;
  format: string;
  coverPath: string | null;
  addedAt: string;
}

interface AlbumGridProps {
  tracks: TrackRow[];
  currentPath: string;
  onPlay: (path: string) => void;
  onDelete: (id: number) => void;
}

function fmtDuration(secs: number): string {
  if (!secs || secs <= 0) return '';
  const m = Math.floor(secs / 60);
  const s = Math.floor(secs % 60);
  return `${m}:${s.toString().padStart(2, '0')}`;
}

function isAtmosTrack(track: TrackRow): boolean {
  const fmt = track.format.toLowerCase();
  const p = track.path.toLowerCase();
  return fmt === 'mp4' || p.includes('adm') || p.includes('atmos');
}

function coverGradient(track: TrackRow): string {
  if (isAtmosTrack(track)) return 'radial-gradient(circle at 28% 22%, rgba(211,241,228,0.34), transparent 28%), linear-gradient(145deg, #27302d 0%, #14181a 58%, #0d0e10 100%)';
  const fmt = track.format.toLowerCase();
  if (fmt === 'flac') return 'radial-gradient(circle at 68% 28%, rgba(195,179,139,0.32), transparent 30%), linear-gradient(145deg, #2c2921 0%, #181816 60%, #0f1010 100%)';
  if (fmt === 'wav') return 'radial-gradient(circle at 30% 68%, rgba(194,157,118,0.3), transparent 32%), linear-gradient(145deg, #30261d 0%, #171614 62%, #0e0e0e 100%)';
  return 'radial-gradient(circle at 32% 24%, rgba(180,188,190,0.22), transparent 28%), linear-gradient(145deg, #262a2f 0%, #17191d 60%, #0f1012 100%)';
}

export default function AlbumGrid(props: AlbumGridProps) {
  const [hoveredId, setHoveredId] = createSignal<number | null>(null);

  return (
    <div
      class="grid gap-6 pb-8"
      style={{ 'grid-template-columns': 'repeat(auto-fill, minmax(164px, 1fr))' }}
    >
      <For each={props.tracks}>
        {(track) => {
          const active = () => props.currentPath === track.path;
          const hovered = () => hoveredId() === track.id;

          return (
            <div
              class="relative rounded-xl cursor-pointer group"
              style={{
                background: hovered() || active() ? 'rgba(244,241,234,0.045)' : 'transparent',
                border: active()
                  ? '1px solid rgba(158,216,196,0.34)'
                  : '1px solid transparent',
                'box-shadow': active()
                  ? '0 18px 40px rgba(0,0,0,0.28)'
                  : hovered()
                    ? '0 14px 32px rgba(0,0,0,0.24)'
                    : 'none',
                transform: hovered() ? 'translateY(-3px)' : 'translateY(0)',
                transition: 'all 0.18s ease',
                padding: '12px',
              }}
              onClick={() => props.onPlay(track.path)}
              onMouseEnter={() => setHoveredId(track.id)}
              onMouseLeave={() => setHoveredId(null)}
            >
              {/* Delete button */}
              {hovered() && (
                <button
                  class="absolute top-2 right-2 rounded-full flex items-center justify-center text-xs leading-none z-10 transition-all"
                  style={{
                    width: '20px',
                    height: '20px',
                    background: 'rgba(15,16,18,0.82)',
                    color: 'var(--text-primary)',
                    'backdrop-filter': 'blur(4px)',
                    border: '1px solid rgba(244,241,234,0.18)',
                    'box-shadow': '0 8px 20px rgba(0,0,0,0.28)',
                  }}
                  onClick={(e) => { e.stopPropagation(); props.onDelete(track.id); }}
                >
                  ×
                </button>
              )}

              {/* Art */}
              <div
                class="relative w-full rounded-lg mb-3 flex items-center justify-center overflow-hidden"
                style={{
                  'aspect-ratio': '1',
                  background: track.coverPath ? '#111' : coverGradient(track),
                  border: '1px solid rgba(244,241,234,0.08)',
                  'box-shadow': active()
                    ? '0 18px 34px rgba(0,0,0,0.34), inset 0 1px 0 rgba(244,241,234,0.08)'
                    : '0 14px 28px rgba(0,0,0,0.28), inset 0 1px 0 rgba(244,241,234,0.06)',
                }}
              >
                {track.coverPath ? (
                  <img
                    src={`asset://localhost/${encodeURI(track.coverPath)}`}
                    alt="Cover"
                    class="w-full h-full object-cover"
                    draggable={false}
                  />
                ) : (
                  <div
                    style={{
                      width: '42%',
                      height: '42%',
                      'border-radius': '999px',
                      border: '1px solid rgba(244,241,234,0.22)',
                      background: 'radial-gradient(circle, rgba(244,241,234,0.16), rgba(244,241,234,0.035) 56%, transparent 58%)',
                      'box-shadow': 'inset 0 0 18px rgba(0,0,0,0.28)',
                    }}
                  />
                )}
              </div>

              {/* Title row */}
              <div
                class="flex items-center gap-1.5 mb-0.5"
                style={{ 'min-width': '0' }}
              >
                <span
                  class="text-[12px] font-semibold truncate leading-tight"
                  style={{
                    color: active() ? 'var(--accent-light)' : 'var(--text-primary)',
                    'letter-spacing': '-0.01em',
                  }}
                >
                  {track.title ?? track.path.split('/').pop()}
                </span>
                {isAtmosTrack(track) && (
                  <span
                    class="flex-shrink-0 text-[7px] px-1 py-px rounded font-bold tracking-widest"
                    style={{
                      background: 'rgba(158,216,196,0.14)',
                      color: 'var(--accent-light)',
                      border: '1px solid rgba(158,216,196,0.22)',
                    }}
                  >
                    ATMOS
                  </span>
                )}
              </div>

              {/* Artist / format */}
              <div class="text-[11px] truncate" style={{ color: 'var(--text-secondary)' }}>
                {track.artist ?? track.format.toUpperCase()}
              </div>

              {/* Duration */}
              {track.duration > 0 && (
                <div
                  class="text-[10px] mt-1 tabular-nums"
                  style={{ color: 'var(--text-muted)', opacity: '0.86' }}
                >
                  {fmtDuration(track.duration)}
                </div>
              )}
            </div>
          );
        }}
      </For>
    </div>
  );
}
