use net_reporter_api::api::network_bandwidth_per_endpoint::endpoint::EndpointDTO;

use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct EndpointResponse {
    id: String,
    #[serde(rename = "totalBytesReceived")]
    total_bytes_rec: u64,
    #[serde(rename = "totalBytesSent")]
    total_bytes_sent: u64,
}

impl EndpointResponse {
    pub fn new(id: String, total_bytes_rec: u64, total_bytes_sent: u64) -> Self {
        Self {
            id,
            total_bytes_rec,
            total_bytes_sent,
        }
    }
}

impl From<EndpointDTO> for EndpointResponse {
    fn from(endpoint_dto: EndpointDTO) -> Self {
        Self {
            id: endpoint_dto.get_id().to_owned(),
            total_bytes_rec: endpoint_dto.get_total_bytes_received() as u64,
            total_bytes_sent: endpoint_dto.get_total_bytes_sent() as u64,
        }
    }
}