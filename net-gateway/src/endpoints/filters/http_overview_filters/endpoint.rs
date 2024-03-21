use std::sync::Arc;

use actix_web::get;
use actix_web::web;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;
use net_token_verifier::fusion_auth::fusion_auth_verifier::FusionAuthVerifier;

use crate::authorization;

use crate::config::Config;
use crate::core::general_filters::GeneralFilters;
use crate::core::service_request_management::service_request_manager::ServiceRequestManager;
use crate::endpoints::filters::http_overview_filters::request::manager::HttpOverviewFilterManager;

#[get("/filter/http_overview")]
async fn get_http_overview_filters(
    config: web::Data<Config>,
    params: web::Query<GeneralFilters>,
    req: HttpRequest,
) -> impl Responder {
    //Auth stuff
    let token = match authorization::authorize(req, FusionAuthVerifier::new(&config.fusion_auth_server_addres.addr, Some(config.fusion_auth_api_key.key.clone()))).await {
        Ok(token) => token,
        Err(response) => return response,
    };

    let filters_request_result = HttpOverviewFilterManager::default().request_data(
        config.into_inner(),
        Arc::new(token),
        Arc::new(params.into_inner()),
        None,
    ).await;
    if let Err(e) = filters_request_result {
        //TODO: Write appropriate error returning
        return HttpResponse::InternalServerError().body(e.to_string());
    }
    let chart = filters_request_result.unwrap();
    
    HttpResponse::Ok().json(chart.get_json_value())
}
