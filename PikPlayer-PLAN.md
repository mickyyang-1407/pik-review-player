# Pik Player — Product Vision & Feature Plan

> **核心定位（鎖死，不可動搖）**：
> 這不是 Micky 的個人工具。這是一個**面向一般消費者的商業 macOS app**，目標是推廣 Dolby Atmos 音樂，並以此為差異化核心上架、發行、可能與大廠談合作。
>
> 下一棒 AI 必須以「商業產品」角度思考所有功能決策，不是「個人腳本」。

---

## 產品定位一句話

> **Pik Player = macOS 上最好的 Atmos 音樂播放器，兼容主流串流服務，搭載 AI 歌單與耳機 EQ。**

---

## SPINOFF-01：Pik Player Pro / Review Player（Micky 2026-06-24 新想法）

### 核心定位

在目前 Pik Player 的播放核心與 Atmos / vDSP meter 基礎上，分岔出一個給專業人士使用的版本：製作人、混音師、藝人、經紀人、A&R、導演、客戶可以把歌曲版本拖進播放器，一邊聽一邊用熱鍵或滑鼠在時間軸上寫修改 note，最後輸出成 CSV / 文字 / 可分享報告。

這不是一般消費端 Library Player，而是 **music review / mix revision / approval player**。

### 主要使用情境
- 混音師交 V1，製作人或藝人打開檔案聽，邊聽邊記「01:23:14 小鼓太刺耳，可以小一點」。
- 新版本 V2 拖進來後，系統自動建立版本記錄，播放 V2 時仍顯示 V1 的 note，方便逐點確認是否修好。
- 聽到某個點已經修好時，可用熱鍵或右側按鈕標記 `Done` / `OK`。
- 最後把 note 匯出成 CSV、純文字、Markdown、PDF 或可貼到 LINE / Email / Notion 的整理格式。

### 介面構想
- **上方 / 中央：immersive speaker room visualization**
  - 參考 Dolby / Renderer 風格的 7.1.4 房間線框圖。
  - 喇叭即時渲染，播放時依照各 channel / object / meter 音量大小發光或放大。
  - 喇叭可點擊選擇 Solo / Mute；也可分 group：Front、Wide、Surround、Rear、Top、LFE。
  - 視覺語言：深色專業監聽室、細線框、低亮度、黃綠色 meter glow，不要遊戲化。
- **下方：專業 playback bar**
  - waveform / timeline / timecode。
  - 滑鼠點擊時間軸可新增 note at timecode。
  - 熱鍵新增 note：例如 `N` / `M` / Space 組合；按下後暫停或不中斷播放可設定。
- **右側：Notes panel**
  - `General Notes`：整首歌總評，例如「整體 vocal 可以再前一點」。
  - `Timecoded Notes`：每則 note 綁定精準 timecode，例如 `01:23:14 小鼓太刺耳，可以小一點`。
  - 每則 note 有狀態：Open / Done / OK / Needs Check / Rejected。
  - 每則 note 可標記作者：Producer / Artist / Manager / Mixer / Client。
  - 播放到 note 附近時自動 highlight；點 note 可跳到該 timecode。

### 版本記憶功能
- 專案單位：一首歌 / 一個 cue / 一個廣告案 / 一個 album track。
- 版本單位：V1、V2、V3，自動依照拖入檔案順序或檔名偵測，也可手動改名。
- V2 播放時仍顯示 V1 notes，讓 reviewer 逐點確認。
- 每則 note 可以記錄它在哪個版本建立、在哪個版本被標 Done / OK。
- 支援版本比較欄位：`Created In: V1`、`Resolved In: V2`、`Status: OK`。

### 匯出格式
- CSV：給 spreadsheet / production tracker。
- Markdown / Plain Text：可貼到 Email、LINE、Slack、Notion。
- PDF report：未來可做，包含曲名、版本、reviewer、總 note、timecoded notes。
- DAW-friendly：未來可研究 Reaper markers CSV、Pro Tools marker memory locations、Logic markers 等格式。

### 資料模型草案
```text
Project
- id
- title
- client / artist / album / cue
- createdAt / updatedAt

Version
- id
- projectId
- label: V1 / V2 / Mix 03 / Master 01
- filePath
- duration
- importedAt
- checksum / fingerprint

Note
- id
- projectId
- createdInVersionId
- resolvedInVersionId?
- timecodeMs? null means general note
- body
- authorRole
- status: open / done / ok / needs_check / rejected
- createdAt / updatedAt
```

