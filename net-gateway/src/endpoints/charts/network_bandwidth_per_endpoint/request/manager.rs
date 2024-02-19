use crate::core::chart_management::chart_request_manager::ChartRequestManagaer;

use super::chart_requester::NetworkBandwidthPerEndpointChartRequester;
use super::request_former::NetworkBandwidthPerEndpointRequestFormer;

pub struct NetworkBandwidthPerEndpointChartManager {}

#[async_trait::async_trait]
impl ChartRequestManagaer for NetworkBandwidthPerEndpointChartManager {
    type RequestCreator = NetworkBandwidthPerEndpointRequestFormer;
    type Requester = NetworkBandwidthPerEndpointChartRequester;
}