use actix_web::HttpResponse;
use actix_web::HttpRequest;
use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::api::result::result::ResultDTO;
use net_core_api::core::typed_api::Typed;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::decoder_api::Decoder;
use net_reporter_api::api::network::networks_request::NetworksRequestDTO;
use net_reporter_api::api::network::networks_response::NetworksResponseDTO;
use net_token_verifier::fusion_auth::fusion_auth_verifier::FusionAuthVerifier;

use crate::authorization;
use crate::config::Config;
use crate::core::quinn_client_endpoint_manager::QuinnClientEndpointManager;
use crate::core::user_facing_error::UserFacingError;
use crate::endpoints::networks::core::network::Network;

pub async fn get_networks_handler(config: &Config, req: HttpRequest, network_id: Option<String>) -> Result<HttpResponse, UserFacingError> {
    //Auth stuff
    let token_verifier = FusionAuthVerifier::new(
        &config.fusion_auth_server_address.addr,
        Some(config.fusion_auth_api_key.key.clone())
    );

    let authorization_result = authorization::authorize(
        req,
        Box::new(token_verifier)
    ).await;

    if let Err(_) = authorization_result {
        return Err(UserFacingError::Unauthorized);
    }
    let token = authorization_result.unwrap();

    let tenant_id = token.get_tenant_id();
    if let Err(err) = tenant_id {
        return Err(UserFacingError::InternalErrorWithDescription(err.to_string()));
    }
    let tenant_id = tenant_id.unwrap();
    let get_networks_request = match network_id {
        Some(network_id) => NetworksRequestDTO::new(&[network_id; 1]),
        None => NetworksRequestDTO::default(), 
    };
    let request = Envelope::new(
        tenant_id,
        get_networks_request.get_type(),
        &get_networks_request.encode(),
    );
    let server_connection_result = QuinnClientEndpointManager::start_server_connection(
        &config.quin_client_address.addr,
        &config.quin_inserter.addr,
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
            let description = response.get_description().unwrap_or("didn't get any data").to_string();
            let response = response.into_inner();
            if response.is_none() { return Err(UserFacingError::InternalErrorWithDescription(description)) }
            let response = response.unwrap();
            match response.get_envelope_type() == NetworksResponseDTO::get_data_type() {
                true => Ok(HttpResponse::Ok().json(NetworksResponseDTO::decode(response.get_data()).get_networks().iter().map(|network| network.clone().into()).collect::<Vec<Network>>())),
                false => Err(UserFacingError::InternalErrorWithDescription("Wrong data type has been requested".to_string()))
            }
            
        },
        false => {
            match response.get_description() {
                Ok(desc) => Err(UserFacingError::InternalErrorWithDescription(desc.to_string())),
                Err(_) => Err(UserFacingError::InternalError)
            }
        },
    }
}
