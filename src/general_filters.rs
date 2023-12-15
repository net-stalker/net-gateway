use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct GeneralFilters {
    pub start_date: i64,
    pub end_date: i64,
}
