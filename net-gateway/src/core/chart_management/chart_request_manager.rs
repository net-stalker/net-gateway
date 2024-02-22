use std::sync::Arc;

use actix_web::web;

use net_core_api::decoder_api::Decoder;

use net_core_api::api::API;
use net_core_api::encoder_api::Encoder;
use net_core_api::envelope::envelope::Envelope;
use net_transport::quinn::connection::QuicConnection;

use crate::core::app_state::AppState;
use crate::core::client_data::ClientData;
use crate::core::general_filters::GeneralFilters;
use crate::core::quinn_client_endpoint_manager::QuinnClientEndpointManager;

use super::chart_response::ChartResponse;

#[async_trait::async_trait]
pub trait ChartRequestManagaer: Sync + Send {
    //Requesting chart
    async fn request_chart(
        &self,
        state: Arc<web::Data<AppState>>,
        client_data: Arc<web::Query<ClientData>>,
        params: Arc<web::Query<GeneralFilters>>,
    ) -> Result<Box<dyn ChartResponse>, String> {
        //Form request to the server
        let bytes_to_send = self.form_request(params, client_data);

        //Creating Quinn Client Endpoint
        //Connecting with Quinn Client Endpoint to the server
        let server_connection_result = QuinnClientEndpointManager::start_server_connection(
            state.get_quinn_client_addres(),
            state.get_quinn_server_addres(),
            state.get_quinn_server_application()
        ).await;
        let server_connection = server_connection_result?;

        self.request_chart_from_server(
            &bytes_to_send,
            server_connection
        ).await
    }

    //Requesting chart from server
    async fn request_chart_from_server(
        &self,
        request: &[u8],
        mut server_connection: QuicConnection,
    ) -> Result<Box<dyn ChartResponse>, String> {
        //Sending out data (request) to the server
        server_connection.send_all_reliable(request).await?;

        //Waiting on new data and reading message from the server
        let receiving_result = server_connection.receive_reliable().await;
        let received_bytes = receiving_result?;

        let received_envelope = Envelope::decode(&received_bytes);

        self.decode_received_envelope(received_envelope)
    }

    fn decode_received_envelope(
        &self,
        received_envelope: Envelope
    ) -> Result<Box<dyn ChartResponse>, String>;

    fn get_requesting_type(&self) -> &'static str;

    //Request creating
    fn get_request_type(&self) -> &'static str;

    fn form_dto_request(
        &self,
        params: Arc<web::Query<GeneralFilters>>,
        client_data: Arc<web::Query<ClientData>>
    ) -> Box<dyn API>;

    fn form_enveloped_request(
        &self,
        params: Arc<web::Query<GeneralFilters>>,
        client_data: Arc<web::Query<ClientData>>
    ) -> Envelope {
        Envelope::new(
            Some(&client_data.group_id),
            None,
            self.get_request_type(),
            &self.form_dto_request(
                params,
                client_data.clone()
            ).encode()
        )
    }

    fn form_request(
        &self,
        params: Arc<web::Query<GeneralFilters>>,
        client_data: Arc<web::Query<ClientData>>
    ) -> Vec<u8> {
        self.form_enveloped_request(
            params,
            client_data
        ).encode()
    }
}