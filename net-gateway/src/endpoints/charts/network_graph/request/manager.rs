use actix_web::web;

use net_core_api::api::API;
use net_core_api::decoder_api::Decoder;
use net_core_api::envelope::envelope::Envelope;
use net_core_api::typed_api::Typed;
use net_reporter_api::api::network_graph::network_graph::NetworkGraphDTO;
use net_reporter_api::api::network_graph::network_graph_request::NetworkGraphRequestDTO;

use crate::core::chart_management::chart_request_manager::ChartRequestManagaer;
use crate::core::client_data::ClientData;
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
impl ChartRequestManagaer<NetworkGraphResponse> for NetworkGraphChartManager {
    fn get_request_type(&self) -> &'static str {
        NetworkGraphRequestDTO::get_data_type()
    }

    fn form_dto_request(
        &self,
        params: web::Query<GeneralFilters>,
        #[allow(unused_variables)]
        client_data: &web::Query<ClientData>
    ) -> Box<dyn API> {
        Box::new(NetworkGraphRequestDTO::new(
            params.start_date,
            params.end_date,
            false
        ))
    }

    fn decode_received_envelope(
        &self,
        received_envelope: Envelope
    ) -> Result<NetworkGraphResponse, String> {
        Ok(NetworkGraphResponse::from(
            NetworkGraphDTO::decode(received_envelope.get_data())
        ))
    }
}