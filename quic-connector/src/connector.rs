use crate::handler::{ConnectorHandler, ConnectorState};

pub struct QuicConnector {
    pub endpoint: quinn::Endpoint,
    pub handler: Option<Box<dyn ConnectorHandler>>,
    pub application: String,    
}

impl QuicConnector {
    pub fn new(endpoint: quinn::Endpoint, handler: Box<dyn ConnectorHandler>, application: String) -> Self {
        Self {
            endpoint,
            handler: Some(handler),
            application
        }
    }

    pub async fn activate(mut self) -> (Self, ConnectorState) {
        let handler = self.handler.take().expect("handler is not set");
        let connector_state = handler.handle(&mut self).await;
        self.handler = Some(handler);
        (self, connector_state)
    }
}

#[cfg(test)]
mod tests {
    const TEST_ADDRESS: &str = "127.0.0.1:4545";
    const TEST_APPLICATION: &str = "localhost";
    use crate::{handler::ConnectorState, builder::{QuicConnectorBuilder, ConnectorType}};

    use super::*;

    struct AloneServerHandler {}

    
    #[async_trait::async_trait]
    impl ConnectorHandler for AloneServerHandler {
        async fn handle(&self, _connector: &mut QuicConnector) -> ConnectorState {
            ConnectorState::Message("AloneServerHandler".to_string())
        }
    }

    struct UniServerHandler {}

    #[async_trait::async_trait]
    impl ConnectorHandler for UniServerHandler {
        async fn handle(&self, connector: &mut QuicConnector) -> ConnectorState {
            const EXPTECTED_BYTES: [u8; 10] = [0,1,2,3,4,5,6,7,8,9];
            let incomming_connect = connector.endpoint.accept().await;
            assert!(incomming_connect.is_some());
            let incomming_connect = incomming_connect.unwrap();

            let conn = incomming_connect.await;
            let conn = conn.unwrap();

            let mut read = conn.accept_uni().await.unwrap();

            let mut buffer = [0; 100];

            let bytes = read.read(&mut buffer).await;

            assert!(bytes.is_ok());
            let bytes = bytes.unwrap().unwrap();
            assert_eq!(bytes, EXPTECTED_BYTES.len());
            assert_eq!(EXPTECTED_BYTES, &buffer[0..EXPTECTED_BYTES.len()]);
            ConnectorState::Open
        }
    }

    struct UniClientHandler {}

    #[async_trait::async_trait]
    impl ConnectorHandler for UniClientHandler {
        async fn handle(&self, connector: &mut QuicConnector) -> ConnectorState {
            const BYTES_TO_SEND: [u8; 10] = [0,1,2,3,4,5,6,7,8,9];
            let connection = connector
                .endpoint
                .connect(TEST_ADDRESS.parse().unwrap(), TEST_APPLICATION);
            assert!(connection.is_ok());
            let connection = connection.unwrap().await;
            
            assert!(connection.is_ok());
            let connection = connection.unwrap();
            
            let write = connection.open_uni().await;
            assert!(write.is_ok());
            let mut write = write.unwrap();

            let write_result = write.write(BYTES_TO_SEND.as_ref()).await;
            assert!(write_result.is_ok());
            assert_eq!(write_result.unwrap(), BYTES_TO_SEND.len());
            drop(connection);
            // Make sure the server has a chance to clean up
            connector.endpoint.wait_idle().await;

            ConnectorState::Message("AloneClientHandler".to_string())
        }
    }
    
    #[tokio::test]
    async fn test_binding_and_running_simple_handler() {
        let connector = QuicConnectorBuilder::default()
            .with_addr(TEST_ADDRESS.parse().unwrap())
            .with_application(TEST_APPLICATION.to_string())
            .with_connector_type(ConnectorType::Server)
            .with_handler(Box::new(AloneServerHandler {}))
            .build();
        assert!(connector.is_ok());
        let connector = connector.unwrap();
        let (_connector, connector_state) = connector.activate().await;
        assert_eq!(connector_state, ConnectorState::Message("AloneServerHandler".to_string()));
    }

    #[tokio::test]
    async fn test_binding_and_running_simple_handler_with_client() {
        let client_task = tokio::spawn(async {
            let client_connector = QuicConnectorBuilder::default()
            .with_addr("0.0.0.0:0".parse().unwrap())
            .with_application(TEST_APPLICATION.to_string())
            .with_connector_type(ConnectorType::Client)
            .with_handler(Box::new(UniClientHandler {}))
            .build();
            assert!(client_connector.is_ok());
            let client_connector = client_connector.unwrap();
            let (_client_connector, connector_state) = client_connector.activate().await;
            connector_state
        });
        let server_task = tokio::spawn(async {
            let server_connector = QuicConnectorBuilder::default()
                .with_addr(TEST_ADDRESS.parse().unwrap())
                .with_application(TEST_APPLICATION.to_string())
                .with_connector_type(ConnectorType::Server)
                .with_handler(Box::new(UniServerHandler {}))
                .build();
            assert!(server_connector.is_ok());
            let server_connector = server_connector.unwrap();
            let (_server_connector, connector_state) = server_connector.activate().await;
            connector_state
        });
        tokio::try_join!(client_task, server_task).unwrap();
    }
}
