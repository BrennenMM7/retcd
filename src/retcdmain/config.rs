use serde::{Deserialize, Serialize};

const PROXY_FLAG_OFF: &str = "off";
const PROXY_FLAG_READONLY: &str = "readonly";
const PROXY_FLAG_ON: &str = "on";

const FALLBACK_FLAG_EXIT: &str = "exit";
const FALLBACK_FLAG_PROXY: &str = "proxy";

const IGNORED: &[&str] = &[
    "cluster-active-size",
    "cluster-remove-delay",
    "cluster-sync-interval",
    "config",
    "force",
    "max-result-buffer",
    "max-retry-attempts",
    "peer-heartbeat-interval",
    "peer-election-timeout",
    "retry-interval",
    "snapshot",
    "v",
    "vv",
    // for coverage testing
    "test.coverprofile",
    "test.outputdir",
];

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    cp : ConfigProxy,
    cf : ConfigFlags,
    config_file : String,
    print_version : bool,
    ignored : Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigProxy {
    #[serde(rename = "proxy-failure-wait")]
    proxy_failure_wait_ms: u32,
    #[serde(rename = "proxy-refresh-interval")]
    proxy_refresh_interval_ms: u32,
    #[serde(rename = "proxy-dial-timeout")]
    proxy_dial_timeout_ms: u32,
    #[serde(rename = "proxy-write-timeout")]
    proxy_write_timeout_ms: u32,
    #[serde(rename = "proxy-read-timeout")]
    proxy_read_timeout_ms: u32,
    fallback: String,
    proxy: String,
    #[serde(rename = "proxy")]
    proxy_json: String,
    #[serde(rename = "discovery-fallback")]
    fallback_json: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFlags {
}

pub fn new_config() -> Config {
    Config {
        cp: ConfigProxy {
            proxy_failure_wait_ms: 0,
            proxy_refresh_interval_ms: 0,
            proxy_dial_timeout_ms: 0,
            proxy_write_timeout_ms: 0,
            proxy_read_timeout_ms: 0,
            fallback: String::from(""),
            proxy: String::from(""),
            proxy_json: String::from(""),
            fallback_json: String::from(""),
        },
        cf: ConfigFlags {},
        config_file: String::from(""),
        print_version: false,
        ignored: Vec::new(),
    }
}