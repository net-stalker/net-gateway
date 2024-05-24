use net_reporter_api::api::network::networks::NetworksDTO;
use serde::Deserialize;
use serde::Serialize;

use super::network::Network;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Networks {
    pub networks: Vec<Network>,
}

impl From<NetworksDTO> for Networks {
    fn from(value: NetworksDTO) -> Self {
        Self {
            networks: value.get_networks().iter().map(|network| network.into()).collect(),
        }
    }
}
