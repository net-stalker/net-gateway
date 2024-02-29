use std::sync::Arc;

use actix_web::get;
use actix_web::web;
use actix_web::Responder;
use actix_web::HttpResponse;
use actix_web::HttpRequest;
use net_token_verifier::fusion_auth::fusion_auth_verifier::FusionAuthVerifier;

use crate::authorization;

use crate::config::Config;
use crate::core::client_data::ClientData;
use crate::core::filter::FiltersWrapper;
use crate::core::general_filters::GeneralFilters;

use crate::core::service_request_management::service_request_manager::ServiceRequestManager;
use crate::endpoints::charts::network_graph::request::manager::NetworkGraphChartManager;

#[get("/chart/network_graph")]
async fn get_network_graph(
    config: web::Data<Config>,
    client_data: web::Query<ClientData>,
    params: web::Query<GeneralFilters>,
    filters_wrapper: web::Query<FiltersWrapper>,
    req: HttpRequest,
) -> impl Responder {
    //Auth stuff
    if let Err(response) = authorization::authorize(
        req,
        FusionAuthVerifier::new(&config.fusion_auth_server_addres.addr, Some(config.fusion_auth_api_key.key.clone()))).await {
        return response;
    }
    let chart_request_result = NetworkGraphChartManager::default().request_data(
        config.into_inner(),
        Arc::new(client_data.into_inner()),
        Arc::new(params.into_inner()),
        Some(Arc::new(filters_wrapper.into_inner().into())),
    ).await;
    if let Err(e) = chart_request_result {
        //TODO: Write appropriate error returning
        return HttpResponse::InternalServerError().body(e);
    }
    let chart = chart_request_result.unwrap();
    
    HttpResponse::Ok().json(chart.get_json_value())
}
