use crate::embed::config::EmbedConfig;
use crate::retcdserver::server::RetcdServer;
use super::util::is_member_initialized;


pub struct Retcd {
    pub server: RetcdServer,
    pub cfg: EmbedConfig,
}

pub fn start_retcd(in_cfg: &EmbedConfig) -> Retcd {

    let serving = false;

    let mut members_initialized = true;
    if !is_member_initialized(in_cfg) {
        members_initialized = false;
    }

    



    return Retcd {
        server: RetcdServer{},
        cfg: in_cfg.clone(),
    };
}