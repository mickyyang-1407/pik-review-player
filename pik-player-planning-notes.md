# Pik Player 規劃筆記 — 最終調整版

> 記錄時間：2026/06/22  
> 背景：消費端 macOS Atmos 音樂播放器。已用 AVFoundation 實作 ADM WAV + Atmos MP4 播放，UI 雛形包含 Tracks / Albums / Art Grid / Search / Filter。目標是先完成一個一般消費者願意長期使用的 player，本階段不展開內容商城。

---

## 0. 最終產品定位

Pik Player 不做工程 QC 工具，也不先做發燒友全功能播放器。第一版的核心價值是：

1. 讓一般使用者能安心播放 Atmos / Spatial Audio 音樂。
2. 讓使用者清楚知道現在聽到的是 Atmos、Spatial Audio，還是 Stereo fallback。
3. 讓 Library 匯入、搜尋、瀏覽、播放的體驗像一個完整 native macOS app。
4. 保留未來往 Atmos 內容商城、Hi-Res / 發燒友模式擴展的架構空間。

因此，第一版應避免被「工程級音訊功能」拖走。所有功能排序以「消費者是否馬上感覺可信、好用、完整」為準。

---

## 1. 已確認完成

- 播放 / 暫停 / 停止 / seek / 音量
- Timeline / 時間碼
- 多格式播放雛形：MP3 / FLAC / WAV / OGG / AAC / M4A / ALAC
- Atmos MP4 + ADM WAV 播放雛形：AVFoundation
- Tracks / Albums / Art Grid 瀏覽
- 即時搜尋
- Filter pills：Atmos / Electronic / Pop / Rock / Jazz / Classical / Soundtrack
- Atmos badge 視覺標示

備註：以上列為「功能雛形已存在」，不代表所有格式、metadata、錯誤狀態、輸出裝置情境都已產品化驗證。

---

## 2. 最終架構調整

### 2.1 播放狀態必須統一

不管底層使用 AVFoundation、CoreAudio、或其他 decoder，UI 層只能面對一套 Player State：

- currentTrack
- playState：idle / loading / playing / paused / stopped / error
- timeline：duration / position / buffered
- outputState：device / spatialSupport / actualPlaybackMode
- queueState
- errorState

這是第一優先級。若 state 分裂，後續 Now Playing、Mini Player、Queue、Sleep Timer、媒體鍵、錯誤恢復都會變成雙倍成本。

### 2.2 不建議長期維持 mpv + AVFoundation 雙播放引擎

最終方向建議收斂為：

- Atmos MP4 / ADM WAV：AVFoundation 優先
- Apple 原生支援格式：AVFoundation 優先
- AVFoundation 不穩或不支援的格式：用 decoder 轉 LPCM，再進統一音訊管線

短期如果現有 mpv 已能補格式缺口，可以保留為 compatibility fallback，但不要讓 UI 和產品功能直接依賴 mpv 的獨立狀態。中長期應評估移除 mpv，或至少把它包在同一個 PlaybackAdapter 之下。

### 2.3 Atmos 狀態不能只靠 badge

「檔案是 Atmos」不等於「正在以 Atmos / Spatial Audio 播放」。UI 必須區分：

- File Capability：這首歌是不是 Atmos / ADM / Spatial Audio source
- Output Capability：目前輸出裝置是否支援
- Actual Playback：現在實際是 Spatial Audio 還是 Stereo fallback

這是 Pik Player 信任感的核心。第一版可以先做到「可靠提示」，不必承諾 bit-perfect 或完整 Dolby renderer 內部狀態。

---

## 3. MVP 必做

### P0 — 信任感與 native 基礎

- Output Device 偵測與狀態顯示
  - 顯示目前輸出裝置
  - 顯示 Spatial Audio / Atmos 支援狀態
  - 顯示實際播放模式：Spatial Audio / Stereo fallback / Unknown
