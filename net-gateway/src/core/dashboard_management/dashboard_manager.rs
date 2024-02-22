use std::sync::Arc;

use actix_web::web;
use futures::future::try_join_all;
use tokio::sync::Mutex;

use crate::core::app_state::AppState;
use crate::core::chart_management::chart_request_manager::ChartRequestManagaer;
use crate::core::chart_management::chart_response::ChartResponse;
use crate::core::client_data::ClientData;
use crate::core::general_filters::GeneralFilters;

use super::dashboard::Dashboard;
use super::dashboard_manager_builder::DashboardManagerBuilder;

pub struct DashboardManager {
    chart_requesters: Vec<Box<dyn ChartRequestManagaer>>,
}

impl DashboardManager {
    pub fn new(
        chart_requesters: Vec<Box<dyn ChartRequestManagaer>>
    ) -> Self {
        Self { 
            chart_requesters
        }
    }

    pub fn builder() -> DashboardManagerBuilder {
        DashboardManagerBuilder::default()
    }

    pub async fn request_dashboard(
        self,
        state: Arc<web::Data<AppState>>,
        client_data: Arc<web::Query<ClientData>>,
        params: Arc<web::Query<GeneralFilters>>
    ) -> Result<Dashboard, String> {
        let charts_request_result = self.request_charts(
            state,
            client_data,
            params
        ).await;

        let mut requested_charts = charts_request_result?;

        Ok(
            Dashboard::builder()
                .add_charts(&mut requested_charts)
                .build()
        )
    } 

    async fn request_charts(
        self,
        state: Arc<web::Data<AppState>>,
        client_data: Arc<web::Query<ClientData>>,
        params: Arc<web::Query<GeneralFilters>>
    ) -> Result<Vec<Box<dyn ChartResponse>>, String> {
        let response: Arc<Mutex<Vec<Box<dyn ChartResponse>>>> = Arc::new(Mutex::new(Vec::new()));

        let mut tasks = Vec::new();

        for chart_requester in self.chart_requesters {
            let response_clone = response.clone();
            
            let state_clone = state.clone();
            let client_data_clone = client_data.clone();
            let params_clone = params.clone();
            
            let task = tokio::spawn(async move {
                let request_result = chart_requester.request_chart(
                    state_clone,
                    client_data_clone,
                    params_clone,
                ).await;

                //TODO: Add Error propper handling
                if let Err(e) = request_result {
                    return Err(e);
                }
                let requested_chart = request_result.unwrap();
                
                response_clone.lock().await.push(
                    requested_chart
                );
                
                return Ok(());
            });

            tasks.push(task);
        }

        let task_result = try_join_all(tasks).await;

        if let Err(e) = task_result {
            return Err(format!("error: {:?}", e));
        }

        Ok(
            Arc::try_unwrap(response)
                .unwrap()
                .into_inner()
        )
    }
}