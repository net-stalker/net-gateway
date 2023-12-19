use core::common::{make_server_endpoint, make_client_endpoint};
use std::net::SocketAddr;
use std::vec;

use rustls_pki_types::{CertificateDer, PrivatePkcs1KeyDer};

use crate::connector::QuicConnector;

use crate::core::Handler;

// TODO: may be expandable in future, if we want to create something like zmq patterns
pub enum ConnectorType {
    Client,
    Server,
}

pub struct QuicConnectorBuilder {
    pub certs: Vec<CertificateDer<'static>>,
    pub key: Option<PrivatePkcs1KeyDer<'static>>,
    pub addr: Option<SocketAddr>,
    pub application: Option<String>,
    pub connector_type: Option<ConnectorType>,
    pub handler: Option<Box<dyn Handler>>,
}

impl Default for QuicConnectorBuilder {
    fn default() -> Self {
        Self {
            certs: vec![],
            key: None,
            addr: None,
            application: None,
            connector_type: None,
            handler: None,
        }
    }
}

impl QuicConnectorBuilder {
    pub fn with_cert(mut self, cert: CertificateDer<'static>) -> Self {
        self.certs.push(cert);
        self
    }

    pub fn with_key(mut self, key: PrivatePkcs1KeyDer<'static>) -> Self {
        self.key = Some(key);
        self
    }

    pub fn with_addr(mut self, addr: SocketAddr) -> Self {
        self.addr = Some(addr);
        self
    }

    pub fn with_application(mut self, application: String) -> Self {
        self.application = Some(application);
        self
    }

    pub fn with_connector_type(mut self, connector_type: ConnectorType) -> Self {
        self.connector_type = Some(connector_type);
        self
    }

    pub fn with_handler(mut self, handler: Box<dyn Handler>) -> Self {
        self.handler = Some(handler);
        self
    }

    // TODO: create error type for not returning a String as an error
    pub fn build(self) -> Result<QuicConnector, String> {
        let endpoint = match self.connector_type {
            Some(ConnectorType::Client) => {
                make_client_endpoint(
                    self.addr.unwrap(),
                    self.certs
                ).unwrap()                
            },
            Some(ConnectorType::Server) => {
                make_server_endpoint(
                    self.addr.unwrap(),
                    self.certs,
                    self.key.unwrap(),
                // TODO: replace this unwrap with a proper error
                ).unwrap()
            },
            None => return Err("Connector type not specified".to_string()),
        };
        Ok(QuicConnector::new(endpoint, self.handler.unwrap(), self.application.unwrap()))
    }

}