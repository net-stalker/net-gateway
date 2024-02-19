use crate::core::chart_management::chart_request_manager::ChartResponse;
use crate::core::chart_management::chart_request_manager::ChartRequestManagaer;

use super::dashboard_manager::DashboardManager;

#[derive(Default)]
pub struct DashboardManagerBuilder {
    chart_requesters: Vec<Box<dyn ChartRequestManagaer<dyn ChartResponse>>>,
}

impl DashboardManagerBuilder {
    pub fn add_chart_requester (
        mut self,
        chart_requester: Box<dyn ChartRequestManagaer<dyn ChartResponse>>
    ) -> Self {
        self.chart_requesters.push(chart_requester);
        self
    }

    pub fn build (self) -> DashboardManager {
        DashboardManager::new(self.chart_requesters)
    }
}