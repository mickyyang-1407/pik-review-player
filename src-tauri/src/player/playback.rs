use super::atmos::AtmosPlayer;
use super::mpv::MpvPlayer;
use serde::Serialize;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Listener};

#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PlaybackEngine {
    Mpv,
    Atmos,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PlaybackStatus {
    Idle,
    Loading,
    Playing,
    Paused,
    Stopped,
    Error,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaybackState {
    pub engine: Option<PlaybackEngine>,
    pub status: PlaybackStatus,
    pub file_path: Option<String>,
    pub file_name: Option<String>,
    pub is_atmos_source: bool,
    pub playback_mode: String,
    pub position: f64,
    pub duration: f64,
    pub volume: f64,
    pub error: Option<String>,
    pub output_device_uid: Option<String>,
    pub eq_profile: Option<super::eq::EqProfile>,
    pub eq_enabled: bool,
}

impl Default for PlaybackState {
    fn default() -> Self {
        Self {
            engine: None,
            status: PlaybackStatus::Idle,
            file_path: None,
            file_name: None,
            is_atmos_source: false,
            playback_mode: "unknown".to_string(),
            position: 0.0,
            duration: 0.0,
            volume: 100.0,
            error: None,
            output_device_uid: None,
            eq_profile: None,
            eq_enabled: false,
        }
    }
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct PositionPayload {
    position: f64,
    duration: f64,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct MeterPayload {
    available: bool,
    mode: String,
    channels: Vec<MeterChannel>,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct MeterChannel {
    label: String,
    rms: f64,
    peak: f64,
}

#[derive(Clone)]
pub struct PlaybackPlayer {
    app_handle: Arc<Mutex<Option<AppHandle>>>,
    mpv: Arc<Mutex<Option<MpvPlayer>>>,
    atmos: Arc<Mutex<Option<AtmosPlayer>>>,
    state: Arc<Mutex<PlaybackState>>,
}

impl PlaybackPlayer {
    pub fn new() -> Self {
        Self {
            app_handle: Arc::new(Mutex::new(None)),
            mpv: Arc::new(Mutex::new(None)),
            atmos: Arc::new(Mutex::new(None)),
            state: Arc::new(Mutex::new(PlaybackState::default())),
        }
    }

    pub fn init(&self, app: AppHandle, mpv: MpvPlayer, atmos: AtmosPlayer) {
        *self.app_handle.lock().unwrap() = Some(app.clone());
        *self.mpv.lock().unwrap() = Some(mpv);
        *self.atmos.lock().unwrap() = Some(atmos);
        self.bridge_engine_events(app.clone());
        self.start_observer(app);
    }

    fn bridge_engine_events(&self, app: AppHandle) {
        // mpv:ended → playback:ended
        {
            let state_arc = self.state.clone();
            let app_clone = app.clone();
            app.listen("mpv:ended", move |_| {
                let snapshot = {
                    let mut state = state_arc.lock().unwrap();
                    if state.engine == Some(PlaybackEngine::Mpv) {
                        state.status = PlaybackStatus::Stopped;
                        state.position = 0.0;
                        Some(state.clone())
                    } else {
                        None
                    }
                };
                if let Some(s) = snapshot {
                    let _ = app_clone.emit("playback:state", s);
                    let _ = app_clone.emit("playback:ended", ());
                }
            });
        }

        // av:ended → playback:ended
        {
            let state_arc = self.state.clone();
            let app_clone = app.clone();
            app.listen("av:ended", move |_| {
                let snapshot = {
                    let mut state = state_arc.lock().unwrap();
                    if state.engine == Some(PlaybackEngine::Atmos) {
                        state.status = PlaybackStatus::Stopped;
                        state.position = 0.0;
                        Some(state.clone())
                    } else {
                        None
                    }
                };
                if let Some(s) = snapshot {
                    let _ = app_clone.emit("playback:state", s);
                    let _ = app_clone.emit("playback:ended", ());
                }
            });
        }
    }

    pub fn load(&self, path: String) -> Result<(), String> {
        let file_name = Path::new(&path)
            .file_name()
            .and_then(|name| name.to_str())
            .map(|name| name.to_string())
            .unwrap_or_else(|| path.clone());

        let ext = Path::new(&path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        if !Path::new(&path).exists() {
            self.update_state(|s| {
                s.file_path = Some(path.clone());
                s.file_name = Some(file_name.clone());
                s.status = PlaybackStatus::Error;
                s.error = Some("File not found".to_string());
            });
            self.emit_error("File not found".to_string());
            return Err("File not found".to_string());
        }

        let known_ext = matches!(
            ext.as_str(),
            "mp3" | "flac" | "wav" | "ogg" | "m4a" | "aac" | "opus"
                | "aiff" | "mka" | "mxf" | "mp4" | ""
        );
        if !known_ext {
            let msg = format!("Unsupported format (.{})", ext);
            self.update_state(|s| {
                s.file_path = Some(path.clone());
                s.file_name = Some(file_name.clone());
                s.status = PlaybackStatus::Error;
                s.error = Some(msg.clone());
            });
            self.emit_error(msg.clone());
            return Err(msg);
        }

        let engine = detect_engine(&path);
        let is_atmos_source = match engine {
            PlaybackEngine::Atmos => {
                let e = ext.as_str();
                let lower = path.to_lowercase();
                e == "mp4" || e == "m4a" || (e == "wav" && (lower.contains("adm") || lower.contains("atmos")))
            },
            _ => false,
        };

        self.update_state(|state| {
            state.engine = Some(engine);
            state.status = PlaybackStatus::Loading; // We will change this to Paused after load
            state.file_path = Some(path.clone());
            state.file_name = Some(file_name.clone());
            state.is_atmos_source = is_atmos_source;
            state.playback_mode = if is_atmos_source {
                "unknown".to_string()
            } else {
                "stereo".to_string()
            };
            state.position = 0.0;
            state.duration = 0.0;
            state.error = None;
        });

        let load_result = match engine {
            PlaybackEngine::Atmos => {
                if let Some(mpv) = self.mpv.lock().unwrap().as_ref() {
                    let _ = mpv.stop();
                }
                let atmos = self.atmos.lock().unwrap().as_ref().cloned().ok_or("Atmos player unavailable")?;
                atmos.load(&path).map(|_| {
                    let _ = atmos.pause();
                })
            }
            PlaybackEngine::Mpv => {
                if let Some(atmos) = self.atmos.lock().unwrap().as_ref() {
                    let _ = atmos.stop();
                }
                let mpv = self.mpv.lock().unwrap().as_ref().cloned().ok_or("mpv player unavailable")?;
                mpv.load(path).map(|_| {
                    let _ = mpv.pause();
                })
            }
        };

        match load_result {
            Ok(()) => {
                // Re-apply stored output device on the newly loaded engine
                let state_lock = self.state.lock().unwrap();
                let uid = state_lock.output_device_uid.clone();
                let eq_enabled = state_lock.eq_enabled;
                let eq_profile = state_lock.eq_profile.clone();
                drop(state_lock);

                if let Some(uid) = uid {
                    let _ = match engine {
                        PlaybackEngine::Mpv => self.mpv.lock().unwrap().as_ref()
                            .map(|p| p.set_audio_device(&uid)),
                        PlaybackEngine::Atmos => self.atmos.lock().unwrap().as_ref()
                            .map(|p| p.set_output_device(&uid)),
                    };
                }

                // Apply EQ
                if eq_enabled && eq_profile.is_some() {
                    let _ = match engine {
                        PlaybackEngine::Mpv => self.mpv.lock().unwrap().as_ref()
                            .map(|p| p.set_eq(eq_profile.as_ref())),
                        PlaybackEngine::Atmos => self.atmos.lock().unwrap().as_ref()
                            .map(|p| p.set_eq(true, eq_profile.as_ref())),
                    };
                } else {
                    let _ = match engine {
                        PlaybackEngine::Mpv => self.mpv.lock().unwrap().as_ref()
                            .map(|p| p.set_eq(None)),
                        PlaybackEngine::Atmos => self.atmos.lock().unwrap().as_ref()
                            .map(|p| p.set_eq(false, None)),
                    };
                }

                self.update_state(|s| {
                    s.status = PlaybackStatus::Paused;
                    s.error = None;
                });
                Ok(())
            }
            Err(_raw) => {
                let msg = "Load failed".to_string();
                self.set_error(msg.clone());
                Err(msg)
            }
        }
    }

    pub fn toggle_play_pause(&self) -> Result<(), String> {
        let state = self.get_state();
        let engine = state.engine.ok_or("No file loaded")?;

        match (engine, state.status) {
            (PlaybackEngine::Atmos, PlaybackStatus::Playing) => {
                self.atmos.lock().unwrap().as_ref().ok_or("Atmos player unavailable")?.pause()?;
                self.set_status(PlaybackStatus::Paused);
            }
            (PlaybackEngine::Atmos, _) => {
                self.atmos.lock().unwrap().as_ref().ok_or("Atmos player unavailable")?.play()?;
                self.set_status(PlaybackStatus::Playing);
            }
            (PlaybackEngine::Mpv, PlaybackStatus::Playing) => {
                self.mpv.lock().unwrap().as_ref().ok_or("mpv player unavailable")?.pause()?;
                self.set_status(PlaybackStatus::Paused);
            }
            (PlaybackEngine::Mpv, _) => {
                self.mpv.lock().unwrap().as_ref().ok_or("mpv player unavailable")?.play()?;
                self.set_status(PlaybackStatus::Playing);
            }
        }

        Ok(())
    }

    pub fn play(&self) -> Result<(), String> {
        let state = self.get_state();
        let engine = state.engine.ok_or("No file loaded")?;

        match engine {
            PlaybackEngine::Atmos => {
                self.atmos.lock().unwrap().as_ref().ok_or("Atmos player unavailable")?.play()?;
                self.set_status(PlaybackStatus::Playing);
            }
            PlaybackEngine::Mpv => {
                self.mpv.lock().unwrap().as_ref().ok_or("mpv player unavailable")?.play()?;
                self.set_status(PlaybackStatus::Playing);
            }
        }

        Ok(())
    }

    pub fn stop(&self) -> Result<(), String> {
        let state = self.get_state();
        match state.engine {
            Some(PlaybackEngine::Atmos) => {
                self.atmos.lock().unwrap().as_ref().ok_or("Atmos player unavailable")?.stop()?;
            }
            Some(PlaybackEngine::Mpv) => {
                self.mpv.lock().unwrap().as_ref().ok_or("mpv player unavailable")?.stop()?;
            }
            None => {}
        }

        self.update_state(|state| {
            state.status = PlaybackStatus::Stopped;
            state.position = 0.0;
            state.duration = 0.0;
            state.error = None;
        });
        self.emit_ended();
        Ok(())
    }

    pub fn seek(&self, pos: f64) -> Result<(), String> {
        let engine = self.get_state().engine.ok_or("No file loaded")?;
        match engine {
            PlaybackEngine::Atmos => self.atmos.lock().unwrap().as_ref().ok_or("Atmos player unavailable")?.seek(pos)?,
            PlaybackEngine::Mpv => self.mpv.lock().unwrap().as_ref().ok_or("mpv player unavailable")?.seek(pos)?,
        }
        self.update_state(|state| state.position = pos);
        Ok(())
    }

    pub fn set_volume(&self, vol: f64) -> Result<(), String> {
        let clamped = vol.clamp(0.0, 130.0);
        let engine = self.get_state().engine;

        if let Some(engine) = engine {
            match engine {
                PlaybackEngine::Atmos => self.atmos.lock().unwrap().as_ref().ok_or("Atmos player unavailable")?.set_volume(clamped)?,
                PlaybackEngine::Mpv => self.mpv.lock().unwrap().as_ref().ok_or("mpv player unavailable")?.set_volume(clamped.round() as i64)?,
            }
        }

        self.update_state(|state| state.volume = clamped);
        Ok(())
    }

    pub fn get_state(&self) -> PlaybackState {
        self.state.lock().unwrap().clone()
    }

    pub fn set_output_device(&self, uid: String) -> Result<(), String> {
        let engine = self.get_state().engine;
        if let Some(eng) = engine {
            match eng {
                PlaybackEngine::Mpv => {
                    self.mpv.lock().unwrap().as_ref()
                        .ok_or("mpv player unavailable")?
                        .set_audio_device(&uid)?;
                }
                PlaybackEngine::Atmos => {
                    self.atmos.lock().unwrap().as_ref()
                        .ok_or("Atmos player unavailable")?
                        .set_output_device(&uid)?;
                }
            }
        }
        // Always store the device uid so it persists across track loads
        self.update_state(|s| {
            s.output_device_uid = if uid.is_empty() { None } else { Some(uid.clone()) };
        });
        Ok(())
    }

    pub fn set_eq_enabled(&self, enabled: bool) -> Result<(), String> {
        self.update_state(|s| s.eq_enabled = enabled);
        self.apply_eq_to_engines()
    }

    pub fn set_eq_profile(&self, profile: Option<super::eq::EqProfile>) -> Result<(), String> {
        self.update_state(|s| s.eq_profile = profile);
        self.apply_eq_to_engines()
    }

    fn apply_eq_to_engines(&self) -> Result<(), String> {
        let state = self.get_state();
        let engine = state.engine;
        let active_profile = if state.eq_enabled { state.eq_profile.as_ref() } else { None };

        if let Some(eng) = engine {
            match eng {
                PlaybackEngine::Mpv => {
                    self.mpv.lock().unwrap().as_ref()
                        .ok_or("mpv player unavailable")?
                        .set_eq(active_profile)?;
                }
                PlaybackEngine::Atmos => {
                    self.atmos.lock().unwrap().as_ref()
                        .ok_or("Atmos player unavailable")?
                        .set_eq(state.eq_enabled, active_profile)?;
                }
            }
        }
        Ok(())
    }

    fn start_observer(&self, app: AppHandle) {
        let state_arc = self.state.clone();
        let mpv_arc = self.mpv.clone();
        let atmos_arc = self.atmos.clone();

        thread::spawn(move || loop {
            let snapshot = state_arc.lock().unwrap().clone();
            if let Some(engine) = snapshot.engine {
                let position = match engine {
                    PlaybackEngine::Mpv => mpv_arc.lock().unwrap().as_ref().and_then(|mpv| mpv.position()),
                    PlaybackEngine::Atmos => atmos_arc.lock().unwrap().as_ref().and_then(|atmos| atmos.position()),
                };

                match engine {
                    PlaybackEngine::Atmos => {
                        if let Some(json) = atmos_arc.lock().unwrap().as_ref().and_then(|atmos| atmos.meter_json()) {
                            if let Ok(payload) = serde_json::from_str::<serde_json::Value>(&json) {
                                let _ = app.emit("playback:meter", payload);
                            }
                        }
                    }
                    PlaybackEngine::Mpv => {
                        let _ = app.emit("playback:meter", MeterPayload {
                            available: false,
                            mode: "unavailable".to_string(),
                            channels: vec![],
                        });
                    }
                }

                if let Some((pos, dur)) = position {
                    {
                        let mut state = state_arc.lock().unwrap();
                        state.position = pos;
                        if dur.is_finite() {
                            state.duration = dur;
                        }
                    }

                    let _ = app.emit("playback:position", PositionPayload {
                        position: pos,
                        duration: dur,
                    });
                }
            }

            thread::sleep(Duration::from_millis(250));
        });
    }

    fn set_status(&self, status: PlaybackStatus) {
        self.update_state(|state| {
            state.status = status;
            state.error = None;
        });
    }

    fn set_error(&self, error: String) {
        self.update_state(|state| {
            state.status = PlaybackStatus::Error;
            state.error = Some(error.clone());
        });
        self.emit_error(error);
    }

    fn update_state<F>(&self, update: F)
    where
        F: FnOnce(&mut PlaybackState),
    {
        let snapshot = {
            let mut state = self.state.lock().unwrap();
            update(&mut state);
            state.clone()
        };
        self.emit_state(snapshot);
    }

    fn emit_state(&self, state: PlaybackState) {
        if let Some(app) = self.app_handle.lock().unwrap().as_ref() {
            let _ = app.emit("playback:state", state);
        }
    }

    fn emit_error(&self, error: String) {
        if let Some(app) = self.app_handle.lock().unwrap().as_ref() {
            let _ = app.emit("playback:error", error);
        }
    }

    fn emit_ended(&self) {
        if let Some(app) = self.app_handle.lock().unwrap().as_ref() {
            let _ = app.emit("playback:ended", ());
        }
    }
}

fn detect_engine(path: &str) -> PlaybackEngine {
    let ext = Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        "mp4" | "m4a" | "aac" | "aiff" | "mp3" | "wav" => PlaybackEngine::Atmos,
        _ => PlaybackEngine::Mpv,
    }
}
