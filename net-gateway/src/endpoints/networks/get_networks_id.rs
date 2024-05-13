use actix_web::get;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web::HttpRequest;
use crate::config::Config;
use crate::core::user_facing_error::UserFacingError;
use crate::endpoints::networks::handlers::get_networks_handler::get_networks_handler;

#[get("/networks/{id}")]
async fn networks(
    network_id: web::Path<String>,
    config: web::Data<Config>,
    req: HttpRequest,
) -> Result<HttpResponse, UserFacingError> {
    get_networks_handler(config.into_inner().as_ref(), req, Some(network_id.into_inner())).await
}
