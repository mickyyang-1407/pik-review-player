pub mod watcher;

// ADM WAV sidecar schema (place <stem>.metadata.json or metadata.json next to the .wav file):
// {
//   "title": "Track Title",
//   "artist": "Artist Name",
//   "album": "Album Name",
//   "year": 2024,
//   "track": 1,
//   "cover": "cover.jpg",       // relative path to artwork
//   "atmos": true,              // always true for ADM WAV
//   "bed_channels": 7,          // 7.1.2 bed
//   "object_count": 118,        // max simultaneous objects
//   "binaural_render": "mid"    // none | near | far | mid
// }

use lofty::probe::Probe;
use lofty::tag::Accessor;
use lofty::file::{TaggedFileExt, AudioFile};
use std::path::Path;
use std::hash::{Hash, Hasher};
use std::io::Write;

pub struct Metadata {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub duration: f64,
    pub format: String,
    pub cover_path: Option<String>,
}

#[derive(serde::Deserialize, Default)]
#[serde(default)]
struct SidecarMeta {
    title: Option<String>,
    artist: Option<String>,
    album: Option<String>,
    cover: Option<String>,
}

fn read_sidecar(audio_path: &Path) -> Option<SidecarMeta> {
    let dir = audio_path.parent()?;
    let stem = audio_path.file_stem()?.to_str()?;

    // Try <stem>.metadata.json first, then generic metadata.json
    let by_stem = dir.join(format!("{}.metadata.json", stem));
    let generic = dir.join("metadata.json");

    let path = if by_stem.exists() {
        by_stem
    } else if generic.exists() {
        generic
    } else {
        return None;
    };

    let content = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&content).ok()
}

fn find_cover(audio_path: &Path) -> Option<String> {
    let dir = audio_path.parent()?;
    let candidates = ["cover.jpg", "cover.png", "folder.jpg", "folder.png", "artwork.jpg", "artwork.png"];
    for name in candidates {
        let p = dir.join(name);
        if p.exists() {
            return Some(p.to_string_lossy().to_string());
        }
    }
    None
}

pub fn scan_file<P: AsRef<Path>>(path: P) -> Result<Metadata, String> {
    let path = path.as_ref();
    let format = path.extension().unwrap_or_default().to_string_lossy().to_lowercase();

    let tagged_file = Probe::open(path)
        .map_err(|e| e.to_string())?
        .read()
        .map_err(|e| e.to_string())?;

    let properties = tagged_file.properties();
    let duration = properties.duration().as_secs_f64();

    let tag = tagged_file.primary_tag().or_else(|| tagged_file.first_tag());

    let (mut title, mut artist, mut album) = if let Some(t) = tag {
        (
            t.title().map(|s| s.into_owned()),
            t.artist().map(|s| s.into_owned()),
            t.album().map(|s| s.into_owned()),
        )
    } else {
        (None, None, None)
    };

    // Cover: look for cover.jpg in same dir
    let mut cover_path = find_cover(path);

    // If no local cover.jpg, try extracting embedded picture
    if cover_path.is_none() {
        if let Some(t) = &tag {
            if let Some(pic) = t.pictures().first() {
                let cache_dir = dirs::cache_dir()
                    .unwrap_or_else(|| std::path::PathBuf::from("/tmp"))
                    .join("pik-review/covers");
                let _ = std::fs::create_dir_all(&cache_dir);
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                path.hash(&mut hasher);
                let ext = match pic.mime_type() {
                    Some(mime) if mime.as_str().contains("png") => "png",
                    _ => "jpg",
                };
                let cache_file = cache_dir.join(format!("{:x}.{}", hasher.finish(), ext));
                if !cache_file.exists() {
                    if let Ok(mut f) = std::fs::File::create(&cache_file) {
                        let _ = f.write_all(pic.data());
                    }
                }
                if cache_file.exists() {
                    cover_path = Some(cache_file.to_string_lossy().to_string());
                }
            }
        }
    }

    if format == "wav" {
        if let Some(side) = read_sidecar(path) {
            if side.title.is_some() { title = side.title; }
            if side.artist.is_some() { artist = side.artist; }
            if side.album.is_some() { album = side.album; }
            // Sidecar cover is relative to the audio file's directory
            if let (Some(rel), Some(dir)) = (side.cover, path.parent()) {
                let abs = dir.join(&rel);
                if abs.exists() {
                    cover_path = Some(abs.to_string_lossy().to_string());
                }
            }
        }
    }

    Ok(Metadata {
        title,
        artist,
        album,
        duration,
        format: format.to_string(),
        cover_path,
    })
}
