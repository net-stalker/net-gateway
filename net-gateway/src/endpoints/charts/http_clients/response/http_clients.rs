use net_reporter_api::api::http_clients::http_clients::HttpClientsDTO;
use serde::Deserialize;
use serde::Serialize;

use net_core_api::typed_api::Typed;

use crate::core::service_request_management::service_response::ServiceResponse;

use super::http_client::HttpClientResponse;

const JSON_TYPE: &str = "httpClients";

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct HttpClientsResponse {
    #[serde(rename = "httpClients")]
    pub http_clients: Vec<HttpClientResponse>,
}
impl ServiceResponse for HttpClientsResponse {
    fn get_dto_type(&self) -> &'static str {
        HttpClientsDTO::get_data_type()
    }

    fn get_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }

    fn get_json_type(&self) -> &'static str {
        JSON_TYPE
    }
}

impl HttpClientsResponse {
    pub fn new(http_clients: Vec<HttpClientResponse>) -> Self {
        Self { http_clients }
    }
}

impl From<HttpClientsDTO> for HttpClientsResponse {
    fn from(data: HttpClientsDTO) -> Self {
        let http_clients = data
            .get_http_clients()
            .iter()
            .map(|bucket| HttpClientResponse::from(bucket.clone()))
            .collect();
        Self { http_clients }
    }
}
