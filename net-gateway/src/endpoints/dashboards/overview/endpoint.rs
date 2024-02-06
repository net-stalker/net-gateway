use std::sync::Arc;

use actix_web::get;
use actix_web::web;
use actix_web::Responder;
use actix_web::HttpResponse;
use actix_web::HttpRequest;

use net_proto_api::decoder_api::Decoder;
use net_proto_api::encoder_api::Encoder;
use net_proto_api::envelope::envelope::Envelope;
use net_proto_api::typed_api::Typed;

use net_timescale_api::api::bandwidth_per_endpoint::bandwidth_per_endpoint::BandwidthPerEndpointDTO;
use net_timescale_api::api::bandwidth_per_endpoint::bandwidth_per_endpoint_request::BandwidthPerEndpointRequestDTO;
use net_timescale_api::api::dashboard::dashboard::DashboardDTO;
use net_timescale_api::api::network_bandwidth::network_bandwidth::NetworkBandwidthDTO;
use net_timescale_api::api::network_bandwidth::network_bandwidth_request::NetworkBandwidthRequestDTO;
use net_timescale_api::api::network_graph::network_graph::NetworkGraphDTO;
use net_timescale_api::api::network_graph::network_graph_request::NetworkGraphRequestDTO;
use net_transport::quinn::client::builder::ClientQuicConnectorBuilder;
use tokio::sync::Mutex;

use crate::authorization::Authorization;
use crate::authorization::mock_authenticator::MockAuthenticator;
use crate::app_state::AppState;
use crate::client_data::ClientData;
use crate::endpoints::dashboards::overview::dashboard::OverviewDashboard;
use crate::general_filters::GeneralFilters;


