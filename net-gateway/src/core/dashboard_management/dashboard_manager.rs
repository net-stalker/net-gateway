use std::sync::Arc;

use futures::future::try_join_all;
use tokio::sync::Mutex;

use crate::config::Config;
use crate::core::service_request_management::service_request_manager::ServiceRequestManager;
use crate::core::service_request_management::service_response::ServiceResponse;
use crate::core::client_data::ClientData;
use crate::core::filter::Filters;
use crate::core::general_filters::GeneralFilters;

use super::dashboard::Dashboard;
use super::dashboard_manager_builder::DashboardManagerBuilder;

pub struct DashboardManager {
    data_requesters: Vec<Box<dyn ServiceRequestManager>>,
}

impl DashboardManager {
    pub fn new(
        data_requesters: Vec<Box<dyn ServiceRequestManager>>
    ) -> Self {
        Self { 
            data_requesters
        }
    }

    pub fn builder() -> DashboardManagerBuilder {
        DashboardManagerBuilder::default()
    }

    pub async fn request_dashboard(
        self,
        config: Arc<Config>,
        client_data: Arc<ClientData>,
        params: Arc<GeneralFilters>,
        filters: Option<Arc<Filters>>,
    ) -> Result<Dashboard, String> {
        let charts_request_result = self.request_data(
            config,
            client_data,
            params,
            filters,
        ).await;

        let mut requested_charts = charts_request_result?;

        Ok(
            Dashboard::builder()
                .add_charts(&mut requested_charts)
                .build()
        )
    } 

    async fn request_data(
        self,
        config: Arc<Config>,
        client_data: Arc<ClientData>,
        params: Arc<GeneralFilters>,
        filters: Option<Arc<Filters>>,
    ) -> Result<Vec<Box<dyn ServiceResponse>>, String> {
        let response: Arc<Mutex<Vec<Box<dyn ServiceResponse>>>> = Arc::new(Mutex::new(Vec::new()));

        let mut tasks = Vec::new();

        for chart_requester in self.data_requesters {
            let response_clone = response.clone();
            
            let config_clone = config.clone();
            let client_data_clone = client_data.clone();
            let params_clone = params.clone();
            let filters_clone = filters.clone();
            
            let task = tokio::spawn(async move {
                let request_result = chart_requester.request_data(
                    config_clone,
                    client_data_clone,
                    params_clone,
                    filters_clone,
                ).await;

                //TODO: Add Error propper handling
                #[allow(clippy::question_mark)]
                if let Err(e) = request_result {
                    return Err(e);
                }
                let requested_chart = request_result.unwrap();
                
                response_clone.lock().await.push(
                    requested_chart
                );
                
                Ok(())
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