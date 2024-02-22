use serde::Serialize;
use serde::ser::SerializeMap;

use crate::core::chart_management::chart_response::ChartResponse;

use super::dashboard_builder::DashboardBuilder;

#[derive(Debug)]
pub struct Dashboard {
    charts: Vec<Box<dyn ChartResponse>>,
}

impl Dashboard {
    pub fn new(
        charts: Vec<Box<dyn ChartResponse>>
    ) -> Self {
        Self { 
            charts
        }
    }

    pub fn builder() -> DashboardBuilder {
        DashboardBuilder::default()
    }
}

impl Serialize for Dashboard {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        let mut state = serializer.serialize_map(None)?;
        
        for chart in self.charts.iter() {
            state.serialize_entry(chart.get_json_type(), &chart.get_json_value())?;
        }
        
        state.end()
    }
}