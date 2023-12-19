use crate::core::Handler;

pub struct QuicConnector {
    pub endpoint: quinn::Endpoint,
    pub handler: Box<dyn Handler>,
    pub application: String,    
}
impl QuicConnector {
    pub fn new(endpoint: quinn::Endpoint, handler: Box<dyn Handler>, application: String) -> Self {
        Self {
            endpoint,
            handler,
            application
        }
    }
}