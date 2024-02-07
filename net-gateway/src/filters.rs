use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate, Clone)]
pub struct Filters {
    #[serde(rename = "filters")]
    pub filters: Option<String>,
}

impl Filters {
    pub fn new(filters: String) -> Self {
        Self { filters: Some(filters) }
    }
}

pub trait MapFiltersToDTO {
    fn map_filters(filters: Filters) -> Self;
}
