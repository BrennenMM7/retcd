use std::fs;
use std::path::Path;

pub fn has_wal_files<P: AsRef<Path>>(dir: P) -> bool {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("wal") {
                    return true;
                }
            }
        }
    }
    false
}