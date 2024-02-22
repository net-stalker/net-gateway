use std::sync::Arc;

use actix_web::get;
use actix_web::web;
use actix_web::Responder;
use actix_web::HttpResponse;
use actix_web::HttpRequest;

use net_core_api::decoder_api::Decoder;
use net_core_api::encoder_api::Encoder;
use net_core_api::envelope::envelope::Envelope;
use net_core_api::typed_api::Typed;

use net_reporter_api::api::network_bandwidth::network_bandwidth::NetworkBandwidthDTO;
use net_reporter_api::api::network_bandwidth::network_bandwidth_filters::NetworkBandwidthFiltersDTO;
use net_reporter_api::api::network_bandwidth::network_bandwidth_request::NetworkBandwidthRequestDTO;

use crate::authorization::Authorization;
use crate::authorization::mock_authenticator::MockAuthenticator;

use crate::core::app_state::AppState;
use crate::core::chart_management::chart_request_manager::ChartRequestManagaer;
use crate::core::client_data::ClientData;
use crate::core::filter::Filters;
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
    let filters: NetworkBandwidthFiltersDTO = <FiltersWrapper as Into<Filters>>::into(filters_wrapper.into_inner()).into();
    
    let chart_request_result = NetworkBandwidthChartManager::default().request_chart(
        Arc::new(state),
        Arc::new(client_data),
        Arc::new(params),
        Arc::new(filters),
    ).await;
    if let Err(e) = chart_request_result {
        //TODO: Write appropriate error returning
        return HttpResponse::InternalServerError().body(e);
    }
    let chart = chart_request_result.unwrap();
    
    HttpResponse::Ok().json(chart.get_json_value())
}
