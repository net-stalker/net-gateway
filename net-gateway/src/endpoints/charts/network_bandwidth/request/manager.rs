use actix_web::web;

use net_core_api::api::API;
use net_core_api::decoder_api::Decoder;
use net_core_api::envelope::envelope::Envelope;
use net_core_api::typed_api::Typed;
use net_reporter_api::api::network_bandwidth::network_bandwidth::NetworkBandwidthDTO;
use net_reporter_api::api::network_bandwidth::network_bandwidth_request::NetworkBandwidthRequestDTO;

use crate::core::chart_management::chart_request_manager::ChartRequestManagaer;
use crate::core::client_data::ClientData;
use crate::core::general_filters::GeneralFilters;
use crate::endpoints::charts::network_bandwidth::response::network_bandwidth::NetworkBandwidthResponse;

#[derive(Default)]
pub struct NetworkBandwidthChartManager {}

impl NetworkBandwidthChartManager {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

#[async_trait::async_trait]
impl ChartRequestManagaer<NetworkBandwidthResponse> for NetworkBandwidthChartManager {
    fn get_request_type(&self) -> &'static str {
        NetworkBandwidthRequestDTO::get_data_type()
    }

    fn form_dto_request(
        &self,
        params: web::Query<GeneralFilters>,
        #[allow(unused_variables)]
        client_data: &web::Query<ClientData>
    ) -> Box<dyn API> {
        Box::new(NetworkBandwidthRequestDTO::new(
            params.start_date,
            params.end_date
        ))
    }
    
    fn decode_resieved_envelope(
        &self,
        received_envelope: Envelope
    ) -> Result<NetworkBandwidthResponse, String> {
        Ok(NetworkBandwidthResponse::from(
            NetworkBandwidthDTO::decode(received_envelope.get_data())
        ))
    }
}