use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use net_core_api::{decoder_api::Decoder, encoder_api::Encoder, envelope::envelope::Envelope, typed_api::Typed};
use net_reporter_api::api::network_overview_dashboard_filters::{network_overview_dashboard_filters_request::NetworkOverviewDashboardFiltersRequestDTO, network_overview_dashbord_filters::NetworkOverviewDashboardFiltersDTO};
use net_transport::quinn::client::builder::ClientQuicConnectorBuilder;

use crate::{app_state::AppState, authorization::{mock_authenticator::MockAuthenticator, Authorization}, client_data::ClientData, endpoints::filters::network_overview_filters::filters::NetworkOverviewFilters, filters::Filters, general_filters::GeneralFilters};

#[get("/filter/network_overview")]
async fn get_network_overview_filters(
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
    let network_overview_filters_request = NetworkOverviewDashboardFiltersRequestDTO::new(
        params.start_date,
        params.end_date,
    );
    let enveloped_network_graph_request = Envelope::new(
        Some(client_data.group_id.as_str()),
        None,
        NetworkOverviewDashboardFiltersRequestDTO::get_data_type(),
        &network_overview_filters_request.encode()
    );
    let bytes_to_send = enveloped_network_graph_request.encode();


    //Creating Quinn Client Endpoint
    let client_endpoint_build_result = ClientQuicConnectorBuilder::default()
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
    if received_envelope.get_type() != NetworkOverviewDashboardFiltersDTO::get_data_type() {
        //TODO: Write appropriate error returning
        return HttpResponse::InternalServerError().finish();
    }


    let received_filters = NetworkOverviewDashboardFiltersDTO::decode(received_envelope.get_data());
    let filters = NetworkOverviewFilters::from(received_filters);
    // return jsons
    /*  
    entries: [
        {
            endpoint: "test_endpoint",
            protocols: ["test_protocol"],
            totalBytes: 0
        }
    ]
    */
    HttpResponse::Ok().json(filters)
}
