use std::collections::HashMap;

use actix_web::web;

use crate::core::app_state::AppState;
use crate::core::chart_management::chart_request_manager::ChartRequestManagaer;
use crate::core::client_data::ClientData;
use crate::core::general_filters::GeneralFilters;

pub struct DashboardManager {
    chart_requesters: HashMap<&'static str, Box<dyn ChartRequestManagaer>>,
}

impl DashboardManager {
    pub fn new(
        chart_requesters: HashMap<&'static str, Box<dyn ChartRequestManagaer>>
    ) -> Self {
        Self { 
            chart_requesters
        }
    }

    pub fn request_dashboard(
        &self,
        state: web::Data<AppState>,
        client_data: web::Query<ClientData>,
        params: web::Query<GeneralFilters>
    ) {
        todo!()
    }
}