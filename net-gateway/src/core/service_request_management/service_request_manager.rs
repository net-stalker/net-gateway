use std::sync::Arc;

use net_core_api::decoder_api::Decoder;

use net_core_api::api::API;
use net_core_api::encoder_api::Encoder;
use net_core_api::envelope::envelope::Envelope;
use net_transport::quinn::connection::QuicConnection;

use crate::config::Config;
use crate::core::filter::Filters;
use crate::core::general_filters::GeneralFilters;
use crate::core::quinn_client_endpoint_manager::QuinnClientEndpointManager;

use super::service_response::ServiceResponse;

#[async_trait::async_trait]
pub trait ServiceRequestManager: Sync + Send {
    //Requesting chart
    async fn request_data(
        &self,
        config: Arc<Config>,
        jwt_token: Arc<String>,
        params: Arc<GeneralFilters>,
        filters: Option<Arc<Filters>>,
    ) -> Result<Box<dyn ServiceResponse>, String> {
        //Form request to the server
        let bytes_to_send = self.form_request(params, jwt_token, filters);

        //Creating Quinn Client Endpoint
        //Connecting with Quinn Client Endpoint to the server
        let server_connection_result = QuinnClientEndpointManager::start_server_connection(
            &config.quin_client_addres.addr,
            &config.quin_server_addres.addr,
            &config.quin_server_application.app,
        ).await;
        let server_connection = server_connection_result?;

        self.request_data_from_server(
            &bytes_to_send,
            server_connection
        ).await
    }

    //Requesting chart from server
    async fn request_data_from_server(
        &self,
        request: &[u8],
        mut server_connection: QuicConnection,
    ) -> Result<Box<dyn ServiceResponse>, String> {
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
    ) -> Result<Box<dyn ServiceResponse>, String>;

    fn get_requesting_type(&self) -> &'static str;

    //Request creating
    fn get_request_type(&self) -> &'static str;

    fn form_dto_request(
        &self,
        params: Arc<GeneralFilters>,
        filters: Option<Arc<Filters>>,
    ) -> Box<dyn API>;

    fn form_enveloped_request(
        &self,
        params: Arc<GeneralFilters>,
        jwt_token: Arc<String>,
        filters: Option<Arc<Filters>>,
    ) -> Envelope {
        Envelope::new(
            Some(&jwt_token),
            None,
            self.get_request_type(),
            &self.form_dto_request(
                params,
                filters,
            ).encode()
        )
    }

    fn form_request(
        &self,
        params: Arc<GeneralFilters>,
        jwt_token: Arc<String>,
        filters: Option<Arc<Filters>>,
    ) -> Vec<u8> {
        self.form_enveloped_request(
            params,
            jwt_token,
            filters,
        ).encode()
    }
}