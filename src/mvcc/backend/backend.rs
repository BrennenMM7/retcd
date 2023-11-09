use std::time::Duration;

const DEFAULT_BATCH_LIMIT: i32 = 10000;
const DEFAULT_BATCH_INTERVAL: Duration = Duration::from_millis(100);
const INITIAL_MMAP_SIZE: u64 = 10 * 1024 * 1024 * 1024;




trait Backend {
    fn size(&self) -> i64; 
}

pub struct BackendStruct {
    pub size: i64,
    pub size_in_use: i64,
    pub commits: i64,
    pub open_read_tx_n: i64,
    pub mlock: bool,
    pub batch_limit: i32
}

impl Backend for BackendStruct {
    fn size(&self) -> i64 {
        self.size()
    }
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





