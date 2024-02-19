use crate::core::chart_request_manager::ChartRequestManagaer;

use super::chart_requester::NetworkBandwidthChartRequester;
use super::request_former::NetworkBandwidthRequestFormer;

pub struct NetworkBandwidthChartManager {}

#[async_trait::async_trait]
impl ChartRequestManagaer for NetworkBandwidthChartManager {
    type RequestCreator = NetworkBandwidthRequestFormer;
    type Requester = NetworkBandwidthChartRequester;
}