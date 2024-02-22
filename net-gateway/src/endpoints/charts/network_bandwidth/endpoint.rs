use std::sync::Arc;

use actix_web::get;
use actix_web::web;
use actix_web::Responder;
use actix_web::HttpResponse;
use actix_web::HttpRequest;

use crate::authorization::Authorization;
use crate::authorization::mock_authenticator::MockAuthenticator;

use crate::core::app_state::AppState;
use crate::core::chart_management::chart_request_manager::ChartRequestManagaer;
use crate::core::client_data::ClientData;
use crate::core::filter::FiltersWrapper;
use crate::core::general_filters::GeneralFilters;

use crate::endpoints::charts::network_bandwidth::request::manager::NetworkBandwidthChartManager;

#[get("/chart/network_bandwidth")]
async fn get_network_bandwidth(
    state: web::Data<AppState>,
    client_data: web::Query<ClientData>,
    params: web::Query<GeneralFilters>,
    filters_wrapper: web::Query<FiltersWrapper>,
    req: HttpRequest,
) -> impl Responder {
    //Auth stuff
    if let Err(response) = Authorization::authorize(req, MockAuthenticator {}).await {
        return response;
    }
    
    let chart_request_result = NetworkBandwidthChartManager::default().request_chart(
        state.into_inner(),
        Arc::new(client_data.into_inner()),
        Arc::new(params.into_inner()),
        Arc::new(filters_wrapper.into_inner().into()),
    ).await;
    if let Err(e) = chart_request_result {
        //TODO: Write appropriate error returning
        return HttpResponse::InternalServerError().body(e);
    }
    let chart = chart_request_result.unwrap();
    
    HttpResponse::Ok().json(chart.get_json_value())
}
