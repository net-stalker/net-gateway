use std::error::Error;
use std::sync::Arc;

use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::core::api::API;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::typed_api::Typed;

use net_reporter_api::api::network_overview_dashboard_filters::network_overview_dashboard_filters_request::NetworkOverviewDashboardFiltersRequestDTO;
use net_reporter_api::api::network_overview_dashboard_filters::network_overview_dashbord_filters::NetworkOverviewDashboardFiltersDTO;

use crate::core::service_request_management::service_request_manager::ServiceRequestManager;
use crate::core::service_request_management::service_response::ServiceResponse;
use crate::core::filter::Filters;
use crate::core::general_filters::GeneralFilters;

use crate::endpoints::filters::network_overview_filters::response::filters::NetworkOverviewFiltersResponse;

#[derive(Default)]
pub struct NetworkOverviewFilterManager {}

impl NetworkOverviewFilterManager {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

#[async_trait::async_trait]
impl ServiceRequestManager for NetworkOverviewFilterManager {
    fn get_requesting_type(&self) -> &'static str {
        NetworkOverviewDashboardFiltersDTO::get_data_type()
    }

    fn get_request_type(&self) -> &'static str {
        NetworkOverviewDashboardFiltersRequestDTO::get_data_type()
    }

    fn form_dto_request(
        &self,
        params: Arc<GeneralFilters>,
        _filters: Option<Arc<Filters>>,
    ) -> Box<dyn API> {
        Box::new(NetworkOverviewDashboardFiltersRequestDTO::new(
            params.start_date,
            params.end_date,
        ))
    }
    
    fn decode_received_envelope(
        &self,
        received_envelope: Envelope
    ) -> Result<Box<dyn ServiceResponse>, Box<dyn Error + Send + Sync>> {
        Ok(Box::new(NetworkOverviewFiltersResponse::from(
            NetworkOverviewDashboardFiltersDTO::decode(received_envelope.get_data())
        )))
    }
}