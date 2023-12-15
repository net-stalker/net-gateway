use actix_web::{HttpResponse, Responder, get, web};
use async_tungstenite::tungstenite::Message;
use net_proto_api::{typed_api::Typed, encoder_api::Encoder, envelope::envelope::Envelope};
use net_timescale_api::api::bandwidth_per_endpoint::bandwidth_per_endpoint_request::BandwidthPerEndpointRequestDTO;
use futures::prelude::*;
use crate::app_state::AppState;
use crate::client_data::ClientData;
use async_tungstenite::tokio;
use crate::general_filters::GeneralFilters;


#[get("/chart/bandwidth_per_endpoint")]
async fn get_bandwidth_per_endpoint(state: web::Data<AppState>, client_data: web::Query<ClientData>, params: web::Query<GeneralFilters>) -> impl Responder {
    let (mut consumer, _) = tokio::connect_async(state.get_ws_url())
        .await
        .expect("Failed to connect");
    let nt_request = BandwidthPerEndpointRequestDTO::new(params.start_date, params.end_date).encode();
    let envelope = Envelope::new(Some(client_data.group_id.as_str()), None, BandwidthPerEndpointRequestDTO::get_data_type(), nt_request.as_slice());
    // TODO: move this boilerplate to a macros
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
        _ => {
            HttpResponse::ExpectationFailed().json("Failed to read")
        }
    }
}
