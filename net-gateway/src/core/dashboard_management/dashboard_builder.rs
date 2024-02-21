use crate::core::chart_management::chart_response::ChartResponse;

use super::dashboard::Dashboard;

#[derive(Default)]
pub struct DashboardBuilder {
    charts: Vec<Box<dyn ChartResponse>>,
}

impl DashboardBuilder {
    pub fn add_chart (
        mut self,
        chart: Box<dyn ChartResponse>
    ) -> Self {
        //TODO: Create Error handling here
        let _ = self.charts.push(chart);
        self
    }

    pub fn add_charts (
        mut self,
        charts: &mut Vec<Box<dyn ChartResponse>>
    ) -> Self {
        //TODO: Create Error handling here
        let _ = self.charts.append(charts);
        self
    }

    pub fn build (self) -> Dashboard {
        Dashboard::new(self.charts)
    }
}