use actix_web::get;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web::HttpRequest;
use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::api::result::result::ResultDTO;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::typed_api::Typed;
use net_reporter_api::api::network::network_id::NetworkIdDTO;
use net_reporter_api::api::network::network_id_request::NetworkIdRequestDTO;
use net_token_verifier::fusion_auth::fusion_auth_verifier::FusionAuthVerifier;
use serde::Deserialize;
use crate::authorization;
use crate::config::Config;
use crate::core::quinn_client_endpoint_manager::QuinnClientEndpointManager;
use crate::core::user_facing_error::UserFacingError;

#[derive(Debug, Deserialize)]
struct RequestQuery {
    pub name: String,
    pub color: String,
}

#[get("/networks/search")]
async fn search_network(
    config: web::Data<Config>,
    req: HttpRequest,
    query: web::Query<RequestQuery>,
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
    let network_id_request = NetworkIdRequestDTO::new(
        &query.name,
        &query.color,
    );
    let request = Envelope::new(
        tenant_id,
        network_id_request.get_type(),
        &network_id_request.encode(),
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
            let response = NetworkIdDTO::decode(response.into_inner().unwrap().get_data());
            Ok(HttpResponse::Ok().json(response.get_id()))
        },
        false => Err(UserFacingError::InternalErrorWithDescription(response.get_description().unwrap().to_string())),
    }
}
