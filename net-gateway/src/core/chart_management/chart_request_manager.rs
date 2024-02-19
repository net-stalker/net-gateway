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

pub trait ChartResponse {}

#[async_trait::async_trait]
pub trait ChartRequestManagaer<R> 
where R : ChartResponse {
    //Requesting chart
    async fn request_chart(
        &self,
        state: web::Data<AppState>,
        client_data: web::Query<ClientData>,
        params: web::Query<GeneralFilters>,
    ) -> Result<R, String> {
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
    ) -> Result<R, String> {
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
    ) -> Result<R, String>;

    //Request creating
    fn get_request_type(&self) -> &'static str;

    fn form_dto_request(
        &self,
        params: web::Query<GeneralFilters>,
        client_data: &web::Query<ClientData>
    ) -> Box<dyn API>;

    fn form_enveloped_request(
        &self,
        params: web::Query<GeneralFilters>,
        client_data: web::Query<ClientData>
    ) -> Envelope {
        Envelope::new(
            Some(&client_data.group_id),
            None,
            self.get_request_type(),
            &self.form_dto_request(
                params,
                &client_data
            ).encode()
        )
    }

    fn form_request(
        &self,
        params: web::Query<GeneralFilters>,
        client_data: web::Query<ClientData>
    ) -> Vec<u8> {
        self.form_enveloped_request(
            params,
            client_data
        ).encode()
    }
}