use rusqlite::{params, Connection, Result};
use serde::Serialize;
use std::path::Path;
use std::sync::Mutex;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub id: i64,
    pub path: String,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub duration: f64,
    pub format: String,
    pub cover_path: Option<String>,
    pub added_at: String,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: i64,
    pub title: String,
    pub client: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub cue: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub id: i64,
    pub project_id: i64,
    pub label: String,
    pub file_path: String,
    pub duration: Option<f64>,
    pub imported_at: String,
    pub checksum: Option<String>,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewNote {
    pub id: i64,
    pub project_id: i64,
    pub created_in_version_id: Option<i64>,
    pub resolved_in_version_id: Option<i64>,
    pub timecode_ms: Option<f64>,
    pub body: String,
    pub author_role: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tracks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                path TEXT UNIQUE NOT NULL,
                title TEXT,
                artist TEXT,
                album TEXT,
                duration REAL NOT NULL,
                format TEXT NOT NULL,
                cover_path TEXT,
                added_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;
        // Safe migration: adds cover_path to existing DBs; no-op if column already exists
        let _ = conn.execute("ALTER TABLE tracks ADD COLUMN cover_path TEXT", []);

        conn.execute(
            "CREATE TABLE IF NOT EXISTS projects (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                client TEXT,
                artist TEXT,
                album TEXT,
                cue TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS versions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                project_id INTEGER NOT NULL,
                label TEXT NOT NULL,
                file_path TEXT NOT NULL,
                duration REAL,
                imported_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                checksum TEXT,
                FOREIGN KEY (project_id) REFERENCES projects(id)
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS notes (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                project_id INTEGER NOT NULL,
                created_in_version_id INTEGER,
                resolved_in_version_id INTEGER,
                timecode_ms REAL,
                body TEXT NOT NULL,
                author_role TEXT NOT NULL DEFAULT 'producer',
                status TEXT NOT NULL DEFAULT 'open',
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (project_id) REFERENCES projects(id),
                FOREIGN KEY (created_in_version_id) REFERENCES versions(id),
                FOREIGN KEY (resolved_in_version_id) REFERENCES versions(id)
            )",
            [],
        )?;

        Ok(Self { conn: Mutex::new(conn) })
    }

    pub fn insert(&self, track: &Track) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR IGNORE INTO tracks (path, title, artist, album, duration, format, cover_path)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                track.path,
                track.title,
                track.artist,
                track.album,
                track.duration,
                track.format,
                track.cover_path,
            ],
        )?;
        Ok(())
    }

    pub fn query(&self) -> Result<Vec<Track>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, path, title, artist, album, duration, format, cover_path, added_at
             FROM tracks ORDER BY added_at DESC"
        )?;
        let tracks = stmt.query_map([], |row| {
            Ok(Track {
                id: row.get(0)?,
                path: row.get(1)?,
                title: row.get(2)?,
                artist: row.get(3)?,
                album: row.get(4)?,
                duration: row.get(5)?,
                format: row.get(6)?,
                cover_path: row.get(7)?,
                added_at: row.get(8)?,
            })
        })?;

        let mut result = Vec::new();
        for t in tracks {
            result.push(t?);
        }
        Ok(result)
    }

    pub fn search(&self, query: &str) -> Result<Vec<Track>> {
        let like_q = format!("%{}%", query);
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, path, title, artist, album, duration, format, cover_path, added_at
             FROM tracks
             WHERE title LIKE ?1 OR artist LIKE ?1 OR album LIKE ?1
             ORDER BY added_at DESC"
        )?;
        let tracks = stmt.query_map([&like_q], |row| {
            Ok(Track {
                id: row.get(0)?,
                path: row.get(1)?,
                title: row.get(2)?,
                artist: row.get(3)?,
                album: row.get(4)?,
                duration: row.get(5)?,
                format: row.get(6)?,
                cover_path: row.get(7)?,
                added_at: row.get(8)?,
            })
        })?;

        let mut result = Vec::new();
        for t in tracks {
            result.push(t?);
        }
        Ok(result)
    }

    pub fn delete(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM tracks WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn create_project(&self, title: &str, client: Option<&str>, artist: Option<&str>, album: Option<&str>, cue: Option<&str>) -> Result<Project> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO projects (title, client, artist, album, cue) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![title, client, artist, album, cue],
        )?;
        let id = conn.last_insert_rowid();
        let mut stmt = conn.prepare("SELECT id, title, client, artist, album, cue, created_at, updated_at FROM projects WHERE id = ?1")?;
        let p = stmt.query_row(params![id], |row| {
            Ok(Project {
                id: row.get(0)?,
                title: row.get(1)?,
                client: row.get(2)?,
                artist: row.get(3)?,
                album: row.get(4)?,
                cue: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        })?;
        Ok(p)
    }

    pub fn get_project(&self, id: i64) -> Result<Project> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, title, client, artist, album, cue, created_at, updated_at FROM projects WHERE id = ?1")?;
        let p = stmt.query_row(params![id], |row| {
            Ok(Project {
                id: row.get(0)?,
                title: row.get(1)?,
                client: row.get(2)?,
                artist: row.get(3)?,
                album: row.get(4)?,
                cue: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        })?;
        Ok(p)
    }

    pub fn list_projects(&self) -> Result<Vec<Project>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, title, client, artist, album, cue, created_at, updated_at FROM projects ORDER BY updated_at DESC")?;
        let iter = stmt.query_map([], |row| {
            Ok(Project {
                id: row.get(0)?,
                title: row.get(1)?,
                client: row.get(2)?,
                artist: row.get(3)?,
                album: row.get(4)?,
                cue: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        })?;
        let mut res = Vec::new();
        for i in iter {
            res.push(i?);
        }
        Ok(res)
    }

    pub fn update_project(&self, id: i64, title: Option<&str>, client: Option<&str>, artist: Option<&str>, album: Option<&str>, cue: Option<&str>) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE projects SET 
                title = COALESCE(?2, title),
                client = COALESCE(?3, client),
                artist = COALESCE(?4, artist),
                album = COALESCE(?5, album),
                cue = COALESCE(?6, cue),
                updated_at = CURRENT_TIMESTAMP
             WHERE id = ?1",
            params![id, title, client, artist, album, cue],
        )?;
        Ok(())
    }

    pub fn delete_project(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM notes WHERE project_id = ?1", params![id])?;
        conn.execute("DELETE FROM projects WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn create_version(&self, project_id: i64, label: &str, file_path: &str, duration: Option<f64>, checksum: Option<&str>) -> Result<Version> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO versions (project_id, label, file_path, duration, checksum) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![project_id, label, file_path, duration, checksum],
        )?;
        let id = conn.last_insert_rowid();
        let mut stmt = conn.prepare("SELECT id, project_id, label, file_path, duration, imported_at, checksum FROM versions WHERE id = ?1")?;
        let v = stmt.query_row(params![id], |row| {
            Ok(Version {
                id: row.get(0)?,
                project_id: row.get(1)?,
                label: row.get(2)?,
                file_path: row.get(3)?,
                duration: row.get(4)?,
                imported_at: row.get(5)?,
                checksum: row.get(6)?,
            })
        })?;
        Ok(v)
    }

    pub fn get_versions(&self, project_id: i64) -> Result<Vec<Version>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, project_id, label, file_path, duration, imported_at, checksum FROM versions WHERE project_id = ?1 ORDER BY imported_at ASC")?;
        let iter = stmt.query_map(params![project_id], |row| {
            Ok(Version {
                id: row.get(0)?,
                project_id: row.get(1)?,
                label: row.get(2)?,
                file_path: row.get(3)?,
                duration: row.get(4)?,
                imported_at: row.get(5)?,
                checksum: row.get(6)?,
            })
        })?;
        let mut res = Vec::new();
        for i in iter {
            res.push(i?);
        }
        Ok(res)
    }

    pub fn delete_version(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("UPDATE notes SET created_in_version_id = NULL WHERE created_in_version_id = ?1", params![id])?;
        conn.execute("UPDATE notes SET resolved_in_version_id = NULL WHERE resolved_in_version_id = ?1", params![id])?;
        conn.execute("DELETE FROM versions WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn create_note(&self, project_id: i64, created_in_version_id: Option<i64>, timecode_ms: Option<f64>, body: &str, author_role: Option<&str>, status: Option<&str>) -> Result<ReviewNote> {
        let conn = self.conn.lock().unwrap();
        let role = author_role.unwrap_or("producer");
        let st = status.unwrap_or("open");
        conn.execute(
            "INSERT INTO notes (project_id, created_in_version_id, timecode_ms, body, author_role, status) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![project_id, created_in_version_id, timecode_ms, body, role, st],
        )?;
        let id = conn.last_insert_rowid();
        let mut stmt = conn.prepare("SELECT id, project_id, created_in_version_id, resolved_in_version_id, timecode_ms, body, author_role, status, created_at, updated_at FROM notes WHERE id = ?1")?;
        let n = stmt.query_row(params![id], |row| {
            Ok(ReviewNote {
                id: row.get(0)?,
                project_id: row.get(1)?,
                created_in_version_id: row.get(2)?,
                resolved_in_version_id: row.get(3)?,
                timecode_ms: row.get(4)?,
                body: row.get(5)?,
                author_role: row.get(6)?,
                status: row.get(7)?,
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
            })
        })?;
        Ok(n)
    }

    pub fn get_notes(&self, project_id: i64) -> Result<Vec<ReviewNote>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, project_id, created_in_version_id, resolved_in_version_id, timecode_ms, body, author_role, status, created_at, updated_at FROM notes WHERE project_id = ?1 ORDER BY timecode_ms ASC, created_at ASC")?;
        let iter = stmt.query_map(params![project_id], |row| {
            Ok(ReviewNote {
                id: row.get(0)?,
                project_id: row.get(1)?,
                created_in_version_id: row.get(2)?,
                resolved_in_version_id: row.get(3)?,
                timecode_ms: row.get(4)?,
                body: row.get(5)?,
                author_role: row.get(6)?,
                status: row.get(7)?,
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
            })
        })?;
        let mut res = Vec::new();
        for i in iter {
            res.push(i?);
        }
        Ok(res)
    }

    pub fn update_note_status(&self, id: i64, status: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE notes SET status = ?2, updated_at = CURRENT_TIMESTAMP WHERE id = ?1",
            params![id, status],
        )?;
        Ok(())
    }

    pub fn update_note_resolved_in(&self, id: i64, resolved_in_version_id: Option<i64>) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE notes SET resolved_in_version_id = ?2, updated_at = CURRENT_TIMESTAMP WHERE id = ?1",
            params![id, resolved_in_version_id],
        )?;
        Ok(())
    }

    pub fn update_note_body(&self, id: i64, body: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE notes SET body = ?2, updated_at = CURRENT_TIMESTAMP WHERE id = ?1",
            params![id, body],
        )?;
        Ok(())
    }

    pub fn delete_note(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM notes WHERE id = ?1", params![id])?;
        Ok(())
    }

}
