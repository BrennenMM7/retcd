use std::fs;
use std::path::Path;
use log::{error, warn};

enum DirType {
    DirEmpty,
    DirMember,
    DirProxy,
}


fn start_retcd_or_proxy() {}

fn start_retcd() {}

fn start_proxy() {}

fn identify_data_dir_or_die(dir: &str) -> DirType {
    let entries = match fs::read_dir(Path::new(dir)) {
        Ok(entries) => entries,
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                return DirType::DirEmpty;
            }
            error!("failed to list data directory: dir={}, error={}", dir, e);
            panic!("failed to list data directory");
        }
    };

    let mut m = false;
    let mut p = false;
    for entry in entries {
        match entry {
            Ok(entry) => {
                let name = entry.file_name().into_string().unwrap_or_default();
                match dir_type(&name) {
                    DirType::DirMember => m = true,
                    DirType::DirProxy => p = true,
                    _ => warn!(
                        "found invalid file under data directory: filename={}, data-dir={}",
                        name, dir
                    ),
                }
            }
            Err(e) => {
                warn!("Error reading directory entry: {}", e);
            }
        }
    }

    if m && p {
        error!("invalid datadir; both member and proxy directories exist");
        panic!("invalid datadir");
    }
    if m {
        return DirType::DirMember;
    }
    if p {
        return DirType::DirProxy;
    }
    DirType::DirEmpty
}

fn dir_type(name: &str) -> DirType {
    match name {
        "member" => DirType::DirMember, 
        "proxy" => DirType::DirProxy,
        _ => DirType::DirEmpty,
    }
}
