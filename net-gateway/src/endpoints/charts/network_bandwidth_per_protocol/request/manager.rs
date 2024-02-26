use std::sync::Arc;
use net_core_api::api::API;
use net_core_api::decoder_api::Decoder;
use net_core_api::envelope::envelope::Envelope;
use net_core_api::typed_api::Typed;

use net_reporter_api::api::network_bandwidth_per_protocol::network_bandwidth_per_protocol::NetworkBandwidthPerProtocolDTO;
use net_reporter_api::api::network_bandwidth_per_protocol::network_bandwidth_per_protocol_request::NetworkBandwidthPerProtocolRequestDTO;

use crate::core::service_request_management::service_request_manager::ServiceRequestManager;
use crate::core::service_request_management::service_response::ServiceResponse;
use crate::core::client_data::ClientData;
use crate::core::filter::Filters;
use crate::core::general_filters::GeneralFilters;

use crate::endpoints::charts::network_bandwidth_per_protocol::response::network_bandwidth_per_protocol::NetworkBandwidthPerProtocolResponse;

#[derive(Default)]
pub struct NetworkBandwidthPerProtocolChartManager {}

impl NetworkBandwidthPerProtocolChartManager {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

#[async_trait::async_trait]
impl ServiceRequestManager for NetworkBandwidthPerProtocolChartManager {
    fn get_requesting_type(&self) -> &'static str {
        NetworkBandwidthPerProtocolDTO::get_data_type()
    }

    fn get_request_type(&self) -> &'static str {
        NetworkBandwidthPerProtocolRequestDTO::get_data_type()
    }

    fn form_dto_request(
        &self,
        params: Arc<GeneralFilters>,
        #[allow(unused_variables)]
        client_data: Arc<ClientData>,
        filters: Option<Arc<Filters>>,
    ) -> Box<dyn API> {
        let filters = filters.as_ref().unwrap().as_ref().clone().into();
        Box::new(NetworkBandwidthPerProtocolRequestDTO::new(
            params.start_date,
            params.end_date,
            filters,
        ))
    }

    fn decode_received_envelope(
        &self,
        received_envelope: Envelope
    ) -> Result<Box<dyn ServiceResponse>, String> {
        Ok(Box::new(NetworkBandwidthPerProtocolResponse::from(
            NetworkBandwidthPerProtocolDTO::decode(received_envelope.get_data())
        )))
    }
}
