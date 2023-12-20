use net_timescale_api::api::network_bandwidth::bandwidth_bucket;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetworkBandwidthBucket {
    #[serde(rename = "bucketTimestamp")]
    bucket_timestamp: u64,
    #[serde(rename = "totalBytes")]
    total_bytes: u64,
}

impl NetworkBandwidthBucket {
    pub fn new(bucket_timestamp: u64, total_bytes: u64) -> Self {
        Self {
            bucket_timestamp,
            total_bytes,
        }
    }
}

impl From<bandwidth_bucket::BandwidthBucketDTO> for NetworkBandwidthBucket {
    fn from(endpoint_dto: bandwidth_bucket::BandwidthBucketDTO) -> Self {
        Self {
            bucket_timestamp: endpoint_dto.get_bucket_timestamp() as u64,
            total_bytes: endpoint_dto.get_total_bytes() as u64,
        }
    }
}