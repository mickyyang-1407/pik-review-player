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

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let conn = Connection::open(db_path)?;
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
}
