use actix_web::{HttpResponse, Responder, get, web};
use async_tungstenite::tungstenite::Message;
use net_proto_api::{typed_api::Typed, encoder_api::Encoder, envelope::envelope::Envelope};
use net_timescale_api::api::dashboard::dashboard_request::DashboardRequestDTO;
use net_timescale_api::api::network_graph::network_graph_request::NetworkGraphRequestDTO;
use net_timescale_api::api::bandwidth_per_endpoint::bandwidth_per_endpoint_request::BandwidthPerEndpointRequestDTO;
use net_timescale_api::api::network_bandwidth::network_bandwidth_request::NetworkBandwidthRequestDTO;
use futures::prelude::*;
use crate::app_state::AppState;
use crate::client_data::ClientData;
use crate::general_filters::GeneralFilters;
use async_tungstenite::tokio;

#[get("/dashboard/overview")]
async fn get_overview(state: web::Data<AppState>, client_data: web::Query<ClientData>, params: web::Query<GeneralFilters>) -> impl Responder {
    let (mut consumer, _) = tokio::connect_async(state.get_ws_url())
        .await
        .expect("Failed to connect");
    let nt_request = NetworkGraphRequestDTO::new(params.start_date, params.end_date, false);
    let bpe_request = BandwidthPerEndpointRequestDTO::new(params.start_date, params.end_date);
    let nb_request = NetworkBandwidthRequestDTO::new(params.start_date, params.end_date);
    let dashboard_request = DashboardRequestDTO::new(vec![
        Envelope::new(Some(client_data.group_id.as_str()), None, NetworkGraphRequestDTO::get_data_type(), nt_request.encode().as_slice()),
        Envelope::new(Some(client_data.group_id.as_str()), None, BandwidthPerEndpointRequestDTO::get_data_type(), bpe_request.encode().as_slice()),
        Envelope::new(Some(client_data.group_id.as_str()), None, NetworkBandwidthRequestDTO::get_data_type(), nb_request.encode().as_slice()),
    ].as_slice()).encode();
    let envelope = Envelope::new(Some(client_data.group_id.as_str()), None, DashboardRequestDTO::get_data_type(), dashboard_request.as_slice());
    consumer.send(Message::Binary(envelope.encode())).await.expect("Failed to write");
    let response = consumer.next().await.unwrap();
    let response = match response {
        Ok(response) => response,
        Err(err) => {
            return HttpResponse::ExpectationFailed().json(serde_json::json!({
                "error": err.to_string()
            }));
        }
    };
    match response {
        Message::Binary(data) => {
            HttpResponse::Ok().json(data)
        },
        Message::Text(data) => {
            HttpResponse::Ok().json(data)
        },
        _ => {
            HttpResponse::ExpectationFailed().json("Failed to read")
        }
    }

}
