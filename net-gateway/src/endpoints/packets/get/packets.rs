use actix_web::get;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web::HttpRequest;
use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::api::result::result::ResultDTO;
use net_core_api::core::typed_api::Typed;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::decoder_api::Decoder;
use net_reporter_api::api::network_packet::network_packets::NetworkPacketsDTO;
use net_reporter_api::api::network_packet::network_packets_request::NetworkPacketsRequestDTO;
use net_token_verifier::fusion_auth::fusion_auth_verifier::FusionAuthVerifier;
use serde::Deserialize;
use crate::authorization;
use crate::config::Config;
use crate::core::quinn_client_endpoint_manager::QuinnClientEndpointManager;
use crate::core::user_facing_error::UserFacingError;
use crate::endpoints::packets::get::response::packets::NetworkPackets;

#[derive(Debug, Deserialize)]
struct RequestQuery {
    #[serde(rename = "networkIds")]
    network_ids: String,
}

#[get("/packets")]
async fn packets(
    config: web::Data<Config>,
    req: HttpRequest,
    json: web::Query<RequestQuery>,
) -> Result<HttpResponse, UserFacingError> {
    //Auth stuff
    let token_verifier = FusionAuthVerifier::new(
        &config.fusion_auth_server_address.addr,
        Some(config.fusion_auth_api_key.key.clone())
    );

    let authorization_result = authorization::authorize(
        req,
        Box::new(token_verifier)
    ).await;

    if authorization_result.is_err() { return Err(UserFacingError::Unauthorized); }
    let token = authorization_result.unwrap();

    let tenant_id = token.get_tenant_id();
    if let Err(err) = tenant_id {
        return Err(UserFacingError::InternalErrorWithDescription(err.to_string()));
    }
    let tenant_id = tenant_id.unwrap();
    let network_ids = json.network_ids.split(',').map(|id| {
        match id {
            "null" => None,
            _ => Some(id),
        }
    }).collect::<Vec<Option<&str>>>();
    let network_packet_request = NetworkPacketsRequestDTO::new(&network_ids); 
    let request = Envelope::new(
        tenant_id,
        network_packet_request.get_type(),
        &network_packet_request.encode(),
    );
    let server_connection_result = QuinnClientEndpointManager::start_server_connection(
        &config.quin_client_address.addr,
        &config.quin_reporter.addr,
        &config.quin_server_application.app,
    ).await;
    let mut server_connection = match server_connection_result {
        Ok(server_connection) => server_connection,
        Err(err) => return Err(UserFacingError::InternalErrorWithDescription(err.to_string())),
    };

    match server_connection.send_all_reliable(&request.encode()).await {
        Ok(_) => (),
        Err(err) => return Err(UserFacingError::InternalErrorWithDescription(err.to_string())),
    };

    let response = match server_connection.receive_reliable().await {
        Ok(response) => ResultDTO::decode(Envelope::decode(&response).get_data()),
        Err(err) => return Err(UserFacingError::InternalErrorWithDescription(err.to_string())),
    };

    match response.is_ok() {
        true => {
            let network_packet: NetworkPackets = NetworkPacketsDTO::decode(
                response
                    .into_inner()
                    .unwrap()
                    .get_data()
                ).into(); 
            Ok(HttpResponse::Ok().json(network_packet))
        },
        false => {
            match response.get_description() {
                Ok(desc) => Err(UserFacingError::InternalErrorWithDescription(desc.to_string())),
                Err(_) => Err(UserFacingError::InternalError)
            }
        },
    }
}
