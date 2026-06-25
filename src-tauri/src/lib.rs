pub mod player;
pub mod database;
pub mod scanner;

use player::mpv::MpvPlayer;
use player::atmos::{AtmosPlayer, audio_list_output_devices, free_audio_devices_json, audio_is_headphone_connected};
use player::playback::{PlaybackPlayer, PlaybackState};
use database::{Database, Track};
use tauri::{Manager, Emitter};
use std::ffi::CStr;
use std::sync::Arc;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct AudioDevice {
    uid: String,
    name: String,
    is_default: bool,
}

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct ImportProgress {
    scanned: u32,
    total: u32,
    current: String,
}

fn find_audio_files(dir: &std::path::Path) -> Vec<std::path::PathBuf> {
    const AUDIO_EXTS: &[&str] = &[
        "mp3", "flac", "wav", "ogg", "m4a", "aac", "opus", "aiff", "mka", "mxf", "mp4",
    ];
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            // Use entry.metadata() (does NOT follow symlinks) to avoid symlink loops.
            let Ok(meta) = entry.metadata() else { continue };
            if meta.is_dir() {
                files.extend(find_audio_files(&p));
            } else if meta.is_file() {
                let ext = p.extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("")
                    .to_lowercase();
                if AUDIO_EXTS.contains(&ext.as_str()) {
                    files.push(p);
                }
            }
            // symlinks (meta.is_symlink() || (!is_dir && !is_file)) are silently skipped
        }
    }
    files
}

pub enum PlaybackEngine {
    Mpv,
    AtmosAVPlayer,
}

pub fn detect_format(path: &str) -> PlaybackEngine {
    let lower = path.to_lowercase();
    let ext = std::path::Path::new(path).extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");
    
    match ext {
        "mp4" => PlaybackEngine::AtmosAVPlayer,
        "wav" => {
            if lower.contains("adm") || lower.contains("atmos") {
                PlaybackEngine::AtmosAVPlayer
            } else {
                PlaybackEngine::Mpv
            }
        }
        _ => PlaybackEngine::Mpv
    }
}

#[tauri::command]
fn player_load(path: String, state: tauri::State<'_, MpvPlayer>) -> Result<(), String> {
    state.load(path)
}

#[tauri::command]
fn player_toggle_play_pause(state: tauri::State<'_, MpvPlayer>) -> Result<(), String> {
    state.toggle_play_pause()
}

#[tauri::command]
fn player_stop(state: tauri::State<'_, MpvPlayer>) -> Result<(), String> {
    state.stop()
}

#[tauri::command]
fn player_seek(pos: f64, state: tauri::State<'_, MpvPlayer>) -> Result<(), String> {
    state.seek(pos)
}

#[tauri::command]
fn player_set_volume(vol: i64, state: tauri::State<'_, MpvPlayer>) -> Result<(), String> {
    state.set_volume(vol)
}

#[tauri::command]
fn atmos_load(path: String, state: tauri::State<'_, AtmosPlayer>) -> Result<(), String> {
    state.load(&path)
}

#[tauri::command]
fn atmos_play(state: tauri::State<'_, AtmosPlayer>) -> Result<(), String> {
    state.play()
}

#[tauri::command]
fn atmos_pause(state: tauri::State<'_, AtmosPlayer>) -> Result<(), String> {
    state.pause()
}

#[tauri::command]
fn atmos_seek(pos: f64, state: tauri::State<'_, AtmosPlayer>) -> Result<(), String> {
    state.seek(pos)
}

#[tauri::command]
fn atmos_set_volume(vol: f64, state: tauri::State<'_, AtmosPlayer>) -> Result<(), String> {
    state.set_volume(vol)
}

#[tauri::command]
fn atmos_stop(state: tauri::State<'_, AtmosPlayer>) -> Result<(), String> {
    state.stop()
}