### MVP 範圍
1. 從現有 Pik Player fork / clone 出專業版專案。
2. 保留現有 Tauri + Solid + Rust playback core、AtmosPlayer、meter、output device、EQ 基礎。
3. 新增右側 Notes panel：General Notes + Timecoded Notes。
4. 新增熱鍵：當前播放位置建立 timecoded note。
5. 新增 timeline click → 建 note / seek。
6. 新增版本系統：拖入新檔可建立 V1 / V2，V2 播放時顯示 V1 notes。
7. 新增 Done / OK 狀態。
8. 新增 CSV + plain text export。
9. Speaker room visualization 第一版先用 channel meter driving UI，不做完整 Dolby object renderer。

### 重要產品決策
- 這個專業版可以和消費版分開命名、分開 repo，避免污染消費端 Pik Player 的簡潔定位。
- 消費版繼續做「一般使用者 Atmos music player」。
- 專業版聚焦「review notes + version approval + immersive monitoring visualization」。
- 第一版不用做 cloud collaboration；先做好本機 project file / SQLite / export。
- 之後可以加 Share Link、雲端同步、多 reviewer、登入、收費方案。

---

## 商業模式（Micky 的設想）

| 收入來源 | 說明 |
|---|---|
| **Atmos 差異化** | 主力賣點：其他播放器不懂 Atmos，我們懂 |
| **串流服務合作** | 與 Spotify / Tidal / Apple Music 談，透過我的 app 訂閱有折扣；我只是容器，所有串流利潤回母公司，Micky 賺價差 |
| **Premium 功能** | 高級 EQ presets / Celebrity 聯名 EQ chains / AI 功能解鎖 |
| **未來：Suno 整合** | Suno API 開放後，讓使用者在 app 內生成 AI 歌曲加入 library |

---

## IDEA-01：Output 設定

### 決定方向
- ✅ **Output 裝置選擇**：列舉 CoreAudio 裝置（Built-in、USB DAC、Dante、Bluetooth），切換立即生效
- ✅ **AUv3 效果器**（取代 VST）：macOS 原生、沙盒安全、App Store 可用
- ❌ VST3 hosting：scope 太大、App Store 不可行、改用 AUv3 即可

### Celebrity / Named EQ Chains（Micky 的新想法）
**這個非常值得做，而且是差異化賣點。**

想法：
- Micky Yang's Monitoring Curve（Tonmeister 調音視角）
- 哈曼曲線（headphone listening）
- Cinematic / Film Scoring / Studio Reference 等場景 preset
- 未來可以和其他音樂製作人聯名

實作方式：
- 每個 preset = 一組 AUv3 Parametric EQ 參數（JSON 格式）
- 用戶一鍵套用，也可以自行調整後另存
- Preset 可以是免費（基本）或 Premium（聯名版）

**Claude 的建議**：第一版先內建 5 個匿名場景 preset（Reference Monitor / Headphone / Cinematic / Warm / Flat），累積用戶後再談聯名。

---

## IDEA-02：耳機偵測 + AutoEQ

**狀態**：`✅ Accepted` — 優先做

