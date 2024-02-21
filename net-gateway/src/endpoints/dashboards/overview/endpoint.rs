use std::sync::Arc;

use actix_web::get;
use actix_web::web;
use actix_web::Responder;
use actix_web::HttpResponse;
use actix_web::HttpRequest;

use crate::authorization::Authorization;
use crate::authorization::mock_authenticator::MockAuthenticator;

use crate::core::app_state::AppState;
use crate::core::client_data::ClientData;
use crate::core::dashboard_management::dashboard_manager::DashboardManager;
use crate::core::general_filters::GeneralFilters;

use crate::endpoints::charts::network_bandwidth::request::manager::NetworkBandwidthChartManager;
use crate::endpoints::charts::network_bandwidth_per_endpoint::request::manager::NetworkBandwidthPerEndpointChartManager;
use crate::endpoints::charts::network_graph::request::manager::NetworkGraphChartManager;


#[get("/dashboard/overview")]
async fn get_overview(
    state: web::Data<AppState>,
    client_data: web::Query<ClientData>,
    params: web::Query<GeneralFilters>,
    req: HttpRequest,
) -> impl Responder {
    //Auth stuff
    if let Err(response) = Authorization::authorize(req, MockAuthenticator {}).await {
        return response;
    }

    let dashboard_request_result = DashboardManager::builder()
        .add_chart_requester(NetworkBandwidthChartManager::default().boxed())
        .add_chart_requester(NetworkBandwidthPerEndpointChartManager::default().boxed())
        .add_chart_requester(NetworkGraphChartManager::default().boxed())
        .build()
        .request_dashboard(
            Arc::new(state),
            Arc::new(client_data),
            Arc::new(params)
        ).await;

    if let Err(e) = dashboard_request_result {
        //TODO: Write appropriate error returning
        return HttpResponse::InternalServerError().body(e);
    }

    let requested_dashboard = dashboard_request_result.unwrap();
    
    HttpResponse::Ok().json(requested_dashboard)
}