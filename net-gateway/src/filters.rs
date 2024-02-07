use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate, Clone)]
pub struct Filters {
    #[serde(rename = "filters")]
    pub filters: String
}

impl Filters {
    pub fn new(filters: String) -> Self {
        Self { filters }
    }
}

pub trait MapFiltersToDTO {
    type DTO;
    fn map_filters(self) -> Self::DTO;
}
