use std::error::Error;
use std::sync::Arc;

use net_core_api::decoder_api::Decoder;

use net_core_api::api::API;
use net_core_api::encoder_api::Encoder;
use net_core_api::envelope::envelope::Envelope;
use net_reporter_api::api::request_result::request_result::RequestResultDTO;

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
    ) -> Result<Box<dyn ServiceResponse>, Box<dyn Error + Send + Sync>> {
        //Form request to the server
        let request = self.form_request(params, jwt_token, filters);

        //Creating Quinn Client Endpoint
        //Connecting with Quinn Client Endpoint to the server
        let server_connection_result = QuinnClientEndpointManager::start_server_connection(
            &config.quin_client_addres.addr,
            &config.quin_server_addres.addr,
            &config.quin_server_application.app,
        ).await;
        let mut server_connection = server_connection_result?;

        //Sending out data (request) to the server
        server_connection.send_all_reliable(&request).await?;

        //Waiting on new data and reading message from the server
        let receiving_result = server_connection.receive_reliable().await;
        let received_bytes = receiving_result?;

        let received_envelope = Envelope::decode(&received_bytes);
        let received_result = RequestResultDTO::decode(received_envelope.get_data());

        if !received_result.is_ok() {
            match received_result.get_description() {
                Ok(e) => {
                    return Err(e.to_string().into());
                },
                Err(_) => {
                    return Err(String::from("Error with no description was recieved from the report server").into());
                },
            }
        }

        match received_result.into_inner() {
            Some(envelope) => self.decode_received_envelope(envelope),
            None => return Err(String::from("NULL-Response was recieved from the report server").into()),
        }
    }

    fn decode_received_envelope(
        &self,
        received_envelope: Envelope
    ) -> Result<Box<dyn ServiceResponse>, Box<dyn Error + Send + Sync>>;

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