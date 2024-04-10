use actix_web::post;
use actix_web::web;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;
use net_token_verifier::fusion_auth::fusion_auth_verifier::FusionAuthVerifier;
use crate::endpoints::networks::core::network::Network;
use crate::{authorization, config::Config};


#[post("/network")]
async fn network(
    config: web::Data<Config>,
    req: HttpRequest,
    network: web::Json<Network>,
) -> impl Responder {
    //Auth stuff
    let _token = if config.verify_token.verify {
        match authorization::authorize(req, FusionAuthVerifier::new(&config.fusion_auth_server_address.addr, Some(config.fusion_auth_api_key.key.clone()))).await {
            Ok(token) => token,
            Err(response) => return response,
        }
    } else {
        config.verify_token.default_token.clone()
    };
    log::debug!("Network to add: {:?}", network);
    // TODO: need to update those new endpoins to actually have access to the rest of the backend
    HttpResponse::Ok().body("Network uploaded successfully")
}
