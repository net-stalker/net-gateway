use actix_web::{HttpResponse, Responder, get, web};
use async_tungstenite::tungstenite::Message;
use net_proto_api::decoder_api::Decoder;
use net_proto_api::{typed_api::Typed, encoder_api::Encoder, envelope::envelope::Envelope};
use net_timescale_api::api::dashboard::dashboard::DashboardDTO;
use net_timescale_api::api::dashboard::dashboard_request::DashboardRequestDTO;
use net_timescale_api::api::network_graph::network_graph_request::NetworkGraphRequestDTO;
use net_timescale_api::api::bandwidth_per_endpoint::bandwidth_per_endpoint_request::BandwidthPerEndpointRequestDTO;
use net_timescale_api::api::network_bandwidth::network_bandwidth_request::NetworkBandwidthRequestDTO;
use futures::prelude::*;
use crate::app_state::AppState;
use crate::client_data::ClientData;
use crate::endpoints::dashboards::overview::dashboard::OverviewDashboard;
use crate::general_filters::GeneralFilters;
use async_tungstenite::tokio;

#[get("/dashboard/overview")]
async fn get_overview(state: web::Data<AppState>, client_data: web::Query<ClientData>, params: web::Query<GeneralFilters>) -> impl Responder {
    log::info!("Received request for overview dashboard, params: {:?}", params);
    log::info!("Client data: {:?}", client_data);
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
    if wrapped.get_type() != DashboardDTO::get_data_type() {
        return HttpResponse::ExpectationFailed().json("Failed to read")
    }
    let dashboard = DashboardDTO::decode(wrapped.get_data());
    HttpResponse::Ok().json(OverviewDashboard::from(dashboard))

}
