use async_trait::async_trait;

use crate::connector::QuicConnector;

#[derive(Debug, PartialEq)]
pub enum ConnectorState {
    Closed,
    Open,
    Message(String),
}

#[async_trait]
pub trait ConnectorHandler: Send + Sync {
    // probably Connector should be a trait object
    async fn handle(&self, connector: &mut QuicConnector) -> ConnectorState;
}