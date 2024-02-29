const QUINN_CLIENT_ADDRES: &str = "0.0.0.0:0";
const QUINN_SERVER_ADDRES: &str = "127.0.0.1:5454";
const QUINN_SERVER_APPLICATION: &str = "localhost";

#[derive(Clone, Debug)]
pub struct AppState { }

impl AppState {
    pub async fn new() -> Self { 
        Self { }
    }
    
    pub fn get_quinn_client_addres(&self) -> &'static str {
        QUINN_CLIENT_ADDRES
    }
    pub fn get_quinn_server_addres(&self) -> &'static str {
        QUINN_SERVER_ADDRES
    }
    pub fn get_quinn_server_application(&self) -> &'static str {
        QUINN_SERVER_APPLICATION
    }
}
