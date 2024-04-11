use actix_web::delete;
use actix_web::web;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;
use net_token_verifier::fusion_auth::fusion_auth_verifier::FusionAuthVerifier;
use crate::endpoints::packets::core::packet::Packet;
use crate::{authorization, config::Config};


#[delete("/packets")]
async fn packets(
    config: web::Data<Config>,
    req: HttpRequest,
    packets: web::Json<Vec<Packet>>,
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
    // all the packets here must have onlt id and other field are None
    log::debug!("Packets to delete: {:?}", packets);
    HttpResponse::Ok().body("Packets deleted successfully!")
}
