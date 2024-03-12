use net_reporter_api::api::http_request_methods_distribution::http_request_methods_distribution::HttpRequestMethodsDistributionDTO;
use serde::Deserialize;
use serde::Serialize;

use net_core_api::typed_api::Typed;

use net_reporter_api::api::total_http_requests::total_http_requests::TotalHttpRequestsDTO;

use crate::core::service_request_management::service_response::ServiceResponse;

use super::http_request_method::HttpRequestMethodResponse;

const JSON_TYPE: &str = "httpRequestMethodsDistribution";

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct HttpRequestMethodsDistributionResponse {
    #[serde(rename = "httpRequestMethods")]
    pub http_request_methods: Vec<HttpRequestMethodResponse>,
}
impl ServiceResponse for HttpRequestMethodsDistributionResponse {
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

impl HttpRequestMethodsDistributionResponse {
    pub fn new(http_request_methods: Vec<HttpRequestMethodResponse>) -> Self {
        Self { http_request_methods }
    }
}

impl From<HttpRequestMethodsDistributionDTO> for HttpRequestMethodsDistributionResponse {
    fn from(data: HttpRequestMethodsDistributionDTO) -> Self {
        let http_request_methods = data
            .get_http_requests()
            .iter()
            .map(|http_request_method| HttpRequestMethodResponse::from(http_request_method.clone()))
            .collect();
        Self { http_request_methods }
    }
}
