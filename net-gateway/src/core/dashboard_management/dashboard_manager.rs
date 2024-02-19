use crate::core::chart_management::chart_request_manager::ChartResponse;
use crate::core::chart_management::chart_request_manager::ChartRequestManagaer;

pub struct DashboardManager {
    chart_requesters: Vec<Box<dyn ChartRequestManagaer<dyn ChartResponse>>>,
}

impl DashboardManager {
    pub fn new(
        chart_requesters: Vec<Box<dyn ChartRequestManagaer<dyn ChartResponse>>>
    ) -> Self {
        Self { 
            chart_requesters
        }
    }
}