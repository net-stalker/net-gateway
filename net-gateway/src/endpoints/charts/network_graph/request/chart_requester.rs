use net_reporter_api::api::network_graph::network_graph::NetworkGraphDTO;

use crate::core::chart_management::chart_requester::ChartRequester;
use crate::endpoints::charts::network_graph::response::network_graph::NetworkGraphResponse;

pub struct NetworkGraphChartRequester {}

#[async_trait::async_trait]
impl ChartRequester for NetworkGraphChartRequester {
    type ResponseDTO = NetworkGraphDTO;
    type Response = NetworkGraphResponse;
}