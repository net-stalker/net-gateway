const WEB_SOCKET_URL: &str = "ws://localhost:9091";

#[derive(Clone, Debug)]
pub struct AppState { }

impl AppState {
    pub async fn new() -> Self { 
        Self { }
    }
    pub fn get_ws_url(&self) -> &'static str {
        WEB_SOCKET_URL
    }
}
