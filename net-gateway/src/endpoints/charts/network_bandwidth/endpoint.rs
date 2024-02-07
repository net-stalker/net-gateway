use actix_web::get;
use actix_web::web;
use actix_web::Responder;
use actix_web::HttpResponse;
use actix_web::HttpRequest;

use net_core_api::decoder_api::Decoder;
use net_core_api::encoder_api::Encoder;
use net_core_api::envelope::envelope::Envelope;
use net_core_api::typed_api::Typed;

use net_reporter_api::api::network_bandwidth::network_bandwidth::NetworkBandwidthDTO;
use net_reporter_api::api::network_bandwidth::network_bandwidth_request::NetworkBandwidthRequestDTO;

use net_transport::quinn::client::builder::ClientQuicEndpointBuilder;

use crate::authorization::Authorization;
use crate::authorization::mock_authenticator::MockAuthenticator;
use crate::core::app_state::AppState;
use crate::core::client_data::ClientData;
use crate::core::general_filters::GeneralFilters;
use crate::endpoints::charts::network_bandwidth::chart::NetworkBandwidth;


#[get("/chart/network_bandwidth")]
async fn get_network_bandwidth(
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
    let network_bandwidth_request = NetworkBandwidthRequestDTO::new(
        params.start_date,
        params.end_date
    );
    let enveloped_network_bandwidth_request = Envelope::new(
        Some(client_data.group_id.as_str()),
        None,
        NetworkBandwidthRequestDTO::get_data_type(),
        &network_bandwidth_request.encode()
    );
    let bytes_to_send = enveloped_network_bandwidth_request.encode();


    //Creating Quinn Client Endpoint
    let client_endpoint_build_result = ClientQuicEndpointBuilder::default()
        .with_addr(state.get_quinn_client_addres().parse().unwrap())
        .build();
    if let Err(e) = client_endpoint_build_result {
        //TODO: Write appropriate error returning
        return HttpResponse::InternalServerError().body(e);
    }
    let mut client_endpoint = client_endpoint_build_result.unwrap();


    //Connecting with Quinn Client Endpoint to the server
    let server_connection_result = client_endpoint.connect(
        state.get_quinn_server_addres().parse().unwrap(),
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
    if received_envelope.get_type() != NetworkBandwidthDTO::get_data_type() {
        //TODO: Write appropriate error returning
        return HttpResponse::InternalServerError().finish();
    }


    let received_chart = NetworkBandwidthDTO::decode(received_envelope.get_data());
    let chart = NetworkBandwidth::from(received_chart);
    
    HttpResponse::Ok().json(chart)
}