### 技術方案
- [AutoEQ](https://github.com/jaakkopasanen/AutoEq) 授權 MIT，~5000 支耳機 ParametricEQ
- mpv：`af=lavfi=[anequalizer=...]` 注入
- AtmosPlayer：`AVAudioUnitEQ` node
- 打包 top 300 常見型號 JSON（約 5MB），其餘手動 10-band

### 流程
```
Output 選擇 → 系統偵測到耳機 → 彈出「喇叭 or 耳機？」
→ 耳機 → 搜尋品牌/型號
→ 找到 → [套用哈曼曲線] / [手動 EQ] / [跳過]
→ PlayerBar 常駐 [EQ ON/OFF] 按鈕
```

### Atmos 衝突警示
Atmos 內容 + headphone EQ 同開時，Badge 顯示 ⚠️，提示可能影響 binaural 空間感，使用者自行決定。

---

## IDEA-03：AI 功能（面向一般使用者）

**重要背景修正：這是給下載使用的一般使用者，不是 Micky 自己用。**

### 使用者 API Key 方式（正確方向）

使用者在 Settings 頁輸入自己的 OpenAI API key（或未來其他服務）：
- Key 存進 **macOS Keychain**（透過 Tauri `tauri-plugin-stronghold` 或 `security` CLI），絕不明文存檔
- App 用這個 key 呼叫 OpenAI API

### 難度評估：**中等，不難**

| 功能 | 難度 | 說明 |
|---|---|---|
| API Key 設定頁 + Keychain 儲存 | ⭐⭐ | Tauri 有 plugin，標準流程 |
| AI 對話介面（側邊欄 chat）| ⭐⭐ | HTTP 呼叫 OpenAI，串流回應 |
| 歌單生成（從 library 挑歌）| ⭐⭐⭐ | 需要把 library 摘要塞進 prompt；500 首以上需 RAG |
| Suno API 整合（生成歌曲）| ⭐⭐⭐ | API POST → poll → 下載 MP3 → 加 library；等 Suno 開放 |

### 第一版功能範圍（MVP AI）

```
Settings → AI → 輸入 OpenAI API Key → [Save to Keychain]

側邊欄 AI Chat：
使用者："幫我建一個適合工作的歌單"
→ 系統把 library 前 200 首 title/artist/album 摘要送進 prompt
→ AI 回傳歌單（track title list）
→ [一鍵播放] 按鈕

未來：
使用者："生成一首爵士風格的歌"
→ App 呼叫 Suno API → 下載 → 加入 library
```

### Claude 的懷疑與建議

**好的部分：**
- 讓使用者帶自己的 API key = app 零 AI 成本，完全合規
- OpenAI API 呼叫就是 HTTPS request，Tauri 可以直接做
- 歌單生成 + library 整合，對使用者有實際價值

**需要想清楚的問題：**
1. **隱私**：使用者的 listening history + library 會送到 OpenAI server。需要在 Privacy Policy 明確揭露。
2. **大型 library**：500 首歌全部送進 prompt 超 context limit → 需要 embedding + vector search（第二版再做）
3. **Suno API 現況**：目前 Suno 沒有公開 API，等他們開放再接
4. **API 費用**：使用者用自己的 key，這個風險在使用者端，但 app 要顯示預估 token 消耗

---

## IDEA-04：串流服務整合（最大野心，最難）

**狀態**：`💡 Proposed` — 長期目標

### Micky 的設想
透過 Pik Player 訂閱 Spotify / Tidal / 其他串流服務，有折扣；Pik Player 只是容器，利潤回母公司，Micky 賺價差。

### Claude 的完整分析

**工程難度：極高（但比 VST 更值得投資）**

**Spotify 的現實：**
- Spotify Web Playback SDK 只授權給 Spotify 批准的 Partner
- 不能直接在 app 內播放 Spotify 串流（違反 Terms of Service）
- 合法路徑：申請成為 Spotify Partner App → 需要公司資質、審核、簽約

**可以做的（短期）：**
- Spotify / Apple Music / Last.fm **帳號連結**（OAuth）
- 讀取使用者的 playlist，把名稱顯示在 app 裡（不播放）
- **「找本地版本」**：用 Spotify playlist 的 title/artist 去搜 local library，有的話播本地 Atmos 版本

**中期（商業談判路徑）：**
- 先把 app 做好、有使用者、有 DAU 數字
- 帶著數字去找 Tidal（他們最重視 Hi-Fi 和 Atmos）
- 提案：Tidal Atmos 內容透過 Pik Player 播放，因為我們的 Atmos 渲染最好
- Tidal 比 Spotify 更容易談，因為 Atmos 就是他們的賣點

**Claude 的建議順序：**
1. 現在：把 app 做好，本地播放 Atmos 完美
2. 6 個月後：申請 Tidal Developer API
3. 1 年後：有用戶數後去談 Tidal 合作 → Spotify
4. 最後：subscription 整合才有談判籌碼

**不建議一開始就寫 Spotify 整合 code，因為 Spotify 的 SDK Terms 很嚴格，隨時可能被下架。**

---

## 優先順序建議表

| 優先 | 功能 | 理由 |
|---|---|---|
| 🔴 P0 | 手動測試 + bug fix | 先讓 app 可用 |
| 🔴 P0 | Output 裝置選擇 | 基礎功能，Tonmeister 必要 |
| 🟠 P1 | 耳機 EQ + AutoEQ | 最高差異化，使用者立刻感受到 |
| 🟠 P1 | 5 個 EQ 場景 preset | 第一版聯名 EQ 的地基 |
| 🟡 P2 | AI API Key + 歌單生成 | 有趣 feature，留住進階用戶 |
| 🟡 P2 | Celebrity/Named EQ chains | 需要先找到聯名對象 |
| 🟢 P3 | 串流服務 OAuth 連結 | 讀 playlist，不播放 |
| ⚪ P4 | Suno API 整合 | 等 API 開放 |
| ⚪ P4 | 串流服務播放合作 | 等有用戶數再談 |

---

## 商業產品開發的額外 TODO（非功能性）

這些是做商業 app 不可避免的，提早想：

- [ ] **App 名稱 / 商標**：「Pik Player」是否已註冊？
- [ ] **App Store 分類**：Music 類別，需要 Apple 審核
- [ ] **Privacy Policy**：AI 功能送資料到 OpenAI，必須揭露
- [ ] **付費機制**：App Store IAP（Apple 抽 30%）vs 直接官網購買
- [ ] **版本管理**：免費版（基本播放）vs 付費版（EQ + AI）
- [ ] **Atmos 授權**：「Dolby Atmos」是商標，app UI 上使用需要符合 Dolby 的 branding guide

---

*最後更新：2026-06-22 | 維護：Micky Yang + Claude Code*
*新想法隨時丟給 Claude，追加到這個文件底部，不直接進 TASKS*
