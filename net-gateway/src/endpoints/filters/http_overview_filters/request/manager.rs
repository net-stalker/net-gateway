use std::error::Error;
use std::sync::Arc;

use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::core::api::API;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::typed_api::Typed;

use net_reporter_api::api::http_overview_dashboard_filters::http_overview_dashboard_filters_request::HttpOverviewDashboardFiltersRequestDTO;
use net_reporter_api::api::http_overview_dashboard_filters::http_overview_dashboard_filters::HttpOverviewDashboardFiltersDTO;

use crate::core::service_request_management::service_request_manager::ServiceRequestManager;
use crate::core::service_request_management::service_response::ServiceResponse;
use crate::core::filter::Filters;
use crate::core::general_filters::GeneralFilters;

use crate::endpoints::filters::http_overview_filters::response::http_overview_filters::HttpOverviewFiltersResponse;

#[derive(Default)]
pub struct HttpOverviewFilterManager {}

impl HttpOverviewFilterManager {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

#[async_trait::async_trait]
impl ServiceRequestManager for HttpOverviewFilterManager {
    fn get_requesting_type(&self) -> &'static str {
        HttpOverviewDashboardFiltersDTO::get_data_type()
    }

    fn get_request_type(&self) -> &'static str {
        HttpOverviewDashboardFiltersRequestDTO::get_data_type()
    }

    fn form_dto_request(
        &self,
        params: Arc<GeneralFilters>,
        _filters: Option<Arc<Filters>>,
    ) -> Box<dyn API> {
        Box::new(HttpOverviewDashboardFiltersRequestDTO::new(
            params.start_date,
            params.end_date,
        ))
    }
    
    fn decode_received_envelope(
        &self,
        received_envelope: Envelope
    ) -> Result<Box<dyn ServiceResponse>, Box<dyn Error + Send + Sync>> {
        Ok(Box::new(HttpOverviewFiltersResponse::from(
            HttpOverviewDashboardFiltersDTO::decode(received_envelope.get_data())
        )))
    }
}