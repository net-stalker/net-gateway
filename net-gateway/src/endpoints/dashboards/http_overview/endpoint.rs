use std::sync::Arc;

use actix_web::get;
use actix_web::web;
use actix_web::Responder;
use actix_web::HttpResponse;
use actix_web::HttpRequest;
use net_token_verifier::fusion_auth::fusion_auth_verifier::FusionAuthVerifier;

use crate::authorization;

use crate::config::Config;
use crate::core::filter::Filters;
use crate::core::filter::FiltersWrapper;
use crate::core::dashboard_management::dashboard_manager::DashboardManager;
use crate::core::general_filters::GeneralFilters;

use crate::endpoints::charts::http_clients::request::manager::HttpClientsChartManager;
use crate::endpoints::charts::http_request_methods_distribution::request::manager::HttpRequestMethodsDistChartManager;
use crate::endpoints::charts::http_responses::request::manager::HttpResponsesChartManager;
use crate::endpoints::charts::http_responses_distribution::request::manager::HttpResponsesDistributionChartManager;
use crate::endpoints::charts::total_http_requests::request::manager::TotalHttpRequestsChartManager;
use crate::endpoints::filters::http_overview_filters::request::manager::HttpOverviewFilterManager;


#[get("/dashboard/http_overview")]
async fn get_http_overview(
    config: web::Data<Config>,
    params: web::Query<GeneralFilters>,
    filters_wrapper: web::Query<FiltersWrapper>,
    req: HttpRequest,
) -> impl Responder {
    //Auth stuff
    let token = match authorization::authorize(req,FusionAuthVerifier::new(&config.fusion_auth_server_addres.addr, Some(config.fusion_auth_api_key.key.clone()))).await {
        Ok(token) => token,
        Err(response) => return response,
    };

    let filters: Filters = filters_wrapper.into_inner().into();

    let dashboard_request_result = DashboardManager::builder()
        .add_data_requester(HttpClientsChartManager::default().boxed())
        .add_data_requester(HttpRequestMethodsDistChartManager::default().boxed())
        .add_data_requester(HttpResponsesChartManager::default().boxed())
        .add_data_requester(HttpResponsesDistributionChartManager::default().boxed())
        .add_data_requester(TotalHttpRequestsChartManager::default().boxed())
        .add_data_requester(HttpOverviewFilterManager::default().boxed())
        .build()
        .request_dashboard(
            config.into_inner(),
            Arc::new(token),
            Arc::new(params.into_inner()),
            Some(Arc::new(filters)),
        ).await;

    if let Err(e) = dashboard_request_result {
        //TODO: Write appropriate error returning
        return HttpResponse::InternalServerError().body(e);
    }

    let requested_dashboard = dashboard_request_result.unwrap();
    
    HttpResponse::Ok().json(requested_dashboard)
}