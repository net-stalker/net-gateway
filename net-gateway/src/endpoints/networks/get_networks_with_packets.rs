use actix_web::get;
use actix_web::web;
use actix_web::Responder;
use actix_web::HttpResponse;
use actix_web::HttpRequest;
use net_token_verifier::fusion_auth::fusion_auth_verifier::FusionAuthVerifier;

use crate::authorization;
use crate::config::Config;

#[get("/networks-with-packets")]
async fn networks_with_packets(
    config: web::Data<Config>,
    req: HttpRequest,
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
    log::debug!("getting networks");
    HttpResponse::Ok().body("Networks retrieved successfully")
}
