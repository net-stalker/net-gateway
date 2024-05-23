use actix_web::patch;
use actix_web::web;
use actix_web::HttpRequest;
use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::api::result::result::ResultDTO;
use net_core_api::core::typed_api::Typed;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::encoder_api::Encoder;
use net_token_verifier::fusion_auth::fusion_auth_verifier::FusionAuthVerifier;
use net_updater_api::api::updaters::update_packets_network_id::update_packets_network_id_request::UpdatePacketsNetworkIdRequestDTO;
use serde::Deserialize;
use crate::core::quinn_client_endpoint_manager::QuinnClientEndpointManager;
use crate::core::user_facing_error::UserFacingError;
use crate::authorization;
use crate::config::Config;


#[derive(Debug, Deserialize)]
struct RequestBody {
    #[serde(rename = "networkId")]
    network_id: Option<String>,
}


#[patch("/packets/{id}")]
async fn update_packets_network_id(
    config: web::Data<Config>,
    req: HttpRequest,
    id: web::Query<String>,
    body: web::Json<RequestBody>,
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
    
    let server_connection_result = QuinnClientEndpointManager::start_server_connection(
        &config.quin_client_address.addr,
        &config.quin_inserter.addr,
        &config.quin_server_application.app,
    ).await;
    let mut server_connection = match server_connection_result {
        Ok(server_connection) => server_connection,
        Err(_) => return Err(UserFacingError::Timeout),
    };

    let packets_ids = vec![id.into_inner()];
    let network_id = body.network_id.as_deref();

    let transfer_packets_request = UpdatePacketsNetworkIdRequestDTO::new(network_id, packets_ids.as_slice());

    let request = Envelope::new(
        tenant_id,
        transfer_packets_request.get_type(),
        &transfer_packets_request.encode()
    );

    match server_connection.send_all_reliable(&request.encode()).await {
        Ok(_) => (),
        Err(_) => return Err(UserFacingError::Timeout),
    };

    let response = match server_connection.receive_reliable().await {
        Ok(response) => ResultDTO::decode(Envelope::decode(&response).get_data()),
        Err(err) => return Err(UserFacingError::InternalErrorWithDescription(err.to_string())),
    };

    match response.is_ok() {
        true => Ok("Packet has been updated successfully"),
        false => Err(UserFacingError::InternalError),
    }
}
