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
    let _tenant_id = tenant_id.unwrap();
    log::debug!("Network to add: {:?}", network);
    // TODO: need to update those new endpoins to actually have access to the rest of the backend
    HttpResponse::Ok().body("Network uploaded successfully")
}
