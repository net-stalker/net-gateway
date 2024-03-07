use net_reporter_api::api::http_request_methods_dist::http_request::HttpRequestDTO;

use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct HttpRequestResponse {
    name: String,
    amount: u64,
}

impl HttpRequestResponse {
    pub fn new(name: &str, amount: u64) -> Self {
        Self {
            name: name.to_string(),
            amount,
        }
    }
}

impl From<HttpRequestDTO> for HttpRequestResponse {
    fn from(http_request: HttpRequestDTO) -> Self {
        Self {
            name: http_request.get_method_name().to_string(),
            amount: http_request.get_amount() as u64,
        }
    }
}
