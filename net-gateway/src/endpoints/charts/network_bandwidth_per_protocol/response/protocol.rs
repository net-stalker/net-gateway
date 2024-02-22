use net_reporter_api::api::network_bandwidth_per_protocol::protocol::ProtocolDTO;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ProtocolResponse {
    name: String,
    #[serde(rename = "totalBytes")]
    total_bytes: i64,
}

impl ProtocolResponse {
    pub fn new(name: String, total_bytes: i64) -> Self {
        Self {
            name,
            total_bytes,
        }
    }
}

impl From<ProtocolDTO> for ProtocolResponse {
    fn from(value: ProtocolDTO) -> Self {
        Self {
            name: value.get_name().to_string(),
            total_bytes: value.get_total_bytes(),
        }
    }
}