use net_timescale_api::api::network_bandwidth::network_bandwidth::NetworkBandwidthDTO;
use serde::{Serialize, Deserialize};

use super::network_bandwidth_bucket::NetworkBandwidthBucket;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetworkBandwidth {
    #[serde(rename = "networkBandwidthBuckets")]
    pub buckets: Vec<NetworkBandwidthBucket>,
}

impl NetworkBandwidth {
    pub fn new(buckets: Vec<NetworkBandwidthBucket>) -> Self {
        Self { buckets }
    }
}

impl Default for NetworkBandwidth {
    fn default() -> Self {
        log::info!("warinng: default NetworkBandwidth is being constructed");
        Self { buckets: Vec::new() }
    }
}

impl From<NetworkBandwidthDTO> for NetworkBandwidth {
    fn from(data: NetworkBandwidthDTO) -> Self {
        let buckets = data
            .get_bandwidth_buckets()
            .iter()
            .map(|bucket| NetworkBandwidthBucket::from(bucket.clone()))
            .collect();
        Self { buckets }
    }
}
