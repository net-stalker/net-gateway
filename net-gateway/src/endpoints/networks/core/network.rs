use net_reporter_api::api::network::network::NetworkDTO;
use serde::{Deserialize, Serialize};

use super::network_packet::NetworkPacket;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Network {
    pub id: Option<String>,
    pub name: String,
    pub color: String,
    pub packets: Option<Vec<NetworkPacket>>,
}


impl From<NetworkDTO> for Network {
    fn from(value: NetworkDTO) -> Self {
        Self {
            id: Some(value.get_id().to_string()),
            name: value.get_name().to_string(),
            color: value.get_color().to_string(),
            packets: None,
        }
    }
}
