use net_reporter_api::api::http_responses_distribution::http_responses_distribution::HttpResponsesDistributionDTO;
use serde::Deserialize;
use serde::Serialize;

use net_core_api::typed_api::Typed;

use crate::core::service_request_management::service_response::ServiceResponse;

use super::http_responses_distribution_bucket::HttpResponsesDistributionBucketResponse;

const JSON_TYPE: &str = "httpResponsesDistribution";

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct HttpResponsesDistributionResponse {
    #[serde(rename = "httpResponsesDistributionBuckets")]
    pub buckets: Vec<HttpResponsesDistributionBucketResponse>,
}
impl ServiceResponse for HttpResponsesDistributionResponse {
    fn get_dto_type(&self) -> &'static str {
        HttpResponsesDistributionDTO::get_data_type()
    }

    fn get_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }

    fn get_json_type(&self) -> &'static str {
        JSON_TYPE
    }
}

impl HttpResponsesDistributionResponse {
    pub fn new(buckets: Vec<HttpResponsesDistributionBucketResponse>) -> Self {
        Self { buckets }
    }
}

impl From<HttpResponsesDistributionDTO> for HttpResponsesDistributionResponse {
    fn from(data: HttpResponsesDistributionDTO) -> Self {
        let buckets = data
            .get_http_responses_buckets()
            .iter()
            .map(|bucket| HttpResponsesDistributionBucketResponse::from(bucket.clone()))
            .collect();
        Self { buckets }
    }
}
