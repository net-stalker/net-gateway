use actix_web::web;

use net_core_api::api::API;
use net_core_api::encoder_api::Encoder;
use net_core_api::envelope::envelope::Envelope;
use net_core_api::typed_api::Typed;

use super::client_data::ClientData;
use super::general_filters::GeneralFilters;

pub trait RequestCreator {
    type RequestDTO: API;

    fn form_dto_request(
        params: web::Query<GeneralFilters>,
        client_data: &web::Query<ClientData>
    ) -> Self::RequestDTO;

    fn form_enveloped_request(
        params: web::Query<GeneralFilters>,
        client_data: web::Query<ClientData>
    ) -> Envelope {
        Envelope::new(
            Some(&client_data.group_id),
            None,
            <Self::RequestDTO as Typed>::get_data_type(),
            &Self::form_dto_request(
                params,
                &client_data
            ).encode()
        )
    }

    fn form_request(
        params: web::Query<GeneralFilters>,
        client_data: web::Query<ClientData>
    ) -> Vec<u8> {
        Self::form_enveloped_request(
            params,
            client_data
        ).encode()
    }
}