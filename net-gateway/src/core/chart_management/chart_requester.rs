use net_core_api::api::API;
use net_core_api::decoder_api::Decoder;
use net_core_api::envelope::envelope::Envelope;
use net_core_api::typed_api::Typed;

use net_transport::quinn::connection::QuicConnection;

use serde::Deserialize;
use serde::Serialize;

pub trait ChartResponse<DTO>: Serialize + for<'a> Deserialize<'a> + From<DTO> {}

#[async_trait::async_trait]
pub trait ChartRequester: Send {
    type ResponseDTO: API;
    type Response: ChartResponse<Self::ResponseDTO>;
    
    async fn request_chart(
        request: &[u8],
        mut server_connection: QuicConnection,
    ) -> Result<Self::Response, String> {
        //Sending out data (request) to the server
        server_connection.send_all_reliable(request).await?;

        //Waiting on new data and reading message from the server
        let receiving_result = server_connection.receive_reliable().await;
        let received_bytes = receiving_result?;

        let received_envelope = Envelope::decode(&received_bytes);

        //TODO: Think about letting it all sit here. Maybe this checking is not necessary
        if received_envelope.get_type() != Self::ResponseDTO::get_data_type() {
            //TODO: Write appropriate error returning
            return Err("Wrong type resieved!".to_string());
        }

        let received_chart = Self::ResponseDTO::decode(received_envelope.get_data());
        Ok(Self::Response::from(received_chart))
    }
}