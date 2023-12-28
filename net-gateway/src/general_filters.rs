use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct GeneralFilters {
    #[serde(rename = "startDate")]
    pub start_date: i64,
    #[serde(rename = "endDate")]
    pub end_date: i64,
}
