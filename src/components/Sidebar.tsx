interface SidebarProps {
  onImportFolder: () => void;
  onOpenFile: () => void;
  importing: boolean;
}

function NavItem(props: { label: string; active?: boolean; onClick?: () => void }) {
  return (
    <li>
      <button
        class="w-full text-left px-3 py-2 rounded-md text-[13px] transition-all duration-150"
        style={{
          background: props.active ? 'rgba(244,241,234,0.08)' : 'transparent',
          color: props.active ? 'var(--text-primary)' : 'var(--text-secondary)',
          'font-weight': props.active ? '600' : '500',
          border: props.active ? '1px solid rgba(244,241,234,0.12)' : '1px solid transparent',
          cursor: 'pointer',
        }}
        onMouseOver={(e) => {
          if (!props.active) (e.currentTarget as HTMLElement).style.background = 'rgba(244,241,234,0.055)';
        }}
        onMouseOut={(e) => {
          if (!props.active) (e.currentTarget as HTMLElement).style.background = 'transparent';
        }}
        onClick={props.onClick}
      >
        {props.label}
      </button>
    </li>
  );
}

export default function Sidebar(props: SidebarProps) {
  return (
    <div
      class="flex flex-col h-full flex-shrink-0"
      style={{
        width: '228px',
        background: 'linear-gradient(180deg, #17181b 0%, #111214 100%)',
        'border-right': '1px solid rgba(244,241,234,0.1)',
        'box-shadow': 'inset -1px 0 0 rgba(0,0,0,0.22)',
      }}
    >
      {/* Logo */}
      <div class="px-5 pt-6 pb-5 flex items-center gap-3">
        <div
          class="flex items-center justify-center rounded-full font-bold text-[13px]"
          style={{
            width: '32px',
            height: '32px',
            background: 'linear-gradient(145deg, #2a2d31, #111214)',
            color: 'var(--accent-light)',
            border: '1px solid rgba(244,241,234,0.12)',
            'box-shadow': 'inset 0 1px 0 rgba(244,241,234,0.08), 0 10px 22px rgba(0,0,0,0.22)',
          }}
        >
          P
        </div>
        <div
          class="font-semibold text-[15px] tracking-tight"
          style={{ color: 'var(--text-primary)', 'letter-spacing': '-0.02em' }}
        >
          Pik Review
        </div>
      </div>

      {/* Add to library */}
      <div class="px-4 pb-4 flex flex-col gap-2">
        <button
          onClick={props.onImportFolder}
          disabled={props.importing}
          class="w-full flex items-center gap-2 px-3 py-2 rounded-lg text-[12px] font-medium transition-all duration-150"
          style={{
            background: 'rgba(244,241,234,0.09)',
            color: props.importing ? 'var(--text-muted)' : 'var(--text-primary)',
            border: '1px solid rgba(244,241,234,0.14)',
            cursor: props.importing ? 'not-allowed' : 'pointer',
            opacity: props.importing ? '0.6' : '1',
          }}
          onMouseOver={(e) => {
            if (!props.importing) (e.currentTarget as HTMLElement).style.background = 'rgba(244,241,234,0.13)';
          }}
          onMouseOut={(e) => {
            (e.currentTarget as HTMLElement).style.background = 'rgba(244,241,234,0.09)';
          }}
        >
          <span style={{ 'font-size': '14px' }}>{props.importing ? '⟳' : '+'}</span>
          {props.importing ? 'Importing…' : 'Import Folder'}
        </button>
        <button
          onClick={props.onOpenFile}
          class="w-full flex items-center gap-2 px-3 py-2 rounded-lg text-[12px] transition-all duration-150"
          style={{
            background: 'transparent',
            color: 'var(--text-secondary)',
            border: '1px solid rgba(244,241,234,0.11)',
            cursor: 'pointer',
          }}
          onMouseOver={(e) => {
            (e.currentTarget as HTMLElement).style.background = 'rgba(244,241,234,0.06)';
          }}
          onMouseOut={(e) => {
            (e.currentTarget as HTMLElement).style.background = 'transparent';
          }}
        >
          <span style={{ 'font-size': '14px' }}>↗</span>
          Open File…
        </button>
      </div>

      {/* Divider */}
      <div style={{ height: '1px', background: 'rgba(244,241,234,0.1)', margin: '0 16px 12px' }} />

      {/* Library nav */}
      <div class="px-4 flex-1 overflow-y-auto">
        <div
          class="text-[10px] font-semibold mb-2 px-3 uppercase tracking-widest"
          style={{ color: 'var(--text-muted)' }}
        >
          Library
        </div>
        <ul class="space-y-0.5">
          <NavItem label="Tracks" active />
          <NavItem label="Albums" />
          <NavItem label="Artists" />
          <NavItem label="Genres" />
        </ul>

        <div
          class="text-[10px] font-semibold mb-2 px-3 mt-5 uppercase tracking-widest"
          style={{ color: 'var(--text-muted)' }}
        >
          Playlists
        </div>
        <ul class="space-y-0.5">
          <NavItem label="Favorites" />
          <NavItem label="Recently Added" />
          <NavItem label="Atmos Mixes" />
        </ul>
      </div>

      {/* Settings */}
      <div
        class="p-4 mt-auto"
        style={{ 'border-top': '1px solid rgba(244,241,234,0.1)' }}
      >
        <NavItem label="Settings" />
      </div>
    </div>
  );
}
