use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate, Clone)]
pub struct GeneralFilters {
    #[serde(rename = "startDate")]
    pub start_date: i64,
    #[serde(rename = "endDate")]
    pub end_date: i64,
    #[serde(rename = "networkId")]
    pub network_id: String,
}
