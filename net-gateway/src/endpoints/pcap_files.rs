use actix_web::post;
use actix_web::web;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_multipart::Multipart;
use futures::StreamExt;
use futures::TryStreamExt;
use net_agent_api::api::data_packet::DataPacketDTO;
use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::core::typed_api::Typed;
use net_core_api::core::encoder_api::Encoder;
use net_token_verifier::fusion_auth::fusion_auth_verifier::FusionAuthVerifier;

use crate::core::quinn_client_endpoint_manager::QuinnClientEndpointManager;
use crate::{authorization, config::Config};

#[post("/pcap_files")]
async fn pcap_files(
    config: web::Data<Config>,
    req: HttpRequest,
    mut payload: Multipart
) -> impl Responder {
    //Auth stuff
    let token = if config.verify_token.token {
        match authorization::authorize(req, FusionAuthVerifier::new(&config.fusion_auth_server_address.addr, Some(config.fusion_auth_api_key.key.clone()))).await {
            Ok(token) => token,
            Err(response) => return response,
        }
    } else {
        config.verify_token.default_token.clone()
    };
    while let Ok(Some(mut field)) = payload.try_next().await {
        // read the whole pcap file in bytes
        let mut file_bytes = web::BytesMut::new();
        while let Some(chunk) = field.next().await {
            let chunk = chunk.unwrap();
            file_bytes.extend_from_slice(&chunk);
        }
        let server_connection_result = QuinnClientEndpointManager::start_server_connection(
            &config.quin_client_address.addr,
            &config.quin_inserter.addr,
            &config.quin_server_application.app,
        ).await;
        let mut server_connection = match server_connection_result {
            Ok(server_connection) => server_connection,
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
        };
        let packet_data = DataPacketDTO::new(&file_bytes); 
        let request = Envelope::new(Some(&token), Some("agent_id"), DataPacketDTO::get_data_type(), &packet_data.encode());
        match server_connection.send_all_reliable(&request.encode()).await {
            Ok(_) => (),
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
        };
    }

    HttpResponse::Ok().body("Files uploaded successfully")
}
