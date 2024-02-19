use std::collections::HashMap;
use std::sync::Arc;

use actix_web::web;
use tokio::sync::Mutex;

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

    pub async fn request_dashboard(
        self,
        state: Arc<web::Data<AppState>>,
        client_data: Arc<web::Query<ClientData>>,
        params: Arc<web::Query<GeneralFilters>>
    ) -> Result<HashMap<&'static str, serde_json::Value>, String> {
        let response: Arc<Mutex<HashMap<&'static str, serde_json::Value>>> = Arc::new(Mutex::new(HashMap::new()));

        for (requesting_type, chart_requester) in self.chart_requesters {
            let request_result = chart_requester.request_chart(
                state.clone(),
                client_data.clone(),
                params.clone()
            ).await;

            let requested_chart = request_result?;
            
            response.lock().await.insert(
                requesting_type,
                requested_chart
            );
        }

        Ok(
            Arc::try_unwrap(response)
                .unwrap()
                .into_inner()
        )
    }
}