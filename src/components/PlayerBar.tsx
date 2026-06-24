import { For } from 'solid-js';

function fmt(secs: number): string {
  if (!secs || secs <= 0) return '0:00';
  const m = Math.floor(secs / 60);
  const s = Math.floor(secs % 60);
  return `${m}:${s.toString().padStart(2, '0')}`;
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
  mode: string;
  channels: MeterChannel[];
}

interface PlayerBarProps {
  file: string;
  isAtmos: boolean;
  status: 'idle' | 'loading' | 'playing' | 'paused' | 'stopped' | 'error';
  playbackMode: string;
  error: string;
  position: number;
  duration: number;
  volume: number;
  coverPath: string | null;
  meter: MeterPayload;
  audioDevices: AudioDevice[];
  currentDeviceUid: string;
  eqEnabled: boolean;
  onTogglePlay: () => void;
  onSeek: (val: number) => void;
  onVolume: (val: number) => void;
  onDeviceChange: (uid: string) => void;
  onToggleEq: () => void;
}

function IconPrev() {
  return (
    <svg width="15" height="15" viewBox="0 0 24 24" fill="currentColor">
      <path d="M6 6h2v12H6zm3.5 6 8.5 6V6z"/>
    </svg>
  );
}
function IconNext() {
  return (
    <svg width="15" height="15" viewBox="0 0 24 24" fill="currentColor">
      <path d="M6 18l8.5-6L6 6v12zm2-8.14 4.03 2.14L8 14.14V9.86zM16 6h2v12h-2z"/>
    </svg>
  );
}
function IconPlay() {
  return (
    <svg width="14" height="14" viewBox="0 0 24 24" fill="white">
      <path d="M8 5v14l11-7z"/>
    </svg>
  );
}
function IconPause() {
  return (
    <svg width="14" height="14" viewBox="0 0 24 24" fill="white">
      <rect x="6" y="4" width="4" height="16"/>
      <rect x="14" y="4" width="4" height="16"/>
    </svg>
  );
}
function IconVolume() {
  return (
    <svg width="13" height="13" viewBox="0 0 24 24" fill="currentColor">
      <path d="M3 9v6h4l5 5V4L7 9H3zm13.5 3c0-1.77-1.02-3.29-2.5-4.03v8.05c1.48-.73 2.5-2.25 2.5-4.02z"/>
    </svg>
  );
}

function InlineMeter(props: { isAtmos: boolean; active: boolean; meter: MeterPayload }) {
  const fallbackChannels = () => props.isAtmos
    ? ['L', 'R', 'C', 'LFE', 'Ls', 'Rs', 'Lrs', 'Rrs', 'Ltf', 'Rtf', 'Ltr', 'Rtr'].map((label) => ({ label, rms: 0, peak: 0 }))
    : ['L', 'R'].map((label) => ({ label, rms: 0, peak: 0 }));
  const channels = () => props.meter.available && props.meter.channels.length > 0
    ? props.meter.channels
    : fallbackChannels();
  const available = () => props.meter.available && props.active;

  return (
    <div
      class="flex items-end gap-[2px] h-[34px] px-2.5 py-1.5 rounded-md"
      style={{
        background: 'rgba(244,241,234,0.05)',
        border: '1px solid rgba(244,241,234,0.08)',
      }}
      title={available() ? "Metering Active" : "Metering Unavailable"}
    >
      <For each={channels()}>
        {(channel) => (
          <div class="relative w-[5px] h-full rounded-sm overflow-hidden" style={{ background: 'rgba(244,241,234,0.08)' }}>
            <div
              class="absolute bottom-0 left-0 right-0 transition-[height] duration-75"
              style={{
                height: `${Math.max(0, Math.min(1, channel.rms)) * 100}%`,
                background: 'linear-gradient(0deg, #5fa98f 0%, #9ed8c4 72%, #d8c477 100%)',
                opacity: available() ? 1 : 0.2,
              }}
            />
            {available() && channel.peak > 0 && (
              <div
                class="absolute left-0 right-0 h-px"
                style={{
                  bottom: `${Math.max(0, Math.min(1, channel.peak)) * 100}%`,
                  background: 'rgba(244,241,234,0.9)',
                }}
              />
            )}
          </div>
        )}
      </For>
    </div>
  );
}

