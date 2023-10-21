use serde::{Deserialize, Serialize};
use pnet::datalink;
use std::{net::IpAddr, io::Error};

static mut DEFAULT_HOSTNAME: Option<String> = None;

fn initialize() {
    unsafe {
        if DEFAULT_HOSTNAME.is_none() {
            DEFAULT_HOSTNAME = get_default_host();
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub name : String,
    pub dir : String,
    pub wal_dir : String,
    pub snapshot_count : u64,
    pub snapshot_catch_up_entries : u64,
    pub max_snap_files : u64,
    pub max_wal_files : u64,
    pub tick_ms : u64,
    pub election_tick : u64,
    pub initial_election_tick_advance : bool,
    pub backend_commit_interval : u64,
    pub backend_batch_limit : u64,
    pub backend_freelist_type : String,
    pub quota_backend_bytes : u64,
    pub max_txn_ops : u64,
    pub max_request_bytes : u64,
    pub max_concurrent_streams : u64,
    pub client_auto_tls : bool,
    pub peeer_auto_tls : bool,
    pub self_signed_cert_validity : u64,
    pub cipher_suites : Vec<String>,
    pub tls_min_version : String,
    pub tls_max_version : String,
    pub cluster_state : String,
    pub dns_cluster : String,
    pub dns_cluster_service_name : String,
    pub dproxy : String,
    pub durl : String,
    pub initial_cluster : String,
    pub inital_cluster_token : String,
    pub strict_reconfig_check : bool,
    pub enable : bool,
    pub auto_compaction_mode : String,
    pub pre_vote : bool,
    pub auth_token : String,
    pub bcrypt_cost : u32,
    pub auto_token_ttl : u64,
}


impl Config {
    pub fn update_default_cluster_from_name(default_initial_cluster : String) -> Result<String, Error> {
        initialize();
        if unsafe { DEFAULT_HOSTNAME.is_none() } {
            return Err(Error::new(std::io::ErrorKind::Other, "Unable to get default hostname"));
        } else {
            let default_hostname = unsafe { DEFAULT_HOSTNAME.as_ref().unwrap() };
            let default_cluster = format!("{}=http://{}", default_hostname, default_hostname);
            let default_initial_cluster = if default_initial_cluster.is_empty() {
                default_cluster
            } else {
                default_initial_cluster
            };
            return Ok(default_initial_cluster);
        }

    }

}

pub fn new_config() -> Config {
    Config {
        name : String::from(""),
        dir : String::from(""),
        wal_dir : String::from(""),
        snapshot_count : 0,
        snapshot_catch_up_entries : 0,
        max_snap_files : 0,
        max_wal_files : 0,
        tick_ms : 0,
        election_tick : 0,
        initial_election_tick_advance : false,
        backend_commit_interval : 0,
        backend_batch_limit : 0,
        backend_freelist_type : String::from(""),
        quota_backend_bytes : 0,
        max_txn_ops : 0,
        max_request_bytes : 0,
        max_concurrent_streams : 0,
        client_auto_tls : false,
        peeer_auto_tls : false,
        self_signed_cert_validity : 0,
        cipher_suites : Vec::new(),
        tls_min_version : String::from(""),
        tls_max_version : String::from(""),
        cluster_state : String::from(""),
        dns_cluster : String::from(""),
        dns_cluster_service_name : String::from(""),
        dproxy : String::from(""),
        durl : String::from(""),
        initial_cluster : String::from(""),
        inital_cluster_token : String::from(""),
        strict_reconfig_check : false,
        enable : false,
        auto_compaction_mode : String::from(""),
        pre_vote : false,
        auth_token : String::from(""),
        bcrypt_cost : 0,
        auto_token_ttl : 0,
    }
}

// Get the default host for the current machine.
fn get_default_host() -> Option<String> {
    // Get all the network interfaces for the current machine.
    let all_interfaces = datalink::interfaces();

    // Iterate over the interfaces to find the first IPv4 address.
    for interface in all_interfaces {
        for ip_network in interface.ips {
            if let IpAddr::V4(ipv4_addr) = ip_network.ip() {
                return Some(ipv4_addr.to_string());
            }
        }
    }

    // If no IPv4 address is found, return None.
    None
}