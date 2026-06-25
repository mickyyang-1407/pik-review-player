import re

with open("src-tauri/src/lib.rs", "r") as f:
    content = f.read()

commands = """
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
"""

content = content.replace("#[cfg_attr(mobile, tauri::mobile_entry_point)]\npub fn run() {", commands + "\n#[cfg_attr(mobile, tauri::mobile_entry_point)]\npub fn run() {")

handler_str = "tauri::generate_handler!["
new_handler = handler_str + """
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
            review_delete_note,"""
content = content.replace(handler_str, new_handler)

with open("src-tauri/src/lib.rs", "w") as f:
    f.write(content)
