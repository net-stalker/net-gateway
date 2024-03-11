use serde::Deserialize;
use serde::Serialize;

use net_core_api::typed_api::Typed;

use net_reporter_api::api::http_responses::http_responses::HttpResponsesDTO;

use crate::core::service_request_management::service_response::ServiceResponse;

use super::http_response::HttpResponseResponse;

const JSON_TYPE: &str = "totalHttpRequests";

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct HttpResponsesResponse {
    #[serde(rename = "httpResponses")]
    pub http_responses: Vec<HttpResponseResponse>,
}
impl ServiceResponse for HttpResponsesResponse {
    fn get_dto_type(&self) -> &'static str {
        HttpResponsesDTO::get_data_type()
    }

    fn get_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }

    fn get_json_type(&self) -> &'static str {
        JSON_TYPE
    }
}

impl HttpResponsesResponse {
    pub fn new(http_responses: Vec<HttpResponseResponse>) -> Self {
        Self { http_responses }
    }
}

impl From<HttpResponsesDTO> for HttpResponsesResponse {
    fn from(data: HttpResponsesDTO) -> Self {
        let http_responses = data
            .get_http_responses()
            .iter()
            .map(|bucket| HttpResponseResponse::from(bucket.clone()))
            .collect();
        Self { http_responses }
    }
}
