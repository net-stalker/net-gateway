use std::sync::Arc;

use net_core_api::api::API;
use net_core_api::decoder_api::Decoder;
use net_core_api::envelope::envelope::Envelope;
use net_core_api::typed_api::Typed;

use net_reporter_api::api::network_bandwidth_per_endpoint::network_bandwidth_per_endpoint::NetworkBandwidthPerEndpointDTO;
use net_reporter_api::api::network_bandwidth_per_endpoint::network_bandwidth_per_endpoint_request::NetworkBandwidthPerEndpointRequestDTO;

use crate::core::service_request_management::service_request_manager::ServiceRequestManager;
use crate::core::service_request_management::service_response::ServiceResponse;
use crate::core::client_data::ClientData;
use crate::core::filter::Filters;
use crate::core::general_filters::GeneralFilters;

use crate::endpoints::charts::network_bandwidth_per_endpoint::response::network_bandwidth_per_endpoint::NetworkBandwidthPerEndpointResponse;

#[derive(Default)]
pub struct NetworkBandwidthPerEndpointChartManager {}

impl NetworkBandwidthPerEndpointChartManager {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

#[async_trait::async_trait]
impl ServiceRequestManager for NetworkBandwidthPerEndpointChartManager {
    fn get_requesting_type(&self) -> &'static str {
        NetworkBandwidthPerEndpointDTO::get_data_type()
    }

    fn get_request_type(&self) -> &'static str {
        NetworkBandwidthPerEndpointRequestDTO::get_data_type()
    }

    fn form_dto_request(
        &self,
        params: Arc<GeneralFilters>,
        #[allow(unused_variables)]
        client_data: Arc<ClientData>,
        filters: Option<Arc<Filters>>,
    ) -> Box<dyn API> {
        let filters = filters.as_ref().unwrap().as_ref().clone().into();
        Box::new(NetworkBandwidthPerEndpointRequestDTO::new(
            params.start_date,
            params.end_date,
            filters,
        ))
    }

    fn decode_received_envelope(
        &self,
        received_envelope: Envelope
    ) -> Result<Box<dyn ServiceResponse>, String> {
        Ok(Box::new(NetworkBandwidthPerEndpointResponse::from(
            NetworkBandwidthPerEndpointDTO::decode(received_envelope.get_data())
        )))
    }
}
