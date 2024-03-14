use actix_web::post;
use actix_web::web;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_multipart::Multipart;
use futures::StreamExt;
use futures::TryStreamExt;
use net_token_verifier::fusion_auth::fusion_auth_verifier::FusionAuthVerifier;

use crate::core::quinn_client_endpoint_manager::QuinnClientEndpointManager;
use crate::{authorization, config::Config};

#[post("/pcap_files")]
async fn pcap_files(
    config: web::Data<Config>,
    req: HttpRequest,
    mut payload: Multipart
) -> impl Responder {
    let _token = match authorization::authorize(req, FusionAuthVerifier::new(&config.fusion_auth_server_addres.addr, Some(config.fusion_auth_api_key.key.clone()))).await {
        Ok(token) => token,
        Err(response) => return response,
    }; 
    while let Ok(Some(mut field)) = payload.try_next().await {
        // read the whole pcap file in bytes
        let mut file_bytes = web::BytesMut::new();
        while let Some(chunk) = field.next().await {
            let chunk = chunk.unwrap();
            file_bytes.extend_from_slice(&chunk);
        }
        // then we need to transfer the file to the net-tranlator
        //Creating Quinn Client Endpoint
        //Connecting with Quinn Client Endpoint to the server
        let server_connection_result = QuinnClientEndpointManager::start_server_connection(
            &config.quin_client_addres.addr,
            &config.quin_inserter_addres.addr,
            &config.quin_server_application.app,
        ).await;
        let mut server_connection = match server_connection_result {
            Ok(server_connection) => server_connection,
            Err(e) => return HttpResponse::InternalServerError().body(e),
        };
        //Sending out data (request) to the server
        match server_connection.send_all_reliable(&file_bytes).await {
            Ok(_) => (),
            Err(e) => return HttpResponse::InternalServerError().body(e),
        };
    }

    HttpResponse::Ok().body("Files uploaded successfully")
}
