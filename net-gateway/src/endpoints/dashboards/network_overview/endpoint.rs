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
use crate::core::filter::Filters;
use crate::core::filter::FiltersWrapper;
use crate::core::dashboard_management::dashboard_manager::DashboardManager;
use crate::core::general_filters::GeneralFilters;

use crate::endpoints::charts::network_bandwidth::request::manager::NetworkBandwidthChartManager;
use crate::endpoints::charts::network_bandwidth_per_endpoint::request::manager::NetworkBandwidthPerEndpointChartManager;
use crate::endpoints::charts::network_bandwidth_per_protocol::request::manager::NetworkBandwidthPerProtocolChartManager;
use crate::endpoints::charts::network_graph::request::manager::NetworkGraphChartManager;
use crate::endpoints::filters::network_overview_filters::request::manager::NetworkOverviewFilterManager;


#[get("/dashboard/network_overview")]
async fn get_network_overview(
    state: web::Data<Config>,
    client_data: web::Query<ClientData>,
    params: web::Query<GeneralFilters>,
    filters_wrapper: web::Query<FiltersWrapper>,
    req: HttpRequest,
) -> impl Responder {
    //Auth stuff
    if let Err(response) = authorization::authorize(
        req,
        FusionAuthVerifier::new(&state.fusion_auth_server_addres.addr, Some(state.fusion_auth_api_key.key.clone()))).await {
        return response;
    }

    let filters: Filters = filters_wrapper.into_inner().into();

    let dashboard_request_result = DashboardManager::builder()
        .add_data_requester(NetworkBandwidthChartManager::default().boxed())
        .add_data_requester(NetworkBandwidthPerEndpointChartManager::default().boxed())
        .add_data_requester(NetworkBandwidthPerProtocolChartManager::default().boxed())
        .add_data_requester(NetworkGraphChartManager::default().boxed())
        .add_data_requester(NetworkOverviewFilterManager::default().boxed())
        .build()
        .request_dashboard(
            state.into_inner(),
            Arc::new(client_data.into_inner()),
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