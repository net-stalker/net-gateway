use actix_web::web;

use net_core_api::envelope::envelope::Envelope;
use net_core_api::encoder_api::Encoder;
use net_core_api::typed_api::Typed;

use net_reporter_api::api::network_bandwidth_per_endpoint::network_bandwidth_per_endpoint_request::NetworkBandwidthPerEndpointRequestDTO;

use crate::core::client_data::ClientData;
use crate::core::general_filters::GeneralFilters;
use crate::core::request_former::RequestFormer;

pub struct NetworkBandwidthPerEndpointRequestFormer {}

impl RequestFormer for NetworkBandwidthPerEndpointRequestFormer {
    fn form_enveloped_request(
        params: web::Query<GeneralFilters>,
        client_data: web::Query<ClientData>
    ) -> Envelope {
        let bandwidth_per_endpoint_request = NetworkBandwidthPerEndpointRequestDTO::new(
            params.start_date,
            params.end_date
        );
        
        Envelope::new(
            Some(client_data.group_id.as_str()),
            None,
            NetworkBandwidthPerEndpointRequestDTO::get_data_type(),
            &bandwidth_per_endpoint_request.encode()
        )
    }
}