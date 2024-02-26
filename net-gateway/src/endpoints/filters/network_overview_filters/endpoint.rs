use std::sync::Arc;

use actix_web::get;
use actix_web::web;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;

use crate::authorization::mock_authenticator::MockAuthenticator;
use crate::authorization::Authorization;
use crate::core::app_state::AppState;
use crate::core::client_data::ClientData;
use crate::core::general_filters::GeneralFilters;
use crate::core::service_request_management::service_request_manager::ServiceRequestManager;
use crate::endpoints::filters::network_overview_filters::request::manager::NetworkOverviewFilterManager;

#[get("/filter/network_overview")]
async fn get_network_overview_filters(
    state: web::Data<AppState>,
    client_data: web::Query<ClientData>,
    params: web::Query<GeneralFilters>,
    req: HttpRequest,
) -> impl Responder {
    //Auth stuff
    if let Err(response) = Authorization::authorize(req, MockAuthenticator {}).await {
        // TODO: make sense to move authorization to net-core as well
        return response;
    }
    // TODO: implement FilterManager
    let chart_request_result = NetworkOverviewFilterManager::default().request_data(
        state.into_inner(),
        Arc::new(client_data.into_inner()),
        Arc::new(params.into_inner()),
        None,
    ).await;
    if let Err(e) = chart_request_result {
        //TODO: Write appropriate error returning
        return HttpResponse::InternalServerError().body(e);
    }
    let chart = chart_request_result.unwrap();
    
    HttpResponse::Ok().json(chart.get_json_value())
}
