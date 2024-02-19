use actix_web::web;

use net_reporter_api::api::network_graph::network_graph_request::NetworkGraphRequestDTO;

use crate::core::client_data::ClientData;
use crate::core::general_filters::GeneralFilters;
use crate::core::request_creator::RequestCreator;

pub struct NetworkGraphRequestFormer {}

impl RequestCreator for NetworkGraphRequestFormer {
    type RequestDTO = NetworkGraphRequestDTO;

    fn form_dto_request(
        params: web::Query<GeneralFilters>,
        #[allow(unused_variables)]
        client_data: &web::Query<ClientData>
    ) -> Self::RequestDTO {
        NetworkGraphRequestDTO::new(
            params.start_date,
            params.end_date,
            //TODO: Get rid of subscribe
            false
        )
    }
}