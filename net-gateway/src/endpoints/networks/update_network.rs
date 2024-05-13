use actix_multipart::Multipart;
use actix_web::patch;
use actix_web::web;
use actix_web::HttpRequest;
use futures::StreamExt;
use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::api::result::result::ResultDTO;
use net_token_verifier::fusion_auth::fusion_auth_verifier::FusionAuthVerifier;
use net_updater_api::api::updaters::update_network::update_network_request::UpdateNetworkRequestDTO;
use net_core_api::core::typed_api::Typed;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::decoder_api::Decoder;
use crate::core::quinn_client_endpoint_manager::QuinnClientEndpointManager;
use crate::core::user_facing_error::UserFacingError;
use crate::endpoints::networks::core::network::Network;
use crate::{authorization, config::Config};


#[patch("/network")]
async fn network(
    config: web::Data<Config>,
    req: HttpRequest,
    mut payload: Multipart,
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

    if let Err(_) = authorization_result { return Err(UserFacingError::Unauthorized); }

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

    let network: Option<Network> = if let Some(item) = payload.next().await {
        let mut network = None;
        let mut field = item.unwrap();
        if field.name() == "network" {
            let mut data = Vec::new();
            while let Some(chunk) = field.next().await {
                data.extend_from_slice(&chunk.unwrap());
            }
            network = Some(serde_json::from_slice(&data).unwrap())
        }
        network
    } else {
        None
    };

    if network.is_none() { return Err(UserFacingError::InternalErrorWithDescription("Wrong network data has been sent".to_string())) }
    let network = network.unwrap();
    
    let update_network_request = UpdateNetworkRequestDTO::new(
        &match network.id {
            Some(network_id) => network_id,
            None => return Err(UserFacingError::InternalErrorWithDescription("Network with correct id is expected".to_string()))
        },
        &network.name,
        &network.color,
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

    match response.is_ok() {
        true => Ok("Network has been updated successfully"),
        false => Err(UserFacingError::InternalError),
    }
}
