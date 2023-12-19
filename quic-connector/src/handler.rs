use async_trait::async_trait;

use crate::connector::QuicConnector;

pub enum ConnectorState {
    Closed,
    Open,
    Err(String),
}

#[async_trait]
pub trait ConnectorHandler: Send + Sync {
    // probably Connector should be a trait object
    async fn handle(&self, mut connector: QuicConnector) -> ConnectorState;
}