- Atmos fallback 策略
  - 最終建議：自動 fallback，但明確顯示狀態
  - 不用彈窗打斷播放
  - 可在狀態區提示「目前裝置不支援 Spatial Audio，已以 Stereo 播放」
- macOS Now Playing / Control Center / 媒體鍵
  - 播放資訊
  - 封面
  - Play / Pause / Next / Previous
- Drag & drop 匯入
- 空 Library / 首次啟動畫面
  - 拖入音樂
  - 選擇資料夾
  - 匯入範例素材或示範 Atmos track
- 基本錯誤狀態
  - 檔案不存在
  - 格式不支援
  - 檔案損壞
  - 權限不足

### P0.5 — Library 產品化

- 資料夾掃描與掃描進度 UI
- Background metadata scan，避免阻塞 UI
- CJK 檔名與 tag 編碼驗證
- ADM WAV sidecar metadata
  - 同資料夾 `cover.jpg` / `folder.jpg`
  - 同資料夾 `metadata.json`
  - fallback：檔名解析 + Unknown Artist
- 外接硬碟 / 檔案移動後的 missing file 狀態

---

## 4. V1 建議

### P1 — 播放體驗

- Queue / Play Next
- Shuffle / Repeat
- Mini Player
- 關閉主視窗後繼續播放
- Sleep Timer
- 斷點續播
- 基本 keyboard shortcuts
- Preferences 面板
  - Library 資料夾管理
  - Output device 顯示或選擇
  - Theme
  - 快捷鍵

### P1 — Atmos 消費端體驗

- Signal Chain 面板
  - Source：ADM WAV 24bit/48kHz / Atmos MP4 等
  - Decoder：Apple / AVFoundation / fallback decoder
  - Output：AirPods / HomePod / Built-in / HDMI
  - Mode：Spatial Audio / Stereo fallback
- Hear the Difference
  - 一鍵切換 Atmos source 與 Stereo fallback 的比較體驗
  - 定位為體驗功能，不是工程測量工具
- Head Tracking 狀態提示
  - 若 macOS API 能可靠取得再做
  - 不可靠時不要硬顯示

### P1 — Library 瀏覽

- 三欄式瀏覽：Sidebar -> List -> Detail
- Albums / Artists / Genres
- Sort / Filter persistence
- 封面快取與縮圖生成

---

## 5. V1.5 / V2 延後

這些功能有價值，但不該阻塞第一版。

- Gapless 嚴格模式
- Smart Crossfade
- ReplayGain / Loudness normalization
- Hi-Res / bit-perfect 模式
- Exclusive Mode / Hog Mode
- External AVR passthrough
- Audio Visualizer
- Desktop widget
- Lyrics mode
- 自動更新 Sparkle
- 內容商城 / 帳號 / 購買流程

---

## 6. 重要風險與處理方式

### 6.1 Gapless

風險：AVQueuePlayer / AVFoundation 在部分壓縮格式或 Atmos 檔案上可能有 micro-gap。

最終調整：

- MVP 不承諾 gapless。
- 先實測 Atmos MP4、ADM WAV、ALAC、FLAC、AAC。
- 若 gapless 不穩，V1.5 提供 Smart Crossfade，比硬做工程級 gapless 更符合消費端期待。

### 6.2 FLAC / OGG / 多聲道 WAV

風險：AVFoundation 對所有目標格式的支援度需要逐一驗證。

最終調整：

- 建立格式測試矩陣。
- 若 AVFoundation 不穩，使用 decoder -> LPCM -> 統一管線。
- UI 不暴露「這首歌由哪個引擎播放」；只顯示使用者需要知道的狀態。

### 6.3 Atmos fallback

風險：使用者以為自己聽到 Atmos，但實際是 Stereo。

最終調整：

- 預設自動 fallback。
- UI 必須清楚顯示實際模式。
- 不用頻繁彈窗。
- 在 Signal Chain / 狀態 pill 裡提供可展開說明。

