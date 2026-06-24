# Prompt：以 Pik Player 為基礎開新專案「Pik Player Pro / Review Player」

你是一位資深 macOS/Tauri/Solid/Rust 產品工程師與高端音訊產品設計師。請以現有 `pik-player` 專案為基礎，分岔出一個新的專業版播放器專案，暫名 **Pik Player Pro** 或 **Pik Review Player**。

## 背景

現有 Pik Player 是一個 macOS Atmos / Spatial Audio music player，技術棧與能力包含：

- Tauri v2 + SolidJS + TypeScript frontend
- Rust backend
- AVFoundation / AVPlayer audio-only AtmosPlayer
- MTAudioProcessingTap + vDSP metering / EQ
- mpv fallback / compatibility path
- CoreAudio output device list / switching
- AlbumGrid、PlayerBar、library DB、folder import、cover extraction
- Tauri asset protocol for embedded cover art

請不要重寫整個 player。要 fork / copy 現有播放核心，然後重新設計成專業 review workflow。

## 產品定位

Pik Player Pro 不是一般消費端音樂播放器，而是給製作人、混音師、藝人、經紀人、A&R、導演、客戶使用的 **music review / mix revision / approval player**。

核心用途：

1. 把曲子或 mix 版本拖進來播放。
2. 聽的過程中用熱鍵或滑鼠點擊時間軸建立 timecoded notes。
3. 右側 Notes panel 可以寫總 note 與逐點修改 note。
4. 上方有專業 immersive speaker room visualization，播放時喇叭依照音量 / meter 即時發光。
5. 可以點擊喇叭或 group 做 Solo / Mute。
6. 新版本 V2 / V3 拖進來後，V1 notes 仍顯示在 V2 播放時，方便逐點確認是否修好。
7. 每個 note 可以標記 Done / OK。
8. 最後可以輸出 CSV / Plain Text / Markdown，未來可擴充 PDF / DAW marker export。

## UI 方向

整體風格：深色、專業、像 Dolby Renderer / mastering room / post-production review tool，不要像一般音樂 app，也不要遊戲化。

### 主要版面

- Top / Center：Immersive speaker room visualization
  - 類似 Dolby Atmos 7.1.4 房間線框圖。
  - 預設顯示 Front / Wide / Surround / Rear / Top / LFE tabs 或 group pills。
  - 每顆 speaker 有即時 meter glow：音量越大，線框越亮、光暈越強。
  - 支援點擊 speaker 或 group：Solo / Mute。
  - 第一版可以用現有 meter/channel data 驅動，不需要真的解析 Dolby object metadata。

- Bottom：Professional transport / timeline
  - Play / Pause / Stop / seek。
  - 顯示 current timecode / duration。
  - 支援滑鼠點時間軸 seek。
  - 支援點擊時間軸建立 note at timecode。
  - 支援 keyboard shortcuts。

- Right：Notes panel
  - General Notes 區塊：整首歌總評。
  - Timecoded Notes 區塊：每則 note 綁定時間碼。
  - 每則 note 顯示：timecode、body、status、author role、created version、resolved version。
  - 點 note 可跳回時間碼。
  - 播放到 note 附近自動 highlight。
  - 每則 note 有 `Done` / `OK` 按鈕。

## 核心功能 MVP

### 1. Project / Version model

建立本機 project model：

```text
Project
- id
- title
- client / artist / album / cue optional
- createdAt
- updatedAt

Version
- id
- projectId
- label: V1 / V2 / Mix 03 / Master 01
- filePath
- duration
- checksum or fingerprint optional
- importedAt

Note
- id
- projectId
- createdInVersionId
- resolvedInVersionId nullable
- timecodeMs nullable; null means general note
- body
- authorRole: Producer / Artist / Manager / Mixer / Client / Other
- status: open / done / ok / needs_check / rejected
- createdAt
- updatedAt
```

Use SQLite locally first. Do not build cloud sync in MVP.

### 2. Import workflow

- Drag one audio file into empty app → create new Project + Version V1.
- Drag another file into same project → create Version V2.
- User can rename project and version labels.
- When playing V2, show notes created in V1, with clear version badges.

### 3. Note workflow

- Hotkey `N`: create a timecoded note at current playback position and focus text input.
- Hotkey `Space`: play / pause.
- Hotkey `D`: mark selected/current note Done.
- Hotkey `O`: mark selected/current note OK.
- Clicking timeline with modifier key, e.g. Option-click, creates note at that time.
- Clicking a note seeks to its timecode.
- Notes support edit/delete.

### 4. Export

Implement exports:

- CSV columns: Project, Version Created, Timecode, Note, Status, Author Role, Resolved In, Created At
- Plain text format grouped by General Notes and Timecoded Notes
- Markdown format suitable for Notion / Email

### 5. Speaker visualization

- Build a reusable `SpeakerRoom` component.
- Layout should approximate 7.1.4:
  - L, C, R
  - LFE
  - Lss/Rss
  - Lrs/Rrs
  - Ltf/Rtf/Ltr/Rtr or top channels
- Use existing playback meter if available; otherwise create mock channel meter state for dev.
- Each speaker has state: active level, solo, muted.
- Clicking speaker toggles selection; UI provides Solo/Mute mode.
- Group buttons: Front, Wide, Surround, Rear, Top, LFE.

## Technical constraints

- Preserve the root-cause fix: never reintroduce `AVPlayerView`, `AVPlayerLayer`, `ns_window`, `content_view_ptr`, or native video view attachment.
- UI must remain Solid/Tauri; audio backend remains audio-only.
- Keep existing playback facade pattern; frontend should use unified playback state/events.
- Do not break current output device / EQ / meter foundation.
- Use minimal, focused changes first. Avoid building cloud collaboration, auth, payment, or AI suggestions in MVP.

## Suggested implementation phases

### Phase 0 — Fork and rename
- Create new repo/project from current Pik Player.
- Rename product to Pik Player Pro / Pik Review Player.
- Keep build working.
- Remove consumer-only clutter only if it conflicts with review workflow.

### Phase 1 — Review shell UI
- Replace main layout with speaker room + timeline + right notes panel.
- Keep playback working.
- Add keyboard shortcut handling.

### Phase 2 — Project/version/note DB
- Add SQLite tables for Project, Version, Note.
- Drag import creates project/version.
- Add CRUD for notes.

### Phase 3 — Timecoded workflow
- Hotkey creates note at current time.
- Timeline click/Option-click creates note.
- Click note seeks.
- Playback highlights nearby note.

### Phase 4 — Version review workflow
- V2 import retains V1 notes.
- Add Done / OK status and resolvedInVersionId.
- Filter notes by All / Open / Done / Current Version.

### Phase 5 — Export
- CSV export.
- Plain text export.
- Markdown export.

### Phase 6 — Speaker room v1
- Build visual speaker room with meter glow.
- Solo/Mute by speaker or group.
- Wire to backend channel/meter state where possible.

## Acceptance criteria for MVP

- User can drag V1 file, play it, add timecoded notes, edit notes, export CSV/text.
- User can drag V2 file into same project, play V2, still see V1 notes, and mark them Done/OK.
- Speaker room shows animated meter response during playback or with mocked dev meter if audio meter is unavailable.
- App builds as a macOS `.app` with `npm run tauri build -- --bundles app`.
- No source or binary evidence of project-side `AVPlayerView` / `AVPlayerLayer` regression.
