use actix_web::web;

use net_reporter_api::api::network_bandwidth_per_endpoint::network_bandwidth_per_endpoint_request::NetworkBandwidthPerEndpointRequestDTO;

use crate::core::client_data::ClientData;
use crate::core::general_filters::GeneralFilters;
use crate::core::request_former::RequestFormer;

pub struct NetworkBandwidthPerEndpointRequestFormer {}

impl RequestFormer for NetworkBandwidthPerEndpointRequestFormer {
    type RequestDTO = NetworkBandwidthPerEndpointRequestDTO;

    fn form_dto_request(
        params: web::Query<GeneralFilters>,
        #[allow(unused_variables)]
        client_data: &web::Query<ClientData>
    ) -> Self::RequestDTO {
        NetworkBandwidthPerEndpointRequestDTO::new(
            params.start_date,
            params.end_date
        )
    }
}