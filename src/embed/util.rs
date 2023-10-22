
use crate::embed::config::EmbedConfig;
use crate::storage::wal::util;

pub fn is_member_initialized(cfg: &EmbedConfig) -> bool {
    let mut waldir = cfg.wal_dir.clone();
    if waldir == "" {
        waldir = cfg.dir.clone() + "/member/wal";
    }
    return util::has_wal_files(waldir);
}