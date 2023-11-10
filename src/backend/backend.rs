use std::time::Duration;

use jammdb::{DB, OpenOptions};

const DEFAULT_BATCH_LIMIT: i32 = 10000;
const DEFAULT_BATCH_INTERVAL: Duration = Duration::from_millis(100);
const INITIAL_MMAP_SIZE: u64 = 10 * 1024 * 1024 * 1024;

pub struct Backend {
    pub size: i64,
    pub db: DB,
    pub size_in_use: i64,
    pub commits: i64,
    pub open_read_tx_n: i64,
    pub mlock: bool,
    pub batch_limit: i32
}

pub struct BackendConfig {
    pub path: String,
    pub batch_interval: Duration,
    pub batch_limit: i32,
    pub mmap_size: u64,
    pub unsfae_no_fysnc: bool,
    pub mlock: bool,
}


impl BackendConfig {
    pub fn default_backend_config(&self, path: String) -> Self {
        BackendConfig {
            batch_interval: DEFAULT_BATCH_INTERVAL,
            batch_limit: DEFAULT_BATCH_LIMIT,
            mmap_size: INITIAL_MMAP_SIZE,
            path: path,
            mlock: false,
            unsfae_no_fysnc: false
        }
    }
}


impl Backend {
    pub fn new(bcfg: BackendConfig) -> Self {

        let db = DB::open(&bcfg.path).unwrap_or_else(|e| {
            panic!("Error opening database: {}", e);
        });  

        Backend {
            size: 0,
            db: db,
            size_in_use: 0,
            commits: 0,
            open_read_tx_n: 0,
            mlock: false,
            batch_limit: DEFAULT_BATCH_LIMIT
        }
    }
}




