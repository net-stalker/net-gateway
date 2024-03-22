use net_core_api::core::typed_api::Typed;
// here we need to impl MapFilterToDTO for our DTO type which we still need to update
use net_reporter_api::api::http_overview_dashboard_filters::http_overview_dashboard_filters::HttpOverviewDashboardFiltersDTO;
use serde::{Deserialize, Serialize};
use validator::Validate;

const JSON_TYPE: &str = "filters";

use crate::core::service_request_management::service_response::ServiceResponse;

#[derive(Serialize, Deserialize, Validate, Default, Debug, Clone)]
pub struct HttpOverviewFiltersResponse {
    pub endpoints: Vec<String>,
    #[serde(rename = "httpRequestMethods")]
    pub http_request_methods: Vec<String>,
    #[serde(rename = "httpResponseCodes")]
    pub http_response_codes: Vec<String>,
}

impl ServiceResponse for HttpOverviewFiltersResponse {
    fn get_dto_type(&self) -> &'static str {
        HttpOverviewDashboardFiltersDTO::get_data_type()
    }

    fn get_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }

    fn get_json_type(&self) -> &'static str {
        JSON_TYPE
    }
}

impl From<HttpOverviewDashboardFiltersDTO> for HttpOverviewFiltersResponse {
    fn from(value: HttpOverviewDashboardFiltersDTO) -> Self {
        HttpOverviewFiltersResponse {
            endpoints: value.get_endpoints().to_vec(),
            http_request_methods: value.get_request_methods().to_vec(),
            http_response_codes: value.get_response_codes().to_vec(),
        }
    }
}
