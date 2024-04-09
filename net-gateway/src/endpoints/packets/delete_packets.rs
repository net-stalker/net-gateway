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
    let _token = if config.verify_token.verify {
        match authorization::authorize(req, FusionAuthVerifier::new(&config.fusion_auth_server_address.addr, Some(config.fusion_auth_api_key.key.clone()))).await {
            Ok(token) => token,
            Err(response) => return response,
        }
    } else {
        config.verify_token.default_token.clone()
    };
    // all the packets here must have onlt id and other field are None
    log::debug!("Packets to delete: {:?}", packets);
    HttpResponse::Ok().body("Packets deleted successfully!")
}
