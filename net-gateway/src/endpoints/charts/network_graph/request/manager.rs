use crate::core::chart_request_manager::ChartRequestManagaer;

use super::chart_requester::NetworkGraphChartRequester;
use super::request_former::NetworkGraphRequestFormer;

pub struct NetworkGraphChartManager {}

#[async_trait::async_trait]
impl ChartRequestManagaer for NetworkGraphChartManager {
    type RequestFormer = NetworkGraphRequestFormer;
    type Requester = NetworkGraphChartRequester;
}