use crate::handler::ConnectorHandler;

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

    pub async fn bind(mut self) {
        let handler = self.handler.take().expect("handler is not set");
        handler.handle(self).await;
    }
}
