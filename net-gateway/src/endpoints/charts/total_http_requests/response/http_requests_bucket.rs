use net_reporter_api::api::total_http_requests::http_requests_bucket::HttpRequestsBucketDTO;

use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct HttpRequestsBucketResponse {
    #[serde(rename = "bucketTimestamp")]
    bucket_timestamp: u64,
    #[serde(rename = "totalRequests")]
    total_requests: u64,
}

impl HttpRequestsBucketResponse {
    pub fn new(bucket_timestamp: u64, total_requests: u64) -> Self {
        Self {
            bucket_timestamp,
            total_requests,
        }
    }
}

impl From<HttpRequestsBucketDTO> for HttpRequestsBucketResponse {
    fn from(bucket_dto: HttpRequestsBucketDTO) -> Self {
        Self {
            bucket_timestamp: bucket_dto.get_bucket_timestamp() as u64,
            total_requests: bucket_dto.get_total_requests() as u64,
        }
    }
}
