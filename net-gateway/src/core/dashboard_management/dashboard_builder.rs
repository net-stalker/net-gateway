use crate::core::service_request_management::service_response::ServiceResponse;

use super::dashboard::Dashboard;

#[derive(Default)]
pub struct DashboardBuilder {
    charts: Vec<Box<dyn ServiceResponse>>,
}

impl DashboardBuilder {
    pub fn add_chart (
        mut self,
        chart: Box<dyn ServiceResponse>
    ) -> Self {
        //TODO: Create Error handling here
        self.charts.push(chart);
        self
    }

    pub fn add_charts (
        mut self,
        charts: &mut Vec<Box<dyn ServiceResponse>>
    ) -> Self {
        //TODO: Create Error handling here
        self.charts.append(charts);
        self
    }

    pub fn build (self) -> Dashboard {
        Dashboard::new(self.charts)
    }
}