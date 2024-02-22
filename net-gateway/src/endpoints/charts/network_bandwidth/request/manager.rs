use std::sync::Arc;

use actix_web::web;

use net_core_api::api::API;
use net_core_api::decoder_api::Decoder;
use net_core_api::envelope::envelope::Envelope;
use net_core_api::typed_api::Typed;

use net_reporter_api::api::network_bandwidth::network_bandwidth::NetworkBandwidthDTO;
use net_reporter_api::api::network_bandwidth::network_bandwidth_request::NetworkBandwidthRequestDTO;

use crate::core::chart_management::chart_request_manager::ChartRequestManagaer;
use crate::core::chart_management::chart_response::ChartResponse;
use crate::core::client_data::ClientData;
use crate::core::filter::Filters;
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
impl ChartRequestManagaer for NetworkBandwidthChartManager {
    fn get_requesting_type(&self) -> &'static str {
        NetworkBandwidthDTO::get_data_type()
    }

    fn get_request_type(&self) -> &'static str {
        NetworkBandwidthRequestDTO::get_data_type()
    }

    fn form_dto_request(
        &self,
        params: Arc<web::Query<GeneralFilters>>,
        #[allow(unused_variables)]
        client_data: Arc<web::Query<ClientData>>,
        filters: Arc<Filters>,
    ) -> Box<dyn API> {
        Box::new(NetworkBandwidthRequestDTO::new(
            params.start_date,
            params.end_date,
            filters.as_ref().clone().into(),
        ))
    }
    
    fn decode_received_envelope(
        &self,
        received_envelope: Envelope
    ) -> Result<Box<dyn ChartResponse>, std::string::String> {
        Ok(Box::new(NetworkBandwidthResponse::from(
            NetworkBandwidthDTO::decode(received_envelope.get_data())
        )))
    }
}