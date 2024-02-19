use net_reporter_api::api::network_bandwidth::network_bandwidth::NetworkBandwidthDTO;

use serde::Deserialize;
use serde::Serialize;

use crate::core::chart_management::chart_request_manager::ChartResponse;

use super::network_bandwidth_bucket::NetworkBandwidthBucketResponse;


#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct NetworkBandwidthResponse {
    #[serde(rename = "networkBandwidthBuckets")]
    pub buckets: Vec<NetworkBandwidthBucketResponse>,
}
impl ChartResponse for NetworkBandwidthResponse {
    fn get_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}

impl NetworkBandwidthResponse {
    pub fn new(buckets: Vec<NetworkBandwidthBucketResponse>) -> Self {
        Self { buckets }
    }
}

impl From<NetworkBandwidthDTO> for NetworkBandwidthResponse {
    fn from(data: NetworkBandwidthDTO) -> Self {
        let buckets = data
            .get_bandwidth_buckets()
            .iter()
            .map(|bucket| NetworkBandwidthBucketResponse::from(bucket.clone()))
            .collect();
        Self { buckets }
    }
}
