use actix_web::web;

use crate::core::app_state::AppState;
use crate::core::client_data::ClientData;
use crate::core::general_filters::GeneralFilters;
use crate::core::quinn_client_endpoint_manager::QuinnClientEndpointManager;

use super::chart_requester::ChartRequester;
use super::request_creator::RequestCreator;

#[async_trait::async_trait]
pub trait ChartRequestManagaer {
    type RequestCreator: RequestCreator;
    type Requester: ChartRequester;
    
    async fn request_chart(
        state: web::Data<AppState>,
        client_data: web::Query<ClientData>,
        params: web::Query<GeneralFilters>,
    ) -> Result<<Self::Requester as ChartRequester>::Response, String> {
        //Form request to the server
        let bytes_to_send = Self::RequestCreator::form_request(params, client_data);

        //Creating Quinn Client Endpoint
        //Connecting with Quinn Client Endpoint to the server
        let server_connection_result = QuinnClientEndpointManager::start_server_connection(
            state.get_quinn_client_addres(),
            state.get_quinn_server_addres(),
            state.get_quinn_server_application()
        ).await;
        let server_connection = server_connection_result?;

        Self::Requester::request_chart(
            &bytes_to_send,
            server_connection
        ).await
    }
}