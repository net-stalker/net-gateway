use actix_web::web;

use net_core_api::encoder_api::Encoder;
use net_core_api::envelope::envelope::Envelope;

use super::client_data::ClientData;
use super::general_filters::GeneralFilters;

pub trait RequestFormer {
    fn form_enveloped_request(
        params: web::Query<GeneralFilters>,
        client_data: web::Query<ClientData>
    ) -> Envelope;

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