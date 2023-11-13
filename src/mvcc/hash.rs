extern crate crc32fast;
use crc32fast::Hasher;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::revision::Revision;
use super::buckets::bucket;

const HASH_STORAGE_MAX_SIZE: i8 = 10;

// Define the `KvHasher` struct
struct KvHasher {
    hash: Hasher,
    compact_revision: i64,
    revision: i64,
    keep: HashMap<Revision, ()>,
}

impl KvHasher {
    fn new_kv_hasher(compact_revision: i64, revision: i64, keep: HashMap<Revision, ()> ) -> Self {
        let mut hasher = Hasher::new_with_initial(0u32);
        let bucket_name_bytes = bucket::KEY.name();
        hasher.update(bucket_name_bytes);

        KvHasher { 
            hash: hasher,
            compact_revision,
            revision,
            keep
        }
    }

}