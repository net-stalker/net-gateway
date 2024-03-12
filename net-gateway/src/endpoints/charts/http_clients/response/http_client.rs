use net_reporter_api::api::http_clients::http_client;

use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct HttpClientResponse {
    #[serde(rename = "endpoint")]
    endpoint: String,
    #[serde(rename = "userAgent")]
    user_agent: Option<String>,
    #[serde(rename = "requestsAmount")]
    requests_amount: u64,
}

impl HttpClientResponse {
    pub fn new(endpoint: String, user_agent: Option<String>, requests_amount: u64) -> Self {
        Self {
            endpoint,
            user_agent,
            requests_amount,
        }
    }
}

impl From<http_client::HttpClientDTO> for HttpClientResponse {
    fn from(client: http_client::HttpClientDTO) -> Self {
        Self {
            endpoint: client.get_endpoint().to_string(),
            user_agent: client.get_user_agent().map(|s| s.to_string()),
            requests_amount: client.get_requests_amount() as u64, 
        }
    }
}
