// Standard library imports
use std::fs;
use std::path::Path;

// Relative imports
use super::config;
use super::super::embed::config::EmbedConfig;

// External crate imports
use slog::{Drain, Logger, o};

enum DirType {
    DirEmpty,
    DirMember,
    DirProxy,
}

pub fn start_retcd_or_proxy(args: Vec<String>) {

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let log = slog::Logger::root(drain, o!());

    let mut configuration = config::new_config();
    let default_initial_cluster = &configuration.ec.initial_cluster.clone();

    slog::info!(log, "Running: {:?}", args);

    let default_host = configuration.ec.update_default_cluster_from_name(default_initial_cluster);
    if !default_host.is_empty() {
        slog::info!(log, "Using default host: {}", default_host);
    }

    if configuration.ec.dir == "" {
        configuration.ec.dir = format!("{}.retcd", configuration.ec.name);
        slog::warn!(log, "No data-dir provided, using default data-dir: {}", configuration.ec.dir);
    }

    let which = identify_data_dir_or_die(&configuration.ec.dir, &log);
    match which {
        DirType::DirEmpty => {
            slog::info!(log, "Initilize and start retcd server");
            slog::info!(log, "data-dir: {}", configuration.ec.dir);
            slog::info!(log, "dir-type: {}", "DirEmpty");
            start_retcd(&configuration.ec);
        },
        _ => {
            slog::info!(log, "server has already been initialized");
            slog::info!(log, "data-dir: {}", configuration.ec.dir);
            slog::info!(log, "dir-type: {}", "DirMember");
            start_retcd(&configuration.ec)
        }
    }
}

fn start_retcd(cfg : &EmbedConfig) {
    let _ = crate::embed::retcd::start_retcd(cfg);
}

fn identify_data_dir_or_die(dir: &str, log : &Logger) -> DirType {
    let entries = match fs::read_dir(Path::new(dir)) {
        Ok(entries) => entries,
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                return DirType::DirEmpty;
            }
            slog::error!(log, "failed to list data directory: {}", e);
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
                    _ => slog::warn!(log, "ignoring unknown file in data directory: {}", name),
                }
            }
            Err(e) => {
                slog::error!(log, "failed to list data directory: {}", e);
                panic!("failed to list data directory");
            }
        }
    }

    if m && p {
        slog::error!(log, "invalid datadir: both member and proxy directories exist");
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
