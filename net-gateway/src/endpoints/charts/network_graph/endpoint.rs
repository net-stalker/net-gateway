use actix_web::{HttpResponse, Responder, get, web, HttpRequest};
use async_tungstenite::tungstenite::Message;
use net_proto_api::decoder_api::Decoder;
use net_proto_api::{typed_api::Typed, encoder_api::Encoder, envelope::envelope::Envelope};
use net_timescale_api::api::network_graph::network_graph::NetworkGraphDTO;
use net_timescale_api::api::network_graph::network_graph_request::NetworkGraphRequestDTO;
use futures::prelude::*;
use crate::app_state::AppState;
use crate::authorization::Authorization;
use crate::authorization::mock_authenticator::MockAuthenticator;
use crate::client_data::ClientData;
use crate::endpoints::charts::network_graph::chart::NetworkGraph;
use async_tungstenite::tokio;
use crate::general_filters::GeneralFilters;


#[get("/chart/network_graph")]
async fn get_network_graph(
    state: web::Data<AppState>,
    client_data: web::Query<ClientData>,
    params: web::Query<GeneralFilters>,
    req: HttpRequest,
) -> impl Responder {
    if let Err(response) = Authorization::authorize(req, MockAuthenticator {}).await {
        return response;
    }
    let (mut consumer, _) = tokio::connect_async(state.get_ws_url())
        .await
        .expect("Failed to connect");
    let nt_request = NetworkGraphRequestDTO::new(params.start_date, params.end_date, false).encode();
    let envelope = Envelope::new(Some(client_data.group_id.as_str()), None, NetworkGraphRequestDTO::get_data_type(), nt_request.as_slice());
    // TODO: move this boilerplate to a macros
    consumer.send(Message::Binary(envelope.encode())).await.expect("Failed to write");
    let response = consumer.next().await;
    let response = match response {
        Some(Ok(response)) => response,
        Some(Err(err)) => {
            return HttpResponse::ExpectationFailed().json(serde_json::json!({
                "error": err.to_string()
            }));
        },
        None => {
            return HttpResponse::ExpectationFailed().json("Failed to read");
        }
    };
    let wrapped = match response {
        Message::Binary(data) => Envelope::decode(data.as_slice()),
        _ => {
            return HttpResponse::ExpectationFailed().json("Failed to read")
        }
    };
    if wrapped.get_type() != NetworkGraphDTO::get_data_type() {
        return HttpResponse::ExpectationFailed().json("Failed to read")
    }
    let graph = NetworkGraphDTO::decode(wrapped.get_data());
    HttpResponse::Ok().json(NetworkGraph::from(graph))
}
