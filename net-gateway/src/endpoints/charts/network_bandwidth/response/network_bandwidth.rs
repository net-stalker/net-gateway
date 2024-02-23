use serde::Deserialize;
use serde::Serialize;

use net_core_api::typed_api::Typed;

use net_reporter_api::api::network_bandwidth::network_bandwidth::NetworkBandwidthDTO;

use crate::core::service_request_management::service_response::ServiceResponse;

use super::network_bandwidth_bucket::NetworkBandwidthBucketResponse;

const JSON_TYPE: &str = "networkBandwidth";

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct NetworkBandwidthResponse {
    #[serde(rename = "networkBandwidthBuckets")]
    pub buckets: Vec<NetworkBandwidthBucketResponse>,
}
impl ServiceResponse for NetworkBandwidthResponse {
    fn get_dto_type(&self) -> &'static str {
        NetworkBandwidthDTO::get_data_type()
    }

    fn get_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }

    fn get_json_type(&self) -> &'static str {
        JSON_TYPE
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
