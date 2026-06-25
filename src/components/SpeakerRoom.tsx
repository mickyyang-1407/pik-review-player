import { Component, createSignal, For, Show } from 'solid-js';

interface SpeakerRoomProps {
  channels: { label: string; rms: number; peak: number }[];
  available: boolean;
  active: boolean; // is playing
}

interface SpeakerSoloMute {
  [label: string]: { solo: boolean; mute: boolean };
}

type GroupName = 'Front' | 'Side' | 'Rear' | 'Top' | 'LFE' | 'All';

const groups: Record<GroupName, string[]> = {
  Front: ['L', 'C', 'R'],
  Side: ['Ls', 'Rs'],
  Rear: ['Lrs', 'Rrs'],
  Top: ['Ltf', 'Rtf', 'Ltr', 'Rtr'],
  LFE: ['LFE'],
  All: ['L', 'C', 'R', 'Ls', 'Rs', 'Lrs', 'Rrs', 'Ltf', 'Rtf', 'Ltr', 'Rtr', 'LFE']
};

const SpeakerRoom: Component<SpeakerRoomProps> = (props) => {
  const [soloMute, setSoloMute] = createSignal<SpeakerSoloMute>({});
  const [selectedGroup, setSelectedGroup] = createSignal<GroupName | null>(null);
  
  // Handlers for S/M
  const toggleSolo = (label: string) => {
    setSoloMute(prev => ({
      ...prev,
      [label]: { solo: !prev[label]?.solo, mute: false }
    }));
  };

  const toggleMute = (label: string) => {
    setSoloMute(prev => ({
      ...prev,
      [label]: { solo: false, mute: !prev[label]?.mute }
    }));
  };
  
  const isAnySolo = () => Object.values(soloMute()).some(s => s.solo);
  
  const isSpeakerActive = (label: string) => {
    const sm = soloMute()[label] || { solo: false, mute: false };
    if (sm.mute) return false;
    if (isAnySolo() && !sm.solo) return false;
    return true;
  };
  
  const handleGroupClick = (g: GroupName) => {
    setSelectedGroup(g === selectedGroup() ? null : g);
  };
  
  const handleGroupDblClick = (g: GroupName) => {
    if (g === 'All') {
      setSoloMute({});
    } else {
      const labels = groups[g];
      const newSoloMute: SpeakerSoloMute = {};
      labels.forEach(l => {
        newSoloMute[l] = { solo: true, mute: false };
      });
      setSoloMute(newSoloMute);
    }
  };

  const getChannel = (label: string) => props.channels.find(c => c.label === label);

  const SpeakerDot: Component<{ label: string; x: string; y: string; isTop?: boolean; isLfe?: boolean }> = (p) => {
    const channel = () => getChannel(p.label);
    const rms = () => channel() ? channel()!.rms : -60;
    const peak = () => channel() ? channel()!.peak : 0;
    
    const active = () => props.available && props.active && isSpeakerActive(p.label);
    const mutedBySoloOrMute = () => !isSpeakerActive(p.label);
    
    // Convert dB (-60 to 0) to scale (0 to 1) for glow/size
    const normalizedRms = () => Math.max(0, (rms() + 60) / 60);
    const baseSize = 16;
    const activeSize = () => baseSize + normalizedRms() * 12;
    
    const isPeakClipping = () => peak() > 0.9 && active();
    
    // Group highlighting
    const isHighlighted = () => {
      const group = selectedGroup();
      return group && groups[group].includes(p.label);
    };

    const getGlow = () => {
      if (!active()) return 'none';
      if (rms() > -10) return `0 0 24px rgba(158,216,196,1.0)`;
      if (rms() > -30) return `0 0 16px rgba(158,216,196,0.5)`;
      if (rms() > -60) return `0 0 8px rgba(158,216,196,0.2)`;
      return 'none';
    };

    const getOpacity = () => {
      if (!props.available) return 0.2;
      if (mutedBySoloOrMute()) return 0.1;
      if (!props.active) return 0.5;
      if (!active()) return 0.2;
      
      if (rms() > -10) return 1.0;
      if (rms() > -30) return 0.5;
      return 0.2;
    };
    
    // Tooltip logic
    const [hovered, setHovered] = createSignal(false);
    
    const sm = () => soloMute()[p.label] || { solo: false, mute: false };

    return (
      <div 
        class="absolute"
        style={{
          top: p.y,
          left: p.x,
          transform: "translate(-50%, -50%)",
          "z-index": hovered() ? 50 : 10
        }}
        onMouseEnter={() => setHovered(true)}
        onMouseLeave={() => setHovered(false)}
      >
        <div class="relative flex items-center justify-center">
          {/* Label text next to speaker */}
          <div 
            class="absolute text-[10px] font-mono tracking-wider transition-opacity pointer-events-none"
            style={{
              color: "var(--text-muted)",
              top: "-20px",
              opacity: mutedBySoloOrMute() ? 0.3 : 0.8
            }}
          >
            {p.label}
          </div>

          <div
            class={`transition-all duration-100 ease-out ${isPeakClipping() ? 'animate-blink-red' : ''} ${rms() > -10 && active() ? 'animate-pulse-glow' : ''}`}
            style={{
              width: `${activeSize()}px`,
              height: `${activeSize()}px`,
              "background-color": p.isTop ? "transparent" : "var(--accent-light)",
              border: p.isTop ? `1.5px dashed var(--accent)` : "none",
              "box-shadow": p.isTop ? "none" : getGlow(),
              "border-color": p.isTop && getGlow() !== 'none' ? "var(--accent-light)" : "var(--accent)",
              "border-radius": p.isLfe ? "2px" : "50%",
              opacity: getOpacity(),
              // Additional highlight from group selection
              ...(isHighlighted() && !p.isTop ? { "box-shadow": `0 0 0 2px var(--bg-card), 0 0 0 4px var(--accent)` } : {}),
              ...(isHighlighted() && p.isTop ? { "border-color": "var(--accent-light)", filter: "drop-shadow(0 0 4px var(--accent))" } : {})
            }}
          />
          
          {/* S/M Tooltip */}
          <Show when={hovered()}>
            <div 
              class="absolute bottom-full mb-2 flex items-center gap-1 p-1 rounded bg-[rgba(20,20,25,0.95)] border border-[var(--border-soft)] backdrop-blur shadow-lg pointer-events-auto"
              style={{ "min-width": "max-content", "transform": "translateX(-50%)", "left": "50%" }}
            >
              <button
                class="w-5 h-5 rounded text-[10px] font-bold flex items-center justify-center transition-colors hover:opacity-80"
                style={{
                  background: sm().solo ? "var(--accent)" : "transparent",
                  color: sm().solo ? "var(--bg-primary)" : "var(--text-secondary)"
                }}
                onClick={() => toggleSolo(p.label)}
                title="Solo"
              >
                S
              </button>
              <button
                class="w-5 h-5 rounded text-[10px] font-bold flex items-center justify-center transition-colors hover:opacity-80"
                style={{
                  background: sm().mute ? "#ef4444" : "transparent",
                  color: sm().mute ? "#fff" : "var(--text-secondary)"
                }}
                onClick={() => toggleMute(p.label)}
                title="Mute"
              >
                M
              </button>
            </div>
          </Show>
        </div>
      </div>
    );
  };

  const groupLabels: GroupName[] = ['Front', 'Side', 'Rear', 'Top', 'LFE', 'All'];

  return (
    <div class="relative flex flex-col items-center justify-center h-full w-full min-h-[300px] gap-6">
      <div class="relative w-[320px] h-[240px] border border-[var(--border-soft)] rounded-xl bg-[var(--bg-card)]">
        {/* Head Position */}
        <div class="absolute top-[60%] left-1/2 w-8 h-8 rounded-full border border-[var(--text-muted)] -translate-x-1/2 -translate-y-1/2 flex items-center justify-center opacity-50">
          <div class="w-1 h-1 bg-[var(--text-muted)] rounded-full"></div>
        </div>

        {/* L, C, R */}
        <SpeakerDot label="L" x="20%" y="10%" />
        <SpeakerDot label="C" x="50%" y="5%" />
        <SpeakerDot label="R" x="80%" y="10%" />
        
        {/* LFE */}
        <SpeakerDot label="LFE" x="35%" y="5%" isLfe={true} />

        {/* Side Ls, Rs */}
        <SpeakerDot label="Ls" x="10%" y="50%" />
        <SpeakerDot label="Rs" x="90%" y="50%" />

        {/* Rear Lrs, Rrs */}
        <SpeakerDot label="Lrs" x="25%" y="90%" />
        <SpeakerDot label="Rrs" x="75%" y="90%" />

        {/* Top Front Ltf, Rtf */}
        <SpeakerDot label="Ltf" x="30%" y="30%" isTop={true} />
        <SpeakerDot label="Rtf" x="70%" y="30%" isTop={true} />

        {/* Top Rear Ltr, Rtr */}
        <SpeakerDot label="Ltr" x="30%" y="70%" isTop={true} />
        <SpeakerDot label="Rtr" x="70%" y="70%" isTop={true} />
      </div>

      {/* Group Controls */}
      <div class="flex gap-2 p-1 rounded-lg bg-[rgba(255,255,255,0.02)] border border-[var(--border-soft)]">
        <For each={groupLabels}>
          {(g) => (
            <button
              class="px-3 py-1 text-[11px] font-mono tracking-wider uppercase rounded transition-all hover:bg-[rgba(255,255,255,0.05)]"
              style={{
                background: selectedGroup() === g ? "rgba(158,216,196,0.15)" : "transparent",
                color: selectedGroup() === g ? "var(--accent)" : "var(--text-muted)",
                "box-shadow": selectedGroup() === g ? "inset 0 0 0 1px var(--accent-dark)" : "none"
              }}
              onClick={() => handleGroupClick(g)}
              onDblClick={() => handleGroupDblClick(g)}
              title="Click to highlight, Double-click to solo"
            >
              {g}
            </button>
          )}
        </For>
      </div>
    </div>
  );
};

export default SpeakerRoom;
