use actix_web::delete;
use actix_web::web;
use actix_web::HttpRequest;
use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::api::primitives::integer::Integer;
use net_core_api::api::result::result::ResultDTO;
use net_core_api::core::typed_api::Typed;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::decoder_api::Decoder;
use net_deleter_api::api::packets::DeletePacketsRequestDTO;
use net_token_verifier::fusion_auth::fusion_auth_verifier::FusionAuthVerifier;
use serde::Deserialize;
use crate::core::quinn_client_endpoint_manager::QuinnClientEndpointManager;
use crate::authorization;
use crate::config::Config;
use crate::core::refresh_request_management::refresh_manager::RefreshManager;
use crate::core::user_facing_error::UserFacingError;

#[derive(Debug, Deserialize)]
struct RequestBody {
    #[serde(rename = "packetIds")]
    packet_ids: Vec<String>,
}


#[delete("/packets")]
async fn delete_multiple_packets(
    config: web::Data<Config>,
    req: HttpRequest,
    json: web::Json<RequestBody>,
) -> Result<&'static str, UserFacingError> {
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
    if let Err(e) = tenant_id {
        return Err(UserFacingError::InternalErrorWithDescription(e.to_string()));
    }
    let tenant_id = tenant_id.unwrap();

    let delete_packet_request = DeletePacketsRequestDTO::new(&json.packet_ids);
    let request = Envelope::new(tenant_id, delete_packet_request.get_type(), &delete_packet_request.encode());

    let server_connection_result = QuinnClientEndpointManager::start_server_connection(
        &config.quin_client_address.addr,
        &config.quin_inserter.addr,
        &config.quin_server_application.app,
    ).await;

    let mut server_connection = match server_connection_result {
        Ok(server_connection) => server_connection,
        Err(_) => return Err(UserFacingError::Timeout),
    };

    match server_connection.send_all_reliable(&request.encode()).await {
        Ok(_) => (),
        Err(_) => return Err(UserFacingError::Timeout),
    };

    let response = match server_connection.receive_reliable().await {
        Ok(response) => ResultDTO::decode(Envelope::decode(&response).get_data()),
        Err(_) => return Err(UserFacingError::InternalError),
    };

    if !response.is_ok() {
        return Err(UserFacingError::InternalErrorWithDescription(response.get_description().unwrap_or("no description").to_string()));
    }
    let updated_rows_count = Integer::decode(response.into_inner().unwrap().get_data());
    match RefreshManager::new(&config, tenant_id).refresh(&updated_rows_count).await {
        Ok(_) => Ok("The packets have been deleted successfully"),
        Err(err) => Err(UserFacingError::InternalErrorWithDescription(err.to_string())),
    }
}
