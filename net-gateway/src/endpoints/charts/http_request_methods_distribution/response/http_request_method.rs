use net_reporter_api::api::http_request_methods_distribution::http_request_method::HttpRequestMethodDTO;

use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct HttpRequestMethodResponse {
    name: String,
    amount: u64,
}

impl HttpRequestMethodResponse {
    pub fn new(name: &str, amount: u64) -> Self {
        Self {
            name: name.to_string(),
            amount,
        }
    }
}

impl From<HttpRequestMethodDTO> for HttpRequestMethodResponse {
    fn from(http_request: HttpRequestMethodDTO) -> Self {
        Self {
            name: http_request.get_method_name().to_string(),
            amount: http_request.get_amount() as u64,
        }
    }
}
