import { createSignal } from 'solid-js';

export default function FilterRow() {
  const [active, setActive] = createSignal('All');
  const filters = ['All', 'ATMOS', 'Electronic', 'Pop', 'Rock', 'Jazz', 'Classical', 'Soundtrack'];

  return (
    <div class="flex items-center gap-2 overflow-x-auto pb-2 scrollbar-hide mb-4">
      {filters.map((f) => (
        <button
          onClick={() => setActive(f)}
          class="px-3.5 py-1.5 rounded-full text-[13px] whitespace-nowrap transition-colors"
          style={{
            "background-color": active() === f ? "rgba(244,241,234,0.14)" : "transparent",
            "border": active() === f ? "1px solid rgba(244,241,234,0.18)" : "1px solid transparent",
            "color": active() === f ? "var(--text-primary)" : "var(--text-muted)",
            "font-weight": active() === f ? "650" : "520",
            "box-shadow": active() === f ? "inset 0 1px 0 rgba(244,241,234,0.05)" : "none"
          }}
          onMouseOver={(e) => {
            if (active() !== f) e.currentTarget.style.backgroundColor = 'var(--bg-hover)';
          }}
          onMouseOut={(e) => {
            if (active() !== f) e.currentTarget.style.backgroundColor = 'transparent';
          }}
        >
          {f === 'ATMOS' ? (
            <span class="flex items-center gap-1">
              <span>Atmos</span>
              {active() === 'ATMOS' ? (
                <span class="text-[10px] bg-black/20 px-1 rounded">ATMOS</span>
              ) : (
                <span class="text-[10px] rounded px-1" style={{ "background-color": "rgba(158,216,196,0.16)", color: "var(--accent-light)" }}>ATMOS</span>
              )}
            </span>
          ) : f}
        </button>
      ))}
    </div>
  );
}
