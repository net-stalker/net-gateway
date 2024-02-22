use std::sync::Arc;

use actix_web::web;

use net_core_api::api::API;
use net_core_api::decoder_api::Decoder;
use net_core_api::envelope::envelope::Envelope;
use net_core_api::typed_api::Typed;

use net_reporter_api::api::network_bandwidth_per_protocol::network_bandwidth_per_protocol::NetworkBandwidthPerProtocolDTO;
use net_reporter_api::api::network_bandwidth_per_protocol::network_bandwidth_per_protocol_request::NetworkBandwidthPerProtocolRequestDTO;

use crate::core::chart_management::chart_request_manager::ChartRequestManagaer;
use crate::core::chart_management::chart_response::ChartResponse;
use crate::core::client_data::ClientData;
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
impl ChartRequestManagaer for NetworkBandwidthPerProtocolChartManager {
    fn get_requesting_type(&self) -> &'static str {
        NetworkBandwidthPerProtocolDTO::get_data_type()
    }

    fn get_request_type(&self) -> &'static str {
        NetworkBandwidthPerProtocolRequestDTO::get_data_type()
    }

    fn form_dto_request(
        &self,
        params: Arc<web::Query<GeneralFilters>>,
        #[allow(unused_variables)]
        client_data: Arc<web::Query<ClientData>>
    ) -> Box<dyn API> {
        Box::new(NetworkBandwidthPerProtocolRequestDTO::new(
            params.start_date,
            params.end_date
        ))
    }

    fn decode_received_envelope(
        &self,
        received_envelope: Envelope
    ) -> Result<Box<dyn ChartResponse>, String> {
        Ok(Box::new(NetworkBandwidthPerProtocolResponse::from(
            NetworkBandwidthPerProtocolDTO::decode(received_envelope.get_data())
        )))
    }
}
