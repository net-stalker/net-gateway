use net_reporter_api::api::http_responses_dist::http_responses_dist::HttpResponsesDistDTO;
use serde::Deserialize;
use serde::Serialize;

use net_core_api::typed_api::Typed;

use crate::core::service_request_management::service_response::ServiceResponse;

use super::http_responses_dist_bucket::HttpResponsesDistBucketResponse;

const JSON_TYPE: &str = "httpResponsesDist";

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct HttpResponsesDistResponse {
    #[serde(rename = "httpResponsesDistBuckets")]
    pub buckets: Vec<HttpResponsesDistBucketResponse>,
}
impl ServiceResponse for HttpResponsesDistResponse {
    fn get_dto_type(&self) -> &'static str {
        HttpResponsesDistDTO::get_data_type()
    }

    fn get_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }

    fn get_json_type(&self) -> &'static str {
        JSON_TYPE
    }
}

impl HttpResponsesDistResponse {
    pub fn new(buckets: Vec<HttpResponsesDistBucketResponse>) -> Self {
        Self { buckets }
    }
}

impl From<HttpResponsesDistDTO> for HttpResponsesDistResponse {
    fn from(data: HttpResponsesDistDTO) -> Self {
        let buckets = data
            .get_http_responses_buckets()
            .iter()
            .map(|bucket| HttpResponsesDistBucketResponse::from(bucket.clone()))
            .collect();
        Self { buckets }
    }
}
