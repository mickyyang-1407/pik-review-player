use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter};
use serde::Serialize;
use std::ffi::{CStr, CString};

extern "C" {
    fn atmos_create(path: *const std::os::raw::c_char) -> *mut std::ffi::c_void;
    fn atmos_destroy(player: *mut std::ffi::c_void);
    fn atmos_play(player: *mut std::ffi::c_void);
    fn atmos_pause(player: *mut std::ffi::c_void);
    fn atmos_set_volume(player: *mut std::ffi::c_void, volume: f32);
    fn atmos_seek(player: *mut std::ffi::c_void, position: f64);
    fn atmos_get_position(player: *mut std::ffi::c_void) -> f64;
    fn atmos_get_duration(player: *mut std::ffi::c_void) -> f64;
    fn atmos_is_playing(player: *mut std::ffi::c_void) -> std::os::raw::c_int;
    fn atmos_set_output_device(player: *mut std::ffi::c_void, device_uid: *const std::os::raw::c_char);
    fn atmos_set_eq(player: *mut std::ffi::c_void, enabled: std::os::raw::c_int, preamp: f32, bands_json: *const std::os::raw::c_char);
    fn atmos_get_meter_json(player: *mut std::ffi::c_void) -> *mut std::os::raw::c_char;
    pub fn audio_list_output_devices() -> *mut std::os::raw::c_char;
    pub fn free_audio_devices_json(ptr: *mut std::os::raw::c_char);
    pub fn audio_is_headphone_connected() -> std::os::raw::c_int;
}

#[derive(Clone, Serialize)]
struct PositionPayload {
    position: f64,
    duration: f64,
}

#[derive(Clone)]
pub struct AtmosPlayer {
    player: Arc<Mutex<Option<usize>>>,
    app_handle: Arc<Mutex<Option<AppHandle>>>,
    time_observer_running: Arc<Mutex<bool>>,
}

impl AtmosPlayer {
    pub fn new() -> Self {
        Self {
            player: Arc::new(Mutex::new(None)),
            app_handle: Arc::new(Mutex::new(None)),
            time_observer_running: Arc::new(Mutex::new(false)),
        }
    }

    pub fn init(&self, app: AppHandle) {
        *self.app_handle.lock().unwrap() = Some(app);
    }

    pub fn load(&self, path: &str) -> Result<(), String> {
        let app = self.app_handle.lock().unwrap().as_ref().cloned().ok_or("No app handle")?;
        
        self.stop()?;

        let c_path = CString::new(path).map_err(|e| e.to_string())?;
        
        let player_ptr = unsafe { atmos_create(c_path.as_ptr()) };

        let mut p = self.player.lock().unwrap();
        *p = Some(player_ptr as usize);
        drop(p);
        
        self.start_observer(app);
        
        Ok(())
    }

    fn start_observer(&self, app: AppHandle) {
        let mut running = self.time_observer_running.lock().unwrap();
        if *running {
            return;
        }
        *running = true;
        
        let player_arc = self.player.clone();
        let running_arc = self.time_observer_running.clone();
        
        std::thread::spawn(move || {
            let mut was_playing = false;
            loop {
                if !*running_arc.lock().unwrap() {
                    break;
                }
                
                let player_ptr = *player_arc.lock().unwrap();
                if let Some(ptr) = player_ptr {
                    let ptr = ptr as *mut std::ffi::c_void;
                    let pos = unsafe { atmos_get_position(ptr) };
                    let dur = unsafe { atmos_get_duration(ptr) };
                    let is_playing = unsafe { atmos_is_playing(ptr) } != 0;
                    
                    if is_playing {
                        let _ = app.emit("av:position", PositionPayload { position: pos, duration: dur });
                        if !was_playing {
                            let _ = app.emit("av:playing", ());
                            was_playing = true;
                        }
                    } else if was_playing {
                        let _ = app.emit("av:paused", ());
                        was_playing = false;
                    }
                    
                    if dur > 0.0 && pos >= dur - 0.1 && is_playing {
                        let _ = app.emit("av:ended", ());
                    }
                } else {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(250));
            }
        });
    }

    pub fn play(&self) -> Result<(), String> {
        if let Some(player) = *self.player.lock().unwrap() {
            unsafe { atmos_play(player as *mut std::ffi::c_void); }
        }
        Ok(())
    }

    pub fn pause(&self) -> Result<(), String> {
        if let Some(player) = *self.player.lock().unwrap() {
            unsafe { atmos_pause(player as *mut std::ffi::c_void); }
        }
        Ok(())
    }

    pub fn set_volume(&self, vol: f64) -> Result<(), String> {
        if let Some(player) = *self.player.lock().unwrap() {
            let volume = (vol / 100.0) as f32;
            unsafe { atmos_set_volume(player as *mut std::ffi::c_void, volume); }
        }
        Ok(())
    }

    pub fn seek(&self, pos: f64) -> Result<(), String> {
        if let Some(player) = *self.player.lock().unwrap() {
            unsafe { atmos_seek(player as *mut std::ffi::c_void, pos); }
        }
        Ok(())
    }

    pub fn position(&self) -> Option<(f64, f64)> {
        let player = (*self.player.lock().unwrap())?;
        let ptr = player as *mut std::ffi::c_void;
        let pos = unsafe { atmos_get_position(ptr) };
        let dur = unsafe { atmos_get_duration(ptr) };
        Some((pos, dur))
    }

    pub fn meter_json(&self) -> Option<String> {
        let player = (*self.player.lock().unwrap())?;
        let ptr = player as *mut std::ffi::c_void;
        let json_ptr = unsafe { atmos_get_meter_json(ptr) };
        if json_ptr.is_null() {
            return None;
        }
        let json = unsafe { CStr::from_ptr(json_ptr).to_string_lossy().into_owned() };
        unsafe { free_audio_devices_json(json_ptr); }
        Some(json)
    }

    pub fn is_playing(&self) -> bool {
        if let Some(player) = *self.player.lock().unwrap() {
            unsafe { atmos_is_playing(player as *mut std::ffi::c_void) != 0 }
        } else {
            false
        }
    }
    
    pub fn stop(&self) -> Result<(), String> {
        let mut p = self.player.lock().unwrap();
        if let Some(player) = *p {
            unsafe { atmos_destroy(player as *mut std::ffi::c_void); }
            *p = None;
        }
        *self.time_observer_running.lock().unwrap() = false;
        Ok(())
    }

    pub fn set_output_device(&self, uid: &str) -> Result<(), String> {
        if let Some(player) = *self.player.lock().unwrap() {
            let c_uid = std::ffi::CString::new(uid).map_err(|e| e.to_string())?;
            unsafe { atmos_set_output_device(player as *mut std::ffi::c_void, c_uid.as_ptr()); }
        }
        Ok(())
    }

    pub fn set_eq(&self, enabled: bool, profile: Option<&super::eq::EqProfile>) -> Result<(), String> {
        if let Some(player) = *self.player.lock().unwrap() {
            let preamp = profile.map(|p| p.preamp as f32).unwrap_or(0.0);
            let json_str = if let Some(p) = profile {
                serde_json::to_string(&p.bands).unwrap_or_else(|_| "[]".to_string())
            } else {
                "[]".to_string()
            };
            let c_json = std::ffi::CString::new(json_str).map_err(|e| e.to_string())?;
            unsafe {
                atmos_set_eq(
                    player as *mut std::ffi::c_void,
                    if enabled { 1 } else { 0 },
                    preamp,
                    c_json.as_ptr()
                );
            }
        }
        Ok(())
    }
}