function fileUrl(path: string): string {
  return `asset://localhost/${encodeURI(path)}`;
}

function MiniArtwork(props: { isAtmos: boolean; hasFile: boolean; coverPath: string | null }) {
  return (
    <div
      class="flex-shrink-0 rounded-lg overflow-hidden"
      style={{
        width: '48px',
        height: '48px',
        background: props.hasFile
          ? props.isAtmos
            ? 'radial-gradient(circle at 28% 22%, rgba(211,241,228,0.28), transparent 32%), linear-gradient(145deg, #27302d, #121416)'
            : 'radial-gradient(circle at 32% 24%, rgba(244,241,234,0.18), transparent 30%), linear-gradient(145deg, #2a2d31, #15171a)'
          : 'linear-gradient(145deg, #22252a, #15171a)',
        border: '1px solid rgba(244,241,234,0.1)',
        'box-shadow': '0 12px 24px rgba(0,0,0,0.25), inset 0 1px 0 rgba(244,241,234,0.06)',
      }}
    >
      {props.coverPath ? (
        <img
          src={fileUrl(props.coverPath)}
          alt="Album artwork"
          class="w-full h-full object-cover"
          draggable={false}
        />
      ) : (
        <div
          class="w-full h-full flex items-center justify-center"
          style={{ opacity: props.hasFile ? 1 : 0.42 }}
        >
          <div
            class="rounded-full"
            style={{
              width: '38%',
              height: '38%',
              border: '1px solid rgba(244,241,234,0.2)',
              background: 'radial-gradient(circle, rgba(244,241,234,0.14), transparent 62%)',
            }}
          />
        </div>
      )}
    </div>
  );
}

