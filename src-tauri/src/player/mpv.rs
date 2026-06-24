use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use mpv::{MpvHandler, MpvHandlerBuilder, Event};

struct SharedMpv(MpvHandler);
unsafe impl Send for SharedMpv {}
unsafe impl Sync for SharedMpv {}

#[derive(Clone, serde::Serialize)]
struct PositionPayload {
    position: f64,
    duration: f64,
}

#[derive(Clone)]
pub struct MpvPlayer {
    app_handle: Arc<Mutex<Option<AppHandle>>>,
    mpv: Arc<Mutex<SharedMpv>>,
}

impl MpvPlayer {
    pub fn new() -> Self {
        let mut builder = MpvHandlerBuilder::new().unwrap();
        builder.set_option("vid", "no").unwrap();
        let handler = builder.build().unwrap();
        Self {
            app_handle: Arc::new(Mutex::new(None)),
            mpv: Arc::new(Mutex::new(SharedMpv(handler))),
        }
    }

    pub fn init(&self, app: AppHandle) {
        *self.app_handle.lock().unwrap() = Some(app.clone());

        let mpv_clone = self.mpv.clone();
        let app_clone = app.clone();

        thread::spawn(move || {
            let mut tick = 0;
            loop {
                let event = {
                    let mut mpv = mpv_clone.lock().unwrap();
                    mpv.0.wait_event(0.0)
                };

                if let Some(event) = event {
                    match event {
                        Event::FileLoaded => {
                            let _ = app_clone.emit("mpv:playing", ());
                        }
                        Event::EndFile(_) => {
                            let _ = app_clone.emit("mpv:ended", ());
                        }
                        Event::Shutdown => {
                            break;
                        }
                        _ => {}
                    }
                }

                if tick % 25 == 0 {
                    let (pos_res, dur_res) = {
                        let mpv = mpv_clone.lock().unwrap();
                        (
                            mpv.0.get_property::<f64>("time-pos"),
                            mpv.0.get_property::<f64>("duration"),
                        )
                    };

                    if let (Ok(pos), Ok(dur)) = (pos_res, dur_res) {
                        let _ = app_clone.emit("mpv:position", PositionPayload {
                            position: pos,
                            duration: dur,
                        });
                    }
                }

                tick = (tick + 1) % 250;
                thread::sleep(Duration::from_millis(10));
            }
        });
    }

    pub fn load(&self, path: String) -> Result<(), String> {
        let mut mpv = self.mpv.lock().unwrap();
        let result = mpv.0.command(&["loadfile", &path, "replace"]).map_err(|e| e.to_string());
        if result.is_ok() {
            let _ = mpv.0.set_property("pause", false);
        }
        result
    }

    pub fn play(&self) -> Result<(), String> {
        let mut mpv = self.mpv.lock().unwrap();
        mpv.0.set_property("pause", false).map_err(|e| e.to_string())
    }

    pub fn pause(&self) -> Result<(), String> {
        let mut mpv = self.mpv.lock().unwrap();
        mpv.0.set_property("pause", true).map_err(|e| e.to_string())
    }

    pub fn toggle_play_pause(&self) -> Result<(), String> {
        let mut mpv = self.mpv.lock().unwrap();
        let pause: bool = mpv.0.get_property("pause").unwrap_or(false);
        mpv.0.set_property("pause", !pause).map_err(|e| e.to_string())
    }

    pub fn stop(&self) -> Result<(), String> {
        let mut mpv = self.mpv.lock().unwrap();
        mpv.0.command(&["stop"]).map_err(|e| e.to_string())
    }

    pub fn seek(&self, pos: f64) -> Result<(), String> {
        let mut mpv = self.mpv.lock().unwrap();
        mpv.0.command(&["seek", &pos.to_string(), "absolute"]).map_err(|e| e.to_string())
    }

    pub fn set_volume(&self, vol: i64) -> Result<(), String> {
        let mut mpv = self.mpv.lock().unwrap();
        // mpv expects volume property as f64 generally, but i64 might work, or string
        mpv.0.set_property("volume", vol as f64).map_err(|e| e.to_string())
    }

    pub fn position(&self) -> Option<(f64, f64)> {
        let mpv = self.mpv.lock().unwrap();
        let pos = mpv.0.get_property::<f64>("time-pos").ok()?;
        let dur = mpv.0.get_property::<f64>("duration").ok()?;
        Some((pos, dur))
    }

    pub fn is_paused(&self) -> bool {
        let mpv = self.mpv.lock().unwrap();
        mpv.0.get_property("pause").unwrap_or(false)
    }

    pub fn set_audio_device(&self, uid: &str) -> Result<(), String> {
        let device = if uid.is_empty() {
            "auto".to_string()
        } else {
            format!("coreaudio/{}", uid)
        };
        let mut mpv = self.mpv.lock().unwrap();
        mpv.0.command(&["set", "audio-device", &device]).map_err(|e| e.to_string())
    }

    pub fn set_eq(&self, profile: Option<&super::eq::EqProfile>) -> Result<(), String> {
        let mut mpv = self.mpv.lock().unwrap();
        if let Some(p) = profile {
            let af_str = p.to_mpv_af_string();
            mpv.0.set_property("af", af_str.as_str()).map_err(|e| e.to_string())
        } else {
            mpv.0.set_property("af", "").map_err(|e| e.to_string())
        }
    }
}
