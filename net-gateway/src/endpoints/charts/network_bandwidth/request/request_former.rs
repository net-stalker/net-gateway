use actix_web::web;

use net_reporter_api::api::network_bandwidth::network_bandwidth_request::NetworkBandwidthRequestDTO;

use crate::core::client_data::ClientData;
use crate::core::general_filters::GeneralFilters;
use crate::core::request_former::RequestFormer;

pub struct NetworkBandwidthRequestFormer {}

impl RequestFormer for NetworkBandwidthRequestFormer {
    type RequestDTO = NetworkBandwidthRequestDTO;

    fn form_dto_request(
        params: web::Query<GeneralFilters>,
        #[allow(unused_variables)]
        client_data: &web::Query<ClientData>
    ) -> Self::RequestDTO {
        NetworkBandwidthRequestDTO::new(
            params.start_date,
            params.end_date
        )
    }
}