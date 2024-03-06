use std::sync::Arc;

use net_core_api::api::API;
use net_core_api::decoder_api::Decoder;
use net_core_api::envelope::envelope::Envelope;
use net_core_api::typed_api::Typed;

use net_reporter_api::api::network_graph::network_graph::NetworkGraphDTO;
use net_reporter_api::api::network_graph::network_graph_request::NetworkGraphRequestDTO;

use crate::core::service_request_management::service_request_manager::ServiceRequestManager;
use crate::core::service_request_management::service_response::ServiceResponse;
use crate::core::filter::Filters;
use crate::core::general_filters::GeneralFilters;

use crate::endpoints::charts::network_graph::response::network_graph::NetworkGraphResponse;

#[derive(Default)]
pub struct NetworkGraphChartManager {}

impl NetworkGraphChartManager {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

#[async_trait::async_trait]
impl ServiceRequestManager for NetworkGraphChartManager {
    fn get_requesting_type(&self) -> &'static str {
        NetworkGraphDTO::get_data_type()
    }

    fn get_request_type(&self) -> &'static str {
        NetworkGraphRequestDTO::get_data_type()
    }

    fn form_dto_request(
        &self,
        params: Arc<GeneralFilters>,
        filters: Option<Arc<Filters>>,
    ) -> Box<dyn API> {
        let filters = filters.as_ref().unwrap().as_ref().clone().into();
        Box::new(NetworkGraphRequestDTO::new(
            params.start_date,
            params.end_date,
            filters,
        ))
    }

    fn decode_received_envelope(
        &self,
        received_envelope: Envelope
    ) -> Result<Box<dyn ServiceResponse>, String> {
        Ok(Box::new(NetworkGraphResponse::from(
            NetworkGraphDTO::decode(received_envelope.get_data())
        )))
    }
}