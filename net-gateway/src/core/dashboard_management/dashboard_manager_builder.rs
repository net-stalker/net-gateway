use std::collections::HashMap;

// use crate::core::chart_management::chart_request_manager::ChartResponse;
use crate::core::chart_management::chart_request_manager::ChartRequestManagaer;

use super::dashboard_manager::DashboardManager;

#[derive(Default)]
pub struct DashboardManagerBuilder {
    chart_requesters: HashMap<&'static str, Box<dyn ChartRequestManagaer>>,
}

impl DashboardManagerBuilder {
    pub fn add_chart_requester (
        mut self,
        chart_requester: Box<dyn ChartRequestManagaer>
    ) -> Self {
        //TODO: Create Error handling here
        let _ = self.chart_requesters.insert(chart_requester.get_request_type(), chart_requester);
        self
    }

    pub fn build (self) -> DashboardManager {
        DashboardManager::new(self.chart_requesters)
    }
}