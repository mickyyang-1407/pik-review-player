import { onCleanup } from 'solid-js';

interface SearchBoxProps {
  onSearch: (query: string) => void;
}

export default function SearchBox(props: SearchBoxProps) {
  let debounce: ReturnType<typeof setTimeout>;

  // Clear pending timeout when component unmounts to prevent stale invoke after disposal
  onCleanup(() => clearTimeout(debounce));

  function handleInput(e: InputEvent) {
    clearTimeout(debounce);
    const q = (e.currentTarget as HTMLInputElement).value;
    debounce = setTimeout(() => props.onSearch(q), 250);
  }

  return (
    <div class="relative">
      <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
        <svg class="h-4 w-4" style={{ color: "var(--text-muted)" }} fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
        </svg>
      </div>
      <input
        type="text"
        placeholder="Search library"
        class="block w-64 pl-10 pr-3 py-2 rounded-full text-sm transition-colors focus:outline-none"
        style={{
          "background-color": "rgba(244,241,234,0.07)",
          "border": "1px solid rgba(244,241,234,0.1)",
          "color": "var(--text-primary)",
          "box-shadow": "inset 0 1px 0 rgba(244,241,234,0.05)"
        }}
        onInput={handleInput}
        onFocus={(e) => {
          e.currentTarget.style.borderColor = 'rgba(158,216,196,0.42)';
          e.currentTarget.style.backgroundColor = 'rgba(244,241,234,0.1)';
        }}
        onBlur={(e) => {
          e.currentTarget.style.borderColor = 'rgba(244,241,234,0.1)';
          e.currentTarget.style.backgroundColor = 'rgba(244,241,234,0.07)';
        }}
      />
    </div>
  );
}
