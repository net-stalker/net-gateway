use net_reporter_api::api::http_responses::http_response::HttpResponseDTO;

use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct HttpResponseResponse {
    date: Option<u64>,
    client: String,
    server: String,
    response: u64,
}

impl HttpResponseResponse {
    pub fn new(date: Option<u64>, client: String, server: String, response: u64) -> Self {
        Self {
            date,
            client,
            server,
            response,
        }
    }
}

impl From<HttpResponseDTO> for HttpResponseResponse {
    fn from(http_response: HttpResponseDTO) -> Self {
        Self {
            date: http_response.get_date().map(|date| date as u64),
            client: http_response.get_client().to_string(),
            server: http_response.get_server().to_string(),
            response: http_response.get_response() as u64,
        }
    }
}
