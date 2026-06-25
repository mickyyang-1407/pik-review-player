import re

with open("src-tauri/src/database/mod.rs", "r") as f:
    content = f.read()

methods = """
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
"""

content = content.replace("    pub fn delete(&self, id: i64) -> Result<()> {\n        let conn = self.conn.lock().unwrap();\n        conn.execute(\"DELETE FROM tracks WHERE id = ?1\", params![id])?;\n        Ok(())\n    }", "    pub fn delete(&self, id: i64) -> Result<()> {\n        let conn = self.conn.lock().unwrap();\n        conn.execute(\"DELETE FROM tracks WHERE id = ?1\", params![id])?;\n        Ok(())\n    }\n" + methods)

with open("src-tauri/src/database/mod.rs", "w") as f:
    f.write(content)
