use actix_web::{HttpResponse, Responder, get, web};
use async_tungstenite::tungstenite::Message;
use net_proto_api::decoder_api::Decoder;
use net_proto_api::{typed_api::Typed, encoder_api::Encoder, envelope::envelope::Envelope};
use net_timescale_api::api::network_bandwidth::network_bandwidth::NetworkBandwidthDTO;
use net_timescale_api::api::network_bandwidth::network_bandwidth_request::NetworkBandwidthRequestDTO;
use futures::prelude::*;
use crate::app_state::AppState;
use crate::client_data::ClientData;
use crate::endpoints::charts::network_bandwidth::chart::NetworkBandwidth;
use async_tungstenite::tokio;
use crate::general_filters::GeneralFilters;


#[get("/chart/network_bandwidth")]
async fn get_network_bandwidth(state: web::Data<AppState>, client_data: web::Query<ClientData>, params: web::Query<GeneralFilters>) -> impl Responder {
    let (mut consumer, _) = tokio::connect_async(state.get_ws_url())
        .await
        .expect("Failed to connect");
    let nt_request = NetworkBandwidthRequestDTO::new(params.start_date, params.end_date).encode();
    let envelope = Envelope::new(Some(client_data.group_id.as_str()), None, NetworkBandwidthRequestDTO::get_data_type(), nt_request.as_slice());
    consumer.send(Message::Binary(envelope.encode())).await.expect("Failed to write");
    // TODO: move this boilerplate to a macros
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
    if wrapped.get_type() != NetworkBandwidthDTO::get_data_type() {
        return HttpResponse::ExpectationFailed().json("Failed to read")
    }
    let chart = NetworkBandwidthDTO::decode(wrapped.get_data());
    HttpResponse::Ok().json(NetworkBandwidth::from(chart))
}