#[tauri::command]
fn playback_load(
    path: String,
    state: tauri::State<'_, PlaybackPlayer>,
    db: tauri::State<'_, Arc<Database>>,
) -> Result<(), String> {
    // Scan metadata and add to library (best-effort, failure is non-fatal)
    let meta = scanner::scan_file(&path);
    let file_stem = std::path::Path::new(&path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_string();
    let format = std::path::Path::new(&path)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    let track = match meta {
        Ok(m) => Track {
            id: 0,
            path: path.clone(),
            title: m.title.or(Some(file_stem)),
            artist: m.artist,
            album: m.album,
            duration: m.duration,
            format: m.format,
            cover_path: m.cover_path,
            added_at: String::new(),
        },
        Err(_) => Track {
            id: 0,
            path: path.clone(),
            title: Some(file_stem),
            artist: None,
            album: None,
            duration: 0.0,
            format,
            cover_path: None,
            added_at: String::new(),
        },
    };
    let _ = db.insert(&track);

    state.load(path)
}

#[tauri::command]
fn library_get_tracks(db: tauri::State<'_, Arc<Database>>) -> Result<Vec<Track>, String> {
    db.query().map_err(|e| e.to_string())
}

#[tauri::command]
fn library_search_tracks(
    query: String,
    db: tauri::State<'_, Arc<Database>>,
) -> Result<Vec<Track>, String> {
    db.search(&query).map_err(|e| e.to_string())
}

#[tauri::command]
fn library_remove_track(
    id: i64,
    db: tauri::State<'_, Arc<Database>>,
) -> Result<(), String> {
    db.delete(id).map_err(|e| e.to_string())
}

#[tauri::command]
async fn library_import_folder(
    path: String,
    app: tauri::AppHandle,
    db: tauri::State<'_, Arc<Database>>,
) -> Result<u32, String> {
    // Clone Arc so the blocking closure can own it (State borrow cannot cross await)
    let db_arc = Arc::clone(&*db);

    tauri::async_runtime::spawn_blocking(move || -> Result<u32, String> {
        let dir = std::path::Path::new(&path);
        if !dir.is_dir() {
            return Err("Not a directory".to_string());
        }

        let files = find_audio_files(dir);
        let total = files.len() as u32;
        let mut inserted = 0u32;

        for (i, file_path) in files.iter().enumerate() {
            let path_str = file_path.to_string_lossy().to_string();
            let file_stem = file_path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();
            let current_name = file_path.file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();
            let format = file_path.extension()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_lowercase();

            let _ = app.emit("library:import-progress", ImportProgress {
                scanned: (i + 1) as u32,
                total,
                current: current_name,
            });

            let meta = scanner::scan_file(file_path);
            let track = match meta {
                Ok(m) => Track {
                    id: 0,
                    path: path_str,
                    title: m.title.or(Some(file_stem)),
                    artist: m.artist,
                    album: m.album,
                    duration: m.duration,
                    format: m.format,
                    cover_path: m.cover_path,
                    added_at: String::new(),
                },
                Err(_) => Track {
                    id: 0,
                    path: path_str,
                    title: Some(file_stem),
                    artist: None,
                    album: None,
                    duration: 0.0,
                    format,
                    cover_path: None,
                    added_at: String::new(),
                },
            };

            if db_arc.insert(&track).is_ok() {
                inserted += 1;
            }
        }

        Ok(inserted)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
fn output_list_devices() -> Result<Vec<AudioDevice>, String> {
    let json_ptr = unsafe { audio_list_output_devices() };
    if json_ptr.is_null() {
        return Ok(vec![]);
    }
    let json_str = unsafe { CStr::from_ptr(json_ptr).to_string_lossy().into_owned() };
    unsafe { free_audio_devices_json(json_ptr); }
    serde_json::from_str::<Vec<AudioDevice>>(&json_str).map_err(|e| e.to_string())
}

#[tauri::command]
fn output_is_headphone_connected() -> Result<bool, String> {
    let is_connected = unsafe { audio_is_headphone_connected() };
    Ok(is_connected != 0)
}

#[tauri::command]
fn output_set_device(
    uid: String,
    state: tauri::State<'_, PlaybackPlayer>,
) -> Result<(), String> {
    state.set_output_device(uid)
}

#[tauri::command]
fn playback_toggle_play_pause(state: tauri::State<'_, PlaybackPlayer>) -> Result<(), String> {
    state.toggle_play_pause()
}

#[tauri::command]
fn playback_play(state: tauri::State<'_, PlaybackPlayer>) -> Result<(), String> {
    state.play()
}

#[tauri::command]
fn playback_stop(state: tauri::State<'_, PlaybackPlayer>) -> Result<(), String> {
    state.stop()
}

#[tauri::command]
fn playback_seek(pos: f64, state: tauri::State<'_, PlaybackPlayer>) -> Result<(), String> {
    state.seek(pos)
}

#[tauri::command]
fn playback_set_volume(vol: f64, state: tauri::State<'_, PlaybackPlayer>) -> Result<(), String> {
    state.set_volume(vol)
}

#[tauri::command]
fn playback_get_state(state: tauri::State<'_, PlaybackPlayer>) -> Result<PlaybackState, String> {
    Ok(state.get_state())
}

#[tauri::command]
fn playback_set_eq_enabled(enabled: bool, state: tauri::State<'_, PlaybackPlayer>) -> Result<(), String> {
    state.set_eq_enabled(enabled)
}

#[tauri::command]
fn playback_set_eq_profile(profile: Option<player::eq::EqProfile>, state: tauri::State<'_, PlaybackPlayer>) -> Result<(), String> {
    state.set_eq_profile(profile)
}


#[tauri::command]
fn review_create_project(title: String, client: Option<String>, artist: Option<String>, album: Option<String>, cue: Option<String>, db: tauri::State<'_, Arc<Database>>) -> Result<database::Project, String> {
    db.create_project(&title, client.as_deref(), artist.as_deref(), album.as_deref(), cue.as_deref()).map_err(|e| e.to_string())
}

#[tauri::command]
fn review_get_project(id: i64, db: tauri::State<'_, Arc<Database>>) -> Result<database::Project, String> {
    db.get_project(id).map_err(|e| e.to_string())
}

#[tauri::command]
fn review_list_projects(db: tauri::State<'_, Arc<Database>>) -> Result<Vec<database::Project>, String> {
    db.list_projects().map_err(|e| e.to_string())
}

#[tauri::command]
fn review_update_project(id: i64, title: Option<String>, client: Option<String>, artist: Option<String>, album: Option<String>, cue: Option<String>, db: tauri::State<'_, Arc<Database>>) -> Result<(), String> {
    db.update_project(id, title.as_deref(), client.as_deref(), artist.as_deref(), album.as_deref(), cue.as_deref()).map_err(|e| e.to_string())
}

#[tauri::command]
fn review_delete_project(id: i64, db: tauri::State<'_, Arc<Database>>) -> Result<(), String> {
    db.delete_project(id).map_err(|e| e.to_string())
}

#[tauri::command]
fn review_create_version(project_id: i64, label: String, file_path: String, duration: Option<f64>, checksum: Option<String>, db: tauri::State<'_, Arc<Database>>) -> Result<database::Version, String> {
    db.create_version(project_id, &label, &file_path, duration, checksum.as_deref()).map_err(|e| e.to_string())
}

#[tauri::command]
fn review_get_versions(project_id: i64, db: tauri::State<'_, Arc<Database>>) -> Result<Vec<database::Version>, String> {
    db.get_versions(project_id).map_err(|e| e.to_string())
}

#[tauri::command]
fn review_delete_version(id: i64, db: tauri::State<'_, Arc<Database>>) -> Result<(), String> {
    db.delete_version(id).map_err(|e| e.to_string())
}

#[tauri::command]
fn review_create_note(project_id: i64, created_in_version_id: Option<i64>, timecode_ms: Option<f64>, body: String, author_role: Option<String>, status: Option<String>, db: tauri::State<'_, Arc<Database>>) -> Result<database::ReviewNote, String> {
    db.create_note(project_id, created_in_version_id, timecode_ms, &body, author_role.as_deref(), status.as_deref()).map_err(|e| e.to_string())
}

#[tauri::command]
fn review_get_notes(project_id: i64, db: tauri::State<'_, Arc<Database>>) -> Result<Vec<database::ReviewNote>, String> {
    db.get_notes(project_id).map_err(|e| e.to_string())
}

#[tauri::command]
fn review_update_note_status(id: i64, status: String, db: tauri::State<'_, Arc<Database>>) -> Result<(), String> {
    db.update_note_status(id, &status).map_err(|e| e.to_string())
}

#[tauri::command]
fn review_update_note_body(id: i64, body: String, db: tauri::State<'_, Arc<Database>>) -> Result<(), String> {
    db.update_note_body(id, &body).map_err(|e| e.to_string())
}

#[tauri::command]
fn review_delete_note(id: i64, db: tauri::State<'_, Arc<Database>>) -> Result<(), String> {
    db.delete_note(id).map_err(|e| e.to_string())
}

#[tauri::command]
fn review_update_note_resolved_in(id: i64, resolved_in_version_id: Option<i64>, db: tauri::State<'_, Arc<Database>>) -> Result<(), String> {
    db.update_note_resolved_in(id, resolved_in_version_id).map_err(|e| e.to_string())
}

#[tauri::command]
fn review_export_csv(project_id: i64, db: tauri::State<'_, Arc<Database>>) -> Result<String, String> {
    let notes = db.get_notes(project_id).map_err(|e| e.to_string())?;
    let versions = db.get_versions(project_id).map_err(|e| e.to_string())?;
    
    let mut csv = String::new();
    csv.push_str("Timecode,Body,Status,Author,Created In Version,Resolved In Version,Created At\n");
    
    for note in notes {
        let created_label = note.created_in_version_id.and_then(|id| versions.iter().find(|v| v.id == id)).map(|v| v.label.as_str()).unwrap_or("");
        let resolved_label = note.resolved_in_version_id.and_then(|id| versions.iter().find(|v| v.id == id)).map(|v| v.label.as_str()).unwrap_or("");
        
        let timecode_str = if let Some(ms) = note.timecode_ms {
            let total_seconds = (ms / 1000.0) as u64;
            let minutes = total_seconds / 60;
            let seconds = total_seconds % 60;
            let milli = ((ms % 1000.0) / 10.0) as u64;
            format!("{:02}:{:02}.{:02}", minutes, seconds, milli)
        } else {
            String::new()
        };
        
        let body = note.body.replace("\"", "\"\"");
        csv.push_str(&format!("\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\n", 
            timecode_str, body, note.status, note.author_role, created_label, resolved_label, note.created_at));
    }
    
    Ok(csv)
}

#[tauri::command]
fn review_export_text(project_id: i64, db: tauri::State<'_, Arc<Database>>) -> Result<String, String> {
    let project = db.get_project(project_id).map_err(|e| e.to_string())?;
    let notes = db.get_notes(project_id).map_err(|e| e.to_string())?;
    
    let mut txt = String::new();
    txt.push_str(&format!("Project: {} ({})\n\n", project.title, project.created_at));
    
    let general_notes: Vec<_> = notes.iter().filter(|n| n.timecode_ms.is_none()).collect();
    if !general_notes.is_empty() {
        txt.push_str("General Notes:\n");
        for note in general_notes {
            txt.push_str(&format!("- {} ({}) [{}]\n", note.body, note.author_role, note.status));
        }
        txt.push_str("\n");
    }
    
    let timecoded_notes: Vec<_> = notes.iter().filter(|n| n.timecode_ms.is_some()).collect();
    if !timecoded_notes.is_empty() {
        txt.push_str("Timecoded Notes:\n");
        for note in timecoded_notes {
            let ms = note.timecode_ms.unwrap();
            let total_seconds = (ms / 1000.0) as u64;
            let minutes = total_seconds / 60;
            let seconds = total_seconds % 60;
            let milli = ((ms % 1000.0) / 10.0) as u64;
            txt.push_str(&format!("[{:02}:{:02}.{:02}] {} ({}) [{}]\n", minutes, seconds, milli, note.body, note.author_role, note.status));
        }
    }
    
    Ok(txt)
}

#[tauri::command]
fn review_export_markdown(project_id: i64, db: tauri::State<'_, Arc<Database>>) -> Result<String, String> {
    let project = db.get_project(project_id).map_err(|e| e.to_string())?;
    let notes = db.get_notes(project_id).map_err(|e| e.to_string())?;
    
    let mut md = String::new();
    md.push_str(&format!("# Project: {}\n\n", project.title));
    
    let general_notes: Vec<_> = notes.iter().filter(|n| n.timecode_ms.is_none()).collect();
    if !general_notes.is_empty() {
        md.push_str("## General Notes\n");
        for note in general_notes {
            md.push_str(&format!(" - {}\n", note.body));
        }
        md.push_str("\n");
    }
    
    let timecoded_notes: Vec<_> = notes.iter().filter(|n| n.timecode_ms.is_some()).collect();
    if !timecoded_notes.is_empty() {
        md.push_str("## Timecoded Notes\n");
        md.push_str("| Timecode | Note | Author | Status |\n");
        md.push_str("| --- | --- | --- | --- |\n");
        for note in timecoded_notes {
            let ms = note.timecode_ms.unwrap();
            let total_seconds = (ms / 1000.0) as u64;
            let minutes = total_seconds / 60;
            let seconds = total_seconds % 60;
            let milli = ((ms % 1000.0) / 10.0) as u64;
            
            md.push_str(&format!("| {:02}:{:02}.{:02} | {} | {} | {} |\n", minutes, seconds, milli, note.body.replace("\n", " "), note.author_role, note.status));
        }
    }
    
    Ok(md)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(MpvPlayer::new())
        .manage(AtmosPlayer::new())
        .manage(PlaybackPlayer::new())
        .setup(|app| {
            // Initialize Database
            let db_path = app.path().app_data_dir().unwrap_or_else(|_| std::path::PathBuf::from(".")).join("pik-review-library.db");
            if let Some(parent) = db_path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            let db = Database::new(&db_path).expect("Failed to initialize database");
            app.manage(Arc::new(db));

            // Initialize Player
            let state = app.state::<MpvPlayer>();
            state.init(app.handle().clone());

            let atmos_state = app.state::<AtmosPlayer>();
            atmos_state.init(app.handle().clone());

            let playback_state = app.state::<PlaybackPlayer>();
            playback_state.init(
                app.handle().clone(),
                state.inner().clone(),
                atmos_state.inner().clone(),
            );

            // Initialize Watcher (e.g. watch user music dir or app data dir)
            let watch_path = app.path().document_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
            let _ = scanner::watcher::watch_directory(watch_path);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            review_create_project,
            review_get_project,
            review_list_projects,
            review_update_project,
            review_delete_project,
            review_create_version,
            review_get_versions,
            review_delete_version,
            review_create_note,
            review_get_notes,
            review_update_note_status,
            review_update_note_body,
            review_delete_note,
            review_update_note_resolved_in,
            review_export_csv,
            review_export_text,
            review_export_markdown,
            player_load,
            player_toggle_play_pause,
            player_stop,
            player_seek,
            player_set_volume,
            atmos_load,
            atmos_play,
            atmos_pause,
            atmos_seek,
            atmos_set_volume,
            atmos_stop,
            playback_load,
            playback_toggle_play_pause,
            playback_play,
            playback_stop,
            playback_seek,
            playback_set_volume,
            playback_get_state,
            library_get_tracks,
            library_search_tracks,
            library_remove_track,
            library_import_folder,
            output_list_devices,
            output_set_device,
            output_is_headphone_connected,
            playback_set_eq_enabled,
            playback_set_eq_profile,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Pik Review");
}