export default function PlayerBar(props: PlayerBarProps) {
  const progress = () => props.duration > 0 ? (props.position / props.duration) * 100 : 0;
  const isPlaying = () => props.status === 'playing';

  const modeTag = () => {
    if (props.error) return null;
    if (props.playbackMode === 'spatial') return 'Spatial Audio';
    if (props.playbackMode === 'stereo') return 'Stereo';
    return null;
  };

  return (
    <div
      class="fixed bottom-0 left-0 right-0 z-50 select-none"
      style={{
        height: '78px',
        background: 'linear-gradient(180deg, rgba(25,26,29,0.9) 0%, rgba(18,19,22,0.98) 100%)',
        'backdrop-filter': 'blur(28px)',
        '-webkit-backdrop-filter': 'blur(28px)',
        'box-shadow': '0 -1px 0 rgba(244,241,234,0.12), 0 -18px 48px rgba(0,0,0,0.34), inset 0 1px 0 rgba(244,241,234,0.06)',
      }}
    >


      {/* Top progress bar — flush */}
      <div
        class="absolute top-0 left-0 right-0"
        style={{ height: '3px', background: 'rgba(244,241,234,0.12)' }}
      >
        <div
          class="h-full transition-[width] duration-300 ease-out"
          style={{
            width: `${progress()}%`,
            background: 'linear-gradient(90deg, #5fa98f, #9ed8c4, #d3f1e4)',
            'box-shadow': '0 0 10px rgba(158,216,196,0.28)',
          }}
        />
        <input
          type="range"
          min="0"
          max={props.duration || 1}
          step="0.5"
          value={props.position}
          onInput={(e) => props.onSeek(parseFloat(e.currentTarget.value))}
          class="absolute inset-0 w-full h-full opacity-0 cursor-pointer"
          style={{ margin: 0 }}
        />
      </div>

      <div class="flex items-center h-full px-5 gap-4">

        {/* ── LEFT: Art + Track Info ── */}
        <div class="flex items-center gap-3 w-[28%] min-w-[180px]">
          <MiniArtwork isAtmos={props.isAtmos} hasFile={Boolean(props.file)} coverPath={props.coverPath} />

          {/* Track info */}
          <div class="min-w-0 flex-1">
            {props.file ? (
              <>
                <div class="flex items-center gap-1.5 mb-0.5">
                  <span
                    class="marquee-wrap text-[13px] font-semibold leading-tight overflow-hidden"
                    data-marquee={props.file && props.file.length > 28 ? 'true' : 'false'}
                    style={{ color: 'var(--text-primary)', 'letter-spacing': '-0.01em', '--marquee-width': '176px', 'max-width': '176px' }}
                  >
                    <span class="marquee-track">{props.file}</span>
                  </span>
                  {props.isAtmos && (
                    <span
                      class="flex-shrink-0 text-[8px] px-1.5 py-px rounded font-bold tracking-widest"
                      style={{
                        background: 'rgba(158,216,196,0.14)',
                        color: 'var(--accent-light)',
                        border: '1px solid rgba(158,216,196,0.22)',
                        'letter-spacing': '0.08em',
                      }}
                    >
                      ATMOS
                    </span>
                  )}
                </div>
                <div class="text-[11px] truncate" style={{ color: 'var(--text-secondary)' }}>
                  {props.error
                    ? <span style={{ color: '#f87171' }}>{props.error}</span>
                    : props.status === 'loading'
                      ? <span>Loading…</span>
                      : modeTag()
                        ? <span style={{ color: 'var(--accent-light)', opacity: '0.95' }}>{modeTag()}</span>
                        : <span>—</span>
                  }
                </div>
              </>
            ) : (
              <div class="text-[12px]" style={{ color: 'var(--text-secondary)' }}>No track loaded</div>
            )}
          </div>
        </div>

        {/* ── CENTER: Controls + Time ── */}
        <div class="flex-1 flex flex-col items-center justify-center gap-1.5">
          {/* Buttons */}
          <div class="flex items-center gap-6">
            <button
              class="flex items-center justify-center transition-all duration-150"
              style={{ color: 'var(--text-secondary)' }}
              onMouseOver={(e) => (e.currentTarget as HTMLElement).style.color = 'var(--text-primary)'}
              onMouseOut={(e) => (e.currentTarget as HTMLElement).style.color = 'var(--text-secondary)'}
            >
              <IconPrev />
            </button>

            <button
              onClick={props.onTogglePlay}
              disabled={!props.file}
              class="flex items-center justify-center rounded-full transition-all duration-150 disabled:opacity-30"
              style={{
                width: '38px',
                height: '38px',
                background: props.file ? 'linear-gradient(145deg, #d3f1e4, #9ed8c4)' : 'rgba(244,241,234,0.12)',
                'box-shadow': props.file
                  ? '0 12px 28px rgba(0,0,0,0.22), inset 0 1px 0 rgba(255,255,255,0.35)'
                  : 'none',
              }}
              onMouseOver={(e) => {
                if (props.file) (e.currentTarget as HTMLElement).style.boxShadow = '0 14px 34px rgba(0,0,0,0.28), inset 0 1px 0 rgba(255,255,255,0.35)';
              }}
              onMouseOut={(e) => {
                if (props.file) (e.currentTarget as HTMLElement).style.boxShadow = '0 12px 28px rgba(0,0,0,0.22), inset 0 1px 0 rgba(255,255,255,0.35)';
              }}
            >
              {props.status === 'loading'
                ? <div class="animate-spin" style={{ width: '12px', height: '12px', border: '2px solid rgba(255,255,255,0.3)', 'border-top-color': 'white', 'border-radius': '50%' }} />
                : isPlaying() ? <IconPause /> : <IconPlay />
              }
            </button>

            <button
              class="flex items-center justify-center transition-all duration-150"
              style={{ color: 'var(--text-secondary)' }}
              onMouseOver={(e) => (e.currentTarget as HTMLElement).style.color = 'var(--text-primary)'}
              onMouseOut={(e) => (e.currentTarget as HTMLElement).style.color = 'var(--text-secondary)'}
            >
              <IconNext />
            </button>
          </div>

          {/* Timestamps */}
          <div
            class="flex items-center gap-2 tabular-nums"
            style={{ 'font-size': '10px', color: 'var(--text-secondary)', 'letter-spacing': '0.02em' }}
          >
            <span>{fmt(props.position)}</span>
            <span style={{ opacity: '0.3' }}>·</span>
            <span>{fmt(props.duration)}</span>
          </div>
        </div>

        {/* ── RIGHT: EQ + Device + Volume ── */}
        <div class="flex items-center justify-end gap-3 w-[28%] min-w-[240px]">
          {/* Metering */}
          {props.file && <InlineMeter isAtmos={props.isAtmos} active={props.status === 'playing'} meter={props.meter} />}

          {/* EQ pill */}
          <button
            onClick={props.onToggleEq}
            class="whitespace-nowrap text-[10px] font-semibold px-2.5 py-1 rounded-md transition-all duration-150 tracking-wider"
            style={{
              background: props.eqEnabled ? 'rgba(158,216,196,0.16)' : 'rgba(244,241,234,0.05)',
              color: props.eqEnabled ? 'var(--accent-light)' : 'var(--text-secondary)',
              border: `1px solid ${props.eqEnabled ? 'rgba(158,216,196,0.3)' : 'rgba(244,241,234,0.12)'}`,
              'box-shadow': props.eqEnabled ? '0 0 8px rgba(158,216,196,0.12)' : 'none',
            }}
          >
            {props.eqEnabled ? 'AutoEQ On' : 'AutoEQ Off'}
          </button>

          {/* Device selector */}
          {props.audioDevices.length > 0 && (
            <select
              value={props.currentDeviceUid}
              onChange={(e) => props.onDeviceChange(e.currentTarget.value)}
              class="rounded-md px-2 py-1 max-w-[108px] truncate cursor-pointer outline-none"
              style={{
                'font-size': '10px',
                background: 'rgba(244,241,234,0.07)',
                color: 'var(--text-secondary)',
                border: '1px solid rgba(244,241,234,0.13)',
              }}
            >
              <option value="">System Default</option>
              <For each={props.audioDevices}>
                {(dev) => <option value={dev.uid}>{dev.name}{dev.isDefault ? ' ✓' : ''}</option>}
              </For>
            </select>
          )}

          {/* Volume */}
          <div class="flex items-center gap-2">
            <span style={{ color: 'var(--text-secondary)' }}><IconVolume /></span>
            <div class="flex flex-col gap-1">
              <div
                class="relative rounded-full"
                style={{ width: '92px', height: '4px', background: 'rgba(244,241,234,0.14)' }}
              >
                <div
                  class="absolute inset-y-0 left-0 rounded-full"
                  style={{
                    width: `${Math.min(100, (props.volume / 130) * 100)}%`,
                    background: 'linear-gradient(90deg, #5fa98f, #9ed8c4)',
                  }}
                />
                <div class="absolute top-[-3px] w-px h-[10px]" style={{ left: '0%', background: 'rgba(244,241,234,0.18)' }} />
                <div class="absolute top-[-3px] w-px h-[10px]" style={{ left: '76.9%', background: 'rgba(244,241,234,0.28)' }} />
                <div class="absolute top-[-3px] w-px h-[10px]" style={{ right: '0%', background: 'rgba(244,241,234,0.18)' }} />
                <input
                  type="range"
                  min="0"
                  max="130"
                  step="1"
                  value={props.volume}
                  onInput={(e) => props.onVolume(parseInt(e.currentTarget.value, 10))}
                  class="absolute inset-0 w-full h-full opacity-0 cursor-pointer"
                  style={{ margin: 0 }}
                />
              </div>
              <div class="flex justify-between text-[8px] tabular-nums" style={{ color: 'var(--text-muted)' }}>
                <span>0</span><span>100</span><span>130</span>
              </div>
            </div>
            <input
              type="number"
              min="0"
              max="130"
              value={Math.round(props.volume)}
              onInput={(e) => props.onVolume(Number(e.currentTarget.value || 0))}
              class="w-[42px] rounded-md px-1.5 py-1 text-[10px] tabular-nums outline-none"
              style={{
                background: 'rgba(244,241,234,0.07)',
                color: 'var(--text-primary)',
                border: '1px solid rgba(244,241,234,0.13)',
              }}
            />
            </div>
        </div>

      </div>
    </div>
  );
}
