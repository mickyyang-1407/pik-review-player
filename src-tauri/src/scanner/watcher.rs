use notify::{Watcher, RecursiveMode};
use std::path::Path;
use std::sync::mpsc::channel;

pub fn watch_directory<P: AsRef<Path>>(path: P) -> Result<(), String> {
    let (tx, _rx) = channel();

    // RecommendedWatcher is standard
    let mut watcher = notify::recommended_watcher(tx)
        .map_err(|e| e.to_string())?;

    watcher.watch(path.as_ref(), RecursiveMode::Recursive)
        .map_err(|e| e.to_string())?;

    Box::leak(Box::new(watcher));
    Ok(())
}
