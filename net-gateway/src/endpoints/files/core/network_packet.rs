use chrono::DateTime;
use chrono::TimeZone;
use chrono::Utc;
use net_inserter_api::api::network_packet::network_packet::NetworkPacketDTO;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NetworkPacket {
    pub id: i64,
    pub frametime: DateTime<Utc>,
    pub src: String,
    pub dst: String,
    pub protocols: Vec<String>,
    pub json_data: serde_json::Value,
}

impl From<NetworkPacketDTO> for NetworkPacket {
    fn from(value: NetworkPacketDTO) -> Self {
        Self {
            id: value.get_id(),
            frametime: Utc.timestamp_opt(value.get_frame_time(), 0).unwrap(),
            src: value.get_src().to_string(),
            dst: value.get_dst().to_string(),
            protocols: value.get_protocols().to_vec(),
            json_data: serde_json::from_slice(value.get_json()).unwrap(),
        }
    }
}
