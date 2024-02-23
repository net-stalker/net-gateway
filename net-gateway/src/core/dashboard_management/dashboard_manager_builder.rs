use crate::core::service_request_management::service_request_manager::ServiceRequestManager;

use super::dashboard_manager::DashboardManager;

#[derive(Default)]
pub struct DashboardManagerBuilder {
    chart_requesters: Vec<Box<dyn ServiceRequestManager>>,
}

impl DashboardManagerBuilder {
    pub fn add_chart_requester (
        mut self,
        chart_requester: Box<dyn ServiceRequestManager>
    ) -> Self {
        //TODO: Create Error handling here
        self.chart_requesters.push(chart_requester);
        self
    }

    pub fn build (self) -> DashboardManager {
        DashboardManager::new(self.chart_requesters)
    }
}