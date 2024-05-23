use net_reporter_api::api::network_packet::network_packets::NetworkPacketsDTO;
use serde::Deserialize;
use serde::Serialize;

use super::packet::NetworkPacket;

#[derive(Deserialize, Serialize)]
pub struct NetworkPackets {
    pub packets: Vec<NetworkPacket> 
}

impl From<NetworkPacketsDTO> for NetworkPackets {
    fn from(value: NetworkPacketsDTO) -> Self {
        Self {
            packets: value.get_network_packets().iter().map(|packet| packet.into()).collect()
        }
    }
}
