use net_reporter_api::api::network_bandwidth::bandwidth_bucket;

use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct NetworkBandwidthBucketResponse {
    #[serde(rename = "bucketTimestamp")]
    bucket_timestamp: u64,
    #[serde(rename = "totalBytes")]
    total_bytes: u64,
}

impl NetworkBandwidthBucketResponse {
    pub fn new(bucket_timestamp: u64, total_bytes: u64) -> Self {
        Self {
            bucket_timestamp,
            total_bytes,
        }
    }
}

impl From<bandwidth_bucket::BandwidthBucketDTO> for NetworkBandwidthBucketResponse {
    fn from(endpoint_dto: bandwidth_bucket::BandwidthBucketDTO) -> Self {
        Self {
            bucket_timestamp: endpoint_dto.get_bucket_timestamp() as u64,
            total_bytes: endpoint_dto.get_total_bytes() as u64,
        }
    }
}