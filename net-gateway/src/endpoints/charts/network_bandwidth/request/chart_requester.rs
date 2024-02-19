use net_reporter_api::api::network_bandwidth::network_bandwidth::NetworkBandwidthDTO;

use crate::core::chart_management::chart_requester::ChartRequester;
use crate::endpoints::charts::network_bandwidth::response::network_bandwidth::NetworkBandwidthResponse;

pub struct NetworkBandwidthChartRequester {}

#[async_trait::async_trait]
impl ChartRequester for NetworkBandwidthChartRequester {
    type ResponseDTO = NetworkBandwidthDTO;
    type Response = NetworkBandwidthResponse;
}