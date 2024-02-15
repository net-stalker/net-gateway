use net_reporter_api::api::network_bandwidth_per_endpoint::network_bandwidth_per_endpoint::NetworkBandwidthPerEndpointDTO;

use crate::core::chart_requester::ChartRequester;
use crate::endpoints::charts::network_bandwidth_per_endpoint::response::network_bandwidth_per_endpoint::NetworkBandwidthPerEndpointResponse;

pub struct NetworkBandwidthPerEndpointChartRequester {}

#[async_trait::async_trait]
impl ChartRequester for NetworkBandwidthPerEndpointChartRequester {
    type ResponseDTO = NetworkBandwidthPerEndpointDTO;
    type Response = NetworkBandwidthPerEndpointResponse;
}