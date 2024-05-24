use actix_web::patch;
use actix_web::web;
use actix_web::HttpRequest;
use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::api::primitives::integer::Integer;
use net_core_api::api::result::result::ResultDTO;
use net_token_verifier::fusion_auth::fusion_auth_verifier::FusionAuthVerifier;
use net_updater_api::api::updaters::update_network::update_network_request::UpdateNetworkRequestDTO;
use net_core_api::core::typed_api::Typed;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::decoder_api::Decoder;
use serde::Deserialize;
use crate::core::quinn_client_endpoint_manager::QuinnClientEndpointManager;
use crate::core::refresh_request_management::refresh_manager::RefreshManager;
use crate::core::user_facing_error::UserFacingError;
use crate::authorization;
use crate::config::Config;

#[derive(Debug, Deserialize)]
struct RequestJson {
    pub name: String,
    pub color: String,
}

#[patch("/networks/{id}")]
async fn network(
    id: web::Path<String>,
    config: web::Data<Config>,
    req: HttpRequest,
    json: web::Json<RequestJson>,
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
   
    let update_network_request = UpdateNetworkRequestDTO::new(
        &id,
        &json.name,
        &json.color,
    );

    let request = Envelope::new(tenant_id, update_network_request.get_type(), &update_network_request.encode());

    match server_connection.send_all_reliable(&request.encode()).await {
        Ok(_) => (),
        Err(_) => return Err(UserFacingError::Timeout),
    };

    let response = match server_connection.receive_reliable().await {
        Ok(response) => ResultDTO::decode(Envelope::decode(&response).get_data()),
        Err(err) => return Err(UserFacingError::InternalErrorWithDescription(err.to_string())),
    };

    if !response.is_ok() {
        return Err(UserFacingError::InternalErrorWithDescription(response.get_description().unwrap_or("no description").to_string()));
    }
    let updated_rows_count = Integer::decode(response.into_inner().unwrap().get_data());
    match RefreshManager::new(&config, tenant_id).refresh(&updated_rows_count).await {
        Ok(_) => Ok("Network has been updated successfully"),
        Err(err) => Err(UserFacingError::InternalErrorWithDescription(err.to_string())),
    }
}
