use std::sync::Arc;

use tokio::net::TcpStream;
use tokio_tungstenite::MaybeTlsStream;

const WEB_SOCKET_URL: &str = "ws://localhost:9091";

pub struct AppState {
    pub consumer: Arc<tokio_tungstenite::WebSocketStream<MaybeTlsStream<TcpStream>>>, 
}

impl AppState {
    pub async fn new() -> Self {
        let consumer = tokio_tungstenite::connect_async(WEB_SOCKET_URL)
            .await
            .expect("Failed to connect")
            .0;
        Self { consumer: Arc::new(consumer) }
    }
}

impl Clone for AppState {
    fn clone(&self) -> Self {   
        Self { consumer: self.consumer.clone() }
    }
}