use crate::core::service_request_management::service_request_manager::ServiceRequestManager;

use super::dashboard_manager::DashboardManager;

#[derive(Default)]
pub struct DashboardManagerBuilder {
    data_requesters: Vec<Box<dyn ServiceRequestManager>>,
}

impl DashboardManagerBuilder {
    pub fn add_data_requester (
        mut self,
        data_requester: Box<dyn ServiceRequestManager>
    ) -> Self {
        //TODO: Create Error handling here
        self.data_requesters.push(data_requester);
        self
    }

    pub fn build (self) -> DashboardManager {
        DashboardManager::new(self.data_requesters)
    }
}