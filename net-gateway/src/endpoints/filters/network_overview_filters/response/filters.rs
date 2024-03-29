use net_core_api::core::typed_api::Typed;
// here we need to impl MapFilterToDTO for our DTO type which we still need to update
use net_reporter_api::api::network_overview_dashboard_filters::network_overview_dashbord_filters::NetworkOverviewDashboardFiltersDTO;
use serde::{Deserialize, Serialize};
use validator::Validate;

const JSON_TYPE: &str = "filters";

use crate::core::service_request_management::service_response::ServiceResponse;

use super::filter_entry::NetworkOverviewFilterResponse;

#[derive(Serialize, Deserialize, Validate, Default, Debug, Clone)]
pub struct NetworkOverviewFiltersResponse {
    pub entries: Vec<NetworkOverviewFilterResponse>,
}

impl ServiceResponse for NetworkOverviewFiltersResponse {
    fn get_dto_type(&self) -> &'static str {
        NetworkOverviewDashboardFiltersDTO::get_data_type()
    }

    fn get_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }

    fn get_json_type(&self) -> &'static str {
        JSON_TYPE
    }
}

impl From<NetworkOverviewDashboardFiltersDTO> for NetworkOverviewFiltersResponse {
    fn from(value: NetworkOverviewDashboardFiltersDTO) -> Self {
        NetworkOverviewFiltersResponse {
            entries: value
                .get_entries()
                .iter()
                .map(|entry| NetworkOverviewFilterResponse::from(entry.clone()))
                .collect(),
        }
    }
}
