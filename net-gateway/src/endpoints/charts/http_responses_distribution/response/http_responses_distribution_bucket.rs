use net_reporter_api::api::http_responses_distribution::http_responses_distribution_bucket;

use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct HttpResponsesDistributionBucketResponse {
    #[serde(rename = "bucketTimestamp")]
    bucket_timestamp: u64,
    #[serde(rename = "responseCode")]
    response_code: u64,
    amount: u64,
}

impl HttpResponsesDistributionBucketResponse {
    pub fn new(bucket_timestamp: u64, response_code: u64, amount: u64) -> Self {
        Self {
            bucket_timestamp,
            response_code,
            amount,
        }
    }
}

impl From<http_responses_distribution_bucket::HttpResponsesDistributionBucketDTO> for HttpResponsesDistributionBucketResponse {
    fn from(bucket: http_responses_distribution_bucket::HttpResponsesDistributionBucketDTO) -> Self {
        Self {
            bucket_timestamp: bucket.get_bucket_timestamp() as u64,
            response_code: bucket.get_response_code() as u64,
            amount: bucket.get_response_amount() as u64, 
        }
    }
}