### 6.4 ReplayGain 與 Atmos

風險：Atmos object-based audio 套用 ReplayGain 可能造成 clipping 或破壞 metadata 平衡。

最終調整：

- MVP 不做 ReplayGain。
- 未來若做，Atmos / ADM 預設 bypass。
- Loudness normalization 只先對 Stereo 曲目啟用。

### 6.5 ADM WAV metadata

風險：ADM WAV 通常沒有一般音樂 app 期待的 artist / album / cover。

最終調整：

- Sidecar 是必做，不是加分項。
- 規格簡單即可：`cover.jpg` / `folder.jpg` / `metadata.json`。
- 沒有 sidecar 時要有好看的 Unknown 狀態，不要整個 library 像壞掉。

### 6.6 CJK 與 tag encoding

風險：中文、日文、Big5 / GBK / UTF-8 混雜會讓 Library 掃描看起來很廉價。

最終調整：

- 加入 CJK 檔名測試素材。
- 掃描失敗時保留原檔名，不要丟成空字串或亂碼。
- Metadata 解析失敗要能回退。

---

## 7. 最終決策

- Atmos fallback：採「自動 fallback + 明確狀態顯示」
- 架構方向：UI 層統一 Player State；底層可短期多 adapter，但不可讓產品功能分裂
- mpv：可短期保留為相容性 fallback；中長期評估移除或封裝
- Gapless：MVP 不承諾；先實測，必要時用 Smart Crossfade 解消費端痛點
- ReplayGain：MVP 不做；未來 Atmos 預設 bypass
- ADM WAV：必做 sidecar metadata
- Output 狀態：P0 必做，這比更多視覺化功能重要
- Preferences：V1 必做，否則 app 會停留在 demo 感
- 商城：暫緩，不進入 player MVP

---

## 8. 建議開發順序

### Phase 1 — 可信播放核心

1. 統一 Player State / PlaybackAdapter 介面
2. Output device + actual playback mode 狀態
3. Now Playing / media keys
4. 基本錯誤狀態
5. Drag & drop 匯入

### Phase 2 — Library 產品化

1. Folder import + scan progress
2. Background metadata scanner
3. ADM WAV sidecar
4. Cover cache
5. Missing file / external disk handling
6. Empty state / onboarding

### Phase 3 — 消費端播放體驗

1. Queue / Play Next
2. Shuffle / Repeat
3. Mini Player
4. Sleep Timer
5. Preferences
6. 三欄式 Library

### Phase 4 — Atmos differentiation

1. Signal Chain
2. Hear the Difference
3. Head Tracking 狀態
4. Smart Crossfade
5. Gapless / Hi-Res / Exclusive Mode 研究

---

## 9. 下一步檢查清單

- [ ] 盤點目前播放層是否已經分成 mpv / AVFoundation 兩套 state
- [ ] 建立 10-15 個測試音檔矩陣：Atmos MP4、ADM WAV、ALAC、FLAC、OGG、AAC、MP3、多聲道 WAV、CJK 檔名、壞檔
- [ ] 實測 macOS 輸出裝置偵測 API 可取得到什麼程度
- [ ] 實測 Control Center Spatial Audio 開關是否能被可靠感知
- [ ] 設計狀態 pill 文案：Atmos / Spatial Audio / Stereo fallback / Unknown
- [ ] 定義 `metadata.json` sidecar schema
- [ ] 定義 Player State 與 PlaybackAdapter 介面

---

## 10. 最後結論

Pik Player 第一版最該打穿的不是「支援最多音訊黑科技」，而是「使用者相信它真的在正確播放」。  

所以最後調整是：把 Output 狀態、fallback 說明、Now Playing、Library 匯入與錯誤處理提前；把 gapless、exclusive mode、ReplayGain、visualizer、商城往後放。Atmos 是賣點，但必須以清楚、可信、不中斷的消費端體驗呈現，而不是堆工程名詞。
