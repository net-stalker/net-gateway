use serde::Deserialize;
use serde::Serialize;

use net_core_api::typed_api::Typed;

use net_reporter_api::api::total_http_requests::total_http_requests::TotalHttpRequestsDTO;

use crate::core::service_request_management::service_response::ServiceResponse;

use super::http_requests_bucket::HttpRequestsBucketResponse;

const JSON_TYPE: &str = "totalHttpRequests";

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TotalHttpRequestsResponse {
    #[serde(rename = "httpRequestsBuckets")]
    pub buckets: Vec<HttpRequestsBucketResponse>,
}
impl ServiceResponse for TotalHttpRequestsResponse {
    fn get_dto_type(&self) -> &'static str {
        TotalHttpRequestsDTO::get_data_type()
    }

    fn get_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }

    fn get_json_type(&self) -> &'static str {
        JSON_TYPE
    }
}

impl TotalHttpRequestsResponse {
    pub fn new(buckets: Vec<HttpRequestsBucketResponse>) -> Self {
        Self { buckets }
    }
}

impl From<TotalHttpRequestsDTO> for TotalHttpRequestsResponse {
    fn from(data: TotalHttpRequestsDTO) -> Self {
        let buckets = data
            .get_http_requests_buckets()
            .iter()
            .map(|bucket| HttpRequestsBucketResponse::from(bucket.clone()))
            .collect();
        Self { buckets }
    }
}
