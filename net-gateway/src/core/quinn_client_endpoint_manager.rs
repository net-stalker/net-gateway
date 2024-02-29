use net_transport::quinn::client::builder::ClientQuicEndpointBuilder;
use net_transport::quinn::connection::QuicConnection;

pub struct QuinnClientEndpointManager {}

impl QuinnClientEndpointManager {
    pub async fn start_server_connection(
        quinn_client_addres: &str,
        quinn_server_addres: &str,
        quinn_server_application: &str,
    ) -> Result<QuicConnection, String> {
        //Creating Quinn Client Endpoint
        let client_endpoint_build_result = ClientQuicEndpointBuilder::default()
            .with_addr(quinn_client_addres.parse().unwrap())
            .build();
        let mut client_endpoint = client_endpoint_build_result?;

        //Connecting with Quinn Client Endpoint to the server
        client_endpoint.connect(
            quinn_server_addres.parse().unwrap(),
            quinn_server_application
        ).await
    }
}