#[get("/dashboard/overview")]
async fn get_overview(
    state: web::Data<AppState>,
    client_data: web::Query<ClientData>,
    params: web::Query<GeneralFilters>,
    req: HttpRequest,
) -> impl Responder {
    //Auth stuff
    if let Err(response) = Authorization::authorize(req, MockAuthenticator {}).await {
        return response;
    }

    let enveloped_chart_responses: Arc<Mutex<Vec<Envelope>>> = Default::default();

    let enveloped_chart_responses_clone = enveloped_chart_responses.clone();
    let state_clone = state.clone();
    let client_data_clone = client_data.clone();
    let params_clone = params.clone();
    let bandwidth_per_endpoint_task = tokio::spawn(async move {    
        //Form request to the server
        let bandwidth_per_endpoint_request = BandwidthPerEndpointRequestDTO::new(
            params_clone.start_date,
            params_clone.end_date
        );
        let enveloped_bandwidth_per_endpoint_request = Envelope::new(
            Some(client_data_clone.group_id.as_str()),
            None,
            BandwidthPerEndpointRequestDTO::get_data_type(),
            &bandwidth_per_endpoint_request.encode()
        );
        let bytes_to_send = enveloped_bandwidth_per_endpoint_request.encode();
    
    
        //Creating Quinn Client Endpoint
        let client_endpoint_build_result = ClientQuicConnectorBuilder::default()
            .with_addr(state_clone.get_quinn_client_addres().parse().unwrap())
            .build();
        if client_endpoint_build_result.is_err() {
            //TODO: Write appropriate error returning
            return;
        }
        let mut client_endpoint = client_endpoint_build_result.unwrap();
    
    
        //Connecting with Quinn Client Endpoint to the server
        let server_connection_result = client_endpoint.connect(
            state_clone.get_quinn_server_addres().parse().unwrap(),
            state_clone.get_quinn_server_application()
        ).await;
        if server_connection_result.is_err() {
            //TODO: Write appropriate error returning
            return;
        }
        let mut server_connection = server_connection_result.unwrap();
    
    
        //Sending out data (request) to the server
        let sending_result = server_connection.send_all_reliable(&bytes_to_send).await;
        if sending_result.is_err() {
            //TODO: Write appropriate error returning
            return;
        }
    
    
        //Waiting on new data and reading message from the server
        let receiving_result = server_connection.receive_reliable().await;
        if sending_result.is_err() {
            //TODO: Write appropriate error returning
            return;
        }
        let received_bytes = receiving_result.unwrap();
    
        let received_envelope = Envelope::decode(&received_bytes);

    
        //TODO: Think about letting it all sit here. Maybe this checking is not necessary
        if received_envelope.get_type() != BandwidthPerEndpointDTO::get_data_type() {
            //TODO: Write appropriate error returning
            return;
        }

        enveloped_chart_responses_clone.lock().await.push(received_envelope);
    });

    let enveloped_chart_responses_clone = enveloped_chart_responses.clone();
    let state_clone = state.clone();
    let client_data_clone = client_data.clone();
    let params_clone = params.clone();
    let network_bandwidth_task = tokio::spawn(async move {
        //Form request to the server
        let network_bandwidth_request = NetworkBandwidthRequestDTO::new(
            params_clone.start_date,
            params_clone.end_date
        );
        let enveloped_network_bandwidth_request = Envelope::new(
            Some(client_data_clone.group_id.as_str()),
            None,
            NetworkBandwidthRequestDTO::get_data_type(),
            &network_bandwidth_request.encode()
        );
        let bytes_to_send = enveloped_network_bandwidth_request.encode();


        //Creating Quinn Client Endpoint
        let client_endpoint_build_result = ClientQuicConnectorBuilder::default()
            .with_addr(state_clone.get_quinn_client_addres().parse().unwrap())
            .build();
        if client_endpoint_build_result.is_err() {
            //TODO: Write appropriate error returning
            return;
        }
        let mut client_endpoint = client_endpoint_build_result.unwrap();


        //Connecting with Quinn Client Endpoint to the server
        let server_connection_result = client_endpoint.connect(
            state_clone.get_quinn_server_addres().parse().unwrap(),
            state_clone.get_quinn_server_application()
        ).await;
        if server_connection_result.is_err() {
            //TODO: Write appropriate error returning
            return;
        }
        let mut server_connection = server_connection_result.unwrap();


        //Sending out data (request) to the server
        let sending_result = server_connection.send_all_reliable(&bytes_to_send).await;
        if sending_result.is_err() {
            //TODO: Write appropriate error returning
            return;
        }


        //Waiting on new data and reading message from the server
        let receiving_result = server_connection.receive_reliable().await;
        if sending_result.is_err() {
            //TODO: Write appropriate error returning
            return;
        }
        let received_bytes = receiving_result.unwrap();

        let received_envelope = Envelope::decode(&received_bytes);


        //TODO: Think about letting it all sit here. Maybe this checking is not necessary
        if received_envelope.get_type() != NetworkBandwidthDTO::get_data_type() {
            //TODO: Write appropriate error returning
            return;
        }

        enveloped_chart_responses_clone.lock().await.push(received_envelope);
    });

    let enveloped_chart_responses_clone = enveloped_chart_responses.clone();
    let state_clone = state.clone();
    let client_data_clone = client_data.clone();
    let params_clone = params.clone();
    let network_graph_task = tokio::spawn(async move {
        //Form request to the server
        let network_graph_request = NetworkGraphRequestDTO::new(
            params_clone.start_date,
            params_clone.end_date,
            false
        );
        let enveloped_network_graph_request = Envelope::new(
            Some(client_data_clone.group_id.as_str()),
            None,
            NetworkGraphRequestDTO::get_data_type(),
            &network_graph_request.encode()
        );
        let bytes_to_send = enveloped_network_graph_request.encode();


        //Creating Quinn Client Endpoint
        let client_endpoint_build_result = ClientQuicConnectorBuilder::default()
            .with_addr(state_clone.get_quinn_client_addres().parse().unwrap())
            .build();
        if client_endpoint_build_result.is_err() {
            //TODO: Write appropriate error returning
            return;
        }
        let mut client_endpoint = client_endpoint_build_result.unwrap();


        //Connecting with Quinn Client Endpoint to the server
        let server_connection_result = client_endpoint.connect(
            state_clone.get_quinn_server_addres().parse().unwrap(),
            state_clone.get_quinn_server_application()
        ).await;
        if server_connection_result.is_err() {
            //TODO: Write appropriate error returning
            return;
        }
        let mut server_connection = server_connection_result.unwrap();


        //Sending out data (request) to the server
        let sending_result = server_connection.send_all_reliable(&bytes_to_send).await;
        if sending_result.is_err() {
            //TODO: Write appropriate error returning
            return;
        }


        //Waiting on new data and reading message from the server
        let receiving_result = server_connection.receive_reliable().await;
        if sending_result.is_err() {
            //TODO: Write appropriate error returning
            return;
        }
        let received_bytes = receiving_result.unwrap();

        let received_envelope = Envelope::decode(&received_bytes);


        //TODO: Think about letting it all sit here. Maybe this checking is not necessary
        if received_envelope.get_type() != NetworkGraphDTO::get_data_type() {
            //TODO: Write appropriate error returning
            return;
        }
        enveloped_chart_responses_clone.lock().await.push(received_envelope);
    });

    let _ = tokio::join!(
        bandwidth_per_endpoint_task,
        network_bandwidth_task,
        network_graph_task
    );

    let dashboard_dto = DashboardDTO::new(
        &Arc::try_unwrap(enveloped_chart_responses)
            .unwrap()
            .into_inner()
    );

    let dashpoard = OverviewDashboard::from(dashboard_dto);
    
    HttpResponse::Ok().json(dashpoard)
}