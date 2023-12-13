use actix_web::{HttpResponse, Responder, get, web};
use async_tungstenite::tungstenite::Message;
use net_proto_api::{typed_api::Typed, encoder_api::Encoder, envelope::envelope::Envelope};
use net_timescale_api::api::{dashboard::dashboard_request::DashboardRequestDTO, network_graph::network_graph_request::NetworkGraphRequestDTO, bandwidth_per_endpoint::{bandwidth_per_endpoint::BandwidthPerEndpointDTO, bandwidth_per_endpoint_request::BandwidthPerEndpointRequestDTO}, network_bandwidth::network_bandwidth_request::NetworkBandwidthRequestDTO};
use serde::{Deserialize, Serialize};
use validator::Validate;
use futures::prelude::*;
use crate::app_state::AppState;
use async_tungstenite::tokio;


#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct ClientData {
    pub group_id: String
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct OverviewFilters {
    pub start_date: i64,
    pub end_date: i64,
}

#[get("/dashboard/overview")]
async fn overview(state: web::Data<AppState>, client_data: web::Query<ClientData>, params: web::Query<OverviewFilters>) -> impl Responder {
    log::debug!("client data: {:?}", client_data.0);
    log::debug!("params: {:?}", params.0);
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
    HttpResponse::Ok().json(params.into_inner())
}
