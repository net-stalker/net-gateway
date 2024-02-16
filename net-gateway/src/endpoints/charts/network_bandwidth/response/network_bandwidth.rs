use net_reporter_api::api::network_bandwidth::network_bandwidth::NetworkBandwidthDTO;

use serde::Deserialize;
use serde::Serialize;

use crate::core::chart_requester::ChartResponse;

use super::network_bandwidth_bucket::NetworkBandwidthBucketResponse;


#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct NetworkBandwidthResponse {
    #[serde(rename = "networkBandwidthBuckets")]
    pub buckets: Vec<NetworkBandwidthBucketResponse>,
}

impl ChartResponse<NetworkBandwidthDTO> for NetworkBandwidthResponse {}

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
