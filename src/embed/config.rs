use serde::{Deserialize, Serialize};
use pnet::datalink;
use std::{net::IpAddr, str::FromStr};
use url::Url;

const DEFAULT_INITIAL_ADVERTISE_PEER_URLS: &str = "http://localhost:2380";
const DEFAULT_ADVERTISE_CLIENT_URLS: &str = "http://localhost:2379";
const DEFAULT_NAME : &str = "default";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbedConfig {
    pub name : String,
    pub dir : String,
    pub wal_dir : String,
    pub advertise_peer_urls : Vec<String>,
    pub advertise_client_urls : String,
    pub initial_cluster : String,
    pub inital_cluster_token : String,
}

impl EmbedConfig {
    pub fn new_config() -> Self {
        let apul = Url::parse(DEFAULT_INITIAL_ADVERTISE_PEER_URLS).unwrap();
        let mut cfg = Self {
            name : String::from(DEFAULT_NAME),
            dir : String::from(""),
            wal_dir : String::from(""),
            advertise_peer_urls : vec![String::from(apul.as_str())],
            advertise_client_urls : String::from(""),
            initial_cluster : String::from(""),
            inital_cluster_token : String::from(""),
        };
        cfg.initial_cluster = cfg.initial_cluster_from_name();
        return cfg;
    }
    pub fn update_default_cluster_from_name(&mut self, default_initial_cluster : &String) -> String {
        let default_host_name = get_default_host().unwrap_or(String::from(""));

        if default_host_name == "" {
            if self.name != "default" && self.initial_cluster == *default_initial_cluster {
                self.initial_cluster = self.initial_cluster_from_name();
            }
            return String::from("");
        }

        return default_host_name;
    }
    pub fn initial_cluster_from_name(&self) -> String {
        if self.advertise_peer_urls.is_empty() {
            return String::from("");
        }
        
        if self.name.is_empty() {
            return String::from("default");
        }
        
        self.advertise_peer_urls.iter()
            .map(|url| format!("{}={}", self.name, url))
            .collect::<Vec<_>>()
            .join(",")
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