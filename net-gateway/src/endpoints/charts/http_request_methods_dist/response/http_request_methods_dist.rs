use net_reporter_api::api::http_request_methods_dist::http_request_methods_dist::HttpRequestMethodsDistDTO;
use serde::Deserialize;
use serde::Serialize;

use net_core_api::typed_api::Typed;

use net_reporter_api::api::total_http_requests::total_http_requests::TotalHttpRequestsDTO;

use crate::core::service_request_management::service_response::ServiceResponse;

use super::http_request::HttpRequestResponse;

const JSON_TYPE: &str = "httpRequestMethodsDist";

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct HttpRequestMethodsDistResponse {
    #[serde(rename = "httpRequests")]
    pub http_requests: Vec<HttpRequestResponse>,
}
impl ServiceResponse for HttpRequestMethodsDistResponse {
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

impl HttpRequestMethodsDistResponse {
    pub fn new(http_requests: Vec<HttpRequestResponse>) -> Self {
        Self { http_requests }
    }
}

impl From<HttpRequestMethodsDistDTO> for HttpRequestMethodsDistResponse {
    fn from(data: HttpRequestMethodsDistDTO) -> Self {
        let http_requests = data
            .get_http_requests()
            .iter()
            .map(|http_request| HttpRequestResponse::from(http_request.clone()))
            .collect();
        Self { http_requests }
    }
}
