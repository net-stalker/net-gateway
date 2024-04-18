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
    log::debug!("getting networks");
    HttpResponse::Ok().body("Networks retrieved successfully")
}
