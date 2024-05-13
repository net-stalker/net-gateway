use actix_multipart::Multipart;
use actix_web::delete;
use actix_web::web;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;
use futures::StreamExt;
use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::api::result::result::ResultDTO;
use net_core_api::core::typed_api::Typed;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::decoder_api::Decoder;
use net_deleter_api::api::packets::DeletePacketsRequestDTO;
use net_token_verifier::fusion_auth::fusion_auth_verifier::FusionAuthVerifier;
use crate::core::quinn_client_endpoint_manager::QuinnClientEndpointManager;
use crate::authorization;
use crate::config::Config;


#[delete("/packets")]
async fn packets(
    config: web::Data<Config>,
    req: HttpRequest,
    mut payload: Multipart
) -> impl Responder {
    //Auth stuff
    let token_verifier = FusionAuthVerifier::new(
        &config.fusion_auth_server_address.addr,
        Some(config.fusion_auth_api_key.key.clone())
    );

    let authorization_result = authorization::authorize(
        req,
        Box::new(token_verifier)
    ).await;

    if let Err(e) = authorization_result {
        return e;
    }
    let token = authorization_result.unwrap();

    let tenant_id = token.get_tenant_id();
    if let Err(e) = tenant_id {
        return HttpResponse::InternalServerError().body(e.to_string());
    }
    let tenant_id = tenant_id.unwrap();
    let mut packets_ids = Vec::new();

    while let Some(item) = payload.next().await {
        let mut field = item.unwrap();
        if field.name() == "packets" {
            let mut data = Vec::new();
            while let Some(chunk) = field.next().await {
                data.extend_from_slice(&chunk.unwrap());
            }
            let packet_ids: Vec<String> = serde_json::from_slice(&data).unwrap();
            packets_ids.extend(packet_ids.into_iter().collect::<Vec<String>>());
        }
    }

    let delete_packet_request = DeletePacketsRequestDTO::new(&packets_ids);
    let request = Envelope::new(tenant_id, delete_packet_request.get_type(), &delete_packet_request.encode());

    let server_connection_result = QuinnClientEndpointManager::start_server_connection(
        &config.quin_client_address.addr,
        &config.quin_inserter.addr,
        &config.quin_server_application.app,
    ).await;
    let mut server_connection = match server_connection_result {
        Ok(server_connection) => server_connection,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };
    match server_connection.send_all_reliable(&request.encode()).await {
        Ok(_) => (),
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };
    // the request has been sent, now I need to retrieve the response back
    let response = match server_connection.receive_reliable().await {
        Ok(response) => ResultDTO::decode(Envelope::decode(&response).get_data()),
        Err(err) => return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Invernal Server Error",
            "message": err.to_string()
        })),
    };
    match response.is_ok() {
        true => HttpResponse::Ok().json(serde_json::json!("The packets deleted successfully")),
        false => {
            if response.get_description().is_err() {
                return HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Invernal Server Error",
                    "message": format!("Something went wrong during deleting the packets"),
                }));
            }
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal Server Error",
                "message": response.get_description().unwrap(),
            }))
        },
    }
}
