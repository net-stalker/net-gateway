use net_reporter_api::api::network_overview_dashboard_filters::filter_entry::FilterEntryDTO;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, Default, Debug, Clone)]
pub struct NetworkOverviewFilter {
    pub endpoint: String,
    pub protocols: Vec<String>,
    #[serde(rename = "totalBytes")]
    pub total_bytes: i64,
}

impl From<FilterEntryDTO> for NetworkOverviewFilter {
    fn from(dto: FilterEntryDTO) -> Self {
        NetworkOverviewFilter {
            endpoint: dto.get_endpoint().to_string(),
            protocols: dto.get_protocols().to_vec(),
            total_bytes: dto.get_total_bytes(),
        }
    }
}
