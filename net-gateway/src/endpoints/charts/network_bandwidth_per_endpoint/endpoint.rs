use actix_web::get;
use actix_web::web;
use actix_web::Responder;
use actix_web::HttpResponse;
use actix_web::HttpRequest;

use net_core_api::decoder_api::Decoder;
use net_core_api::encoder_api::Encoder;
use net_core_api::envelope::envelope::Envelope;
use net_core_api::typed_api::Typed;

use net_reporter_api::api::network_bandwidth_per_endpoint::network_bandwidth_per_endpoint::NetworkBandwidthPerEndpointDTO;
use net_reporter_api::api::network_bandwidth_per_endpoint::network_bandwidth_per_endpoint_request::NetworkBandwidthPerEndpointRequestDTO;

use crate::authorization::Authorization;
use crate::authorization::mock_authenticator::MockAuthenticator;
use crate::core::app_state::AppState;
use crate::core::client_data::ClientData;
use crate::core::general_filters::GeneralFilters;
use crate::core::quinn_client_endpoint_manager::QuinnClientEndpointManager;
use crate::endpoints::charts::network_bandwidth_per_endpoint::chart::NetworkBandwidthPerEndpoint;


//TODO: Create cool error handling
//TODO: Move all the repeatable code of creating and connecting to the server to the macro(s)
#[get("/chart/network_bandwidth_per_endpoint")]
async fn get_bandwidth_per_endpoint(
    state: web::Data<AppState>,
    client_data: web::Query<ClientData>,
    params: web::Query<GeneralFilters>,
    req: HttpRequest,
) -> impl Responder {
    //Auth stuff
    if let Err(response) = Authorization::authorize(req, MockAuthenticator {}).await {
        return response;
    }

    //Form request to the server
    let bandwidth_per_endpoint_request = NetworkBandwidthPerEndpointRequestDTO::new(
        params.start_date,
        params.end_date
    );
    let enveloped_bandwidth_per_endpoint_request = Envelope::new(
        Some(client_data.group_id.as_str()),
        None,
        NetworkBandwidthPerEndpointRequestDTO::get_data_type(),
        &bandwidth_per_endpoint_request.encode()
    );
    let bytes_to_send = enveloped_bandwidth_per_endpoint_request.encode();


    let server_connection_result = QuinnClientEndpointManager::start_server_connection(
        state.get_quinn_client_addres(),
        state.get_quinn_server_addres(),
        state.get_quinn_server_application()
    ).await;
    if let Err(e) = server_connection_result {
        //TODO: Write appropriate error returning
        return HttpResponse::InternalServerError().body(e);
    }
    let mut server_connection = server_connection_result.unwrap();


    //Sending out data (request) to the server
    let sending_result = server_connection.send_all_reliable(&bytes_to_send).await;
    if let Err(e) = sending_result {
        //TODO: Write appropriate error returning
        return HttpResponse::InternalServerError().body(e);
    }


    //Waiting on new data and reading message from the server
    let receiving_result = server_connection.receive_reliable().await;
    if let Err(e) = sending_result {
        //TODO: Write appropriate error returning
        return HttpResponse::InternalServerError().body(e);
    }
    let received_bytes = receiving_result.unwrap();

    let received_envelope = Envelope::decode(&received_bytes);


    //TODO: Think about letting it all sit here. Maybe this checking is not necessary
    if received_envelope.get_type() != NetworkBandwidthPerEndpointDTO::get_data_type() {
        //TODO: Write appropriate error returning
        return HttpResponse::InternalServerError().finish();
    }


    let received_chart = NetworkBandwidthPerEndpointDTO::decode(received_envelope.get_data());
    let chart = NetworkBandwidthPerEndpoint::from(received_chart);
    
    HttpResponse::Ok().json(chart)
}
