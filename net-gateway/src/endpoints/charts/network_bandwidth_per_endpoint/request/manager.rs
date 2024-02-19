use actix_web::web;
use net_core_api::{api::API, decoder_api::Decoder, envelope::envelope::Envelope, typed_api::Typed};
use net_reporter_api::api::network_bandwidth_per_endpoint::{network_bandwidth_per_endpoint::NetworkBandwidthPerEndpointDTO, network_bandwidth_per_endpoint_request::NetworkBandwidthPerEndpointRequestDTO};

use crate::{core::{chart_management::chart_request_manager::ChartRequestManagaer, client_data::ClientData, general_filters::GeneralFilters}, endpoints::charts::network_bandwidth_per_endpoint::response::network_bandwidth_per_endpoint::NetworkBandwidthPerEndpointResponse};

#[derive(Default)]
pub struct NetworkBandwidthPerEndpointChartManager {}

impl NetworkBandwidthPerEndpointChartManager {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

#[async_trait::async_trait]
impl ChartRequestManagaer<NetworkBandwidthPerEndpointResponse> for NetworkBandwidthPerEndpointChartManager {
    fn get_request_type(&self) -> &'static str {
        NetworkBandwidthPerEndpointRequestDTO::get_data_type()
    }

    fn form_dto_request(
        &self,
        params: web::Query<GeneralFilters>,
        #[allow(unused_variables)]
        client_data: &web::Query<ClientData>
    ) -> Box<dyn API> {
        Box::new(NetworkBandwidthPerEndpointRequestDTO::new(
            params.start_date,
            params.end_date
        ))
    }

    fn decode_resieved_envelope(
        &self,
        received_envelope: Envelope
    ) -> Result<NetworkBandwidthPerEndpointResponse, String> {
        Ok(NetworkBandwidthPerEndpointResponse::from(
            NetworkBandwidthPerEndpointDTO::decode(received_envelope.get_data())
        ))
    }
}
