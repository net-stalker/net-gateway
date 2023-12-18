use quinn::{ClientConfig, Endpoint, ServerConfig};
use rustls::Certificate;
use rustls_pki_types::{CertificateDer, PrivatePkcs1KeyDer};
use x509_parser::nom::AsBytes;
use std::{error::Error, net::SocketAddr, sync::Arc};

/// Constructs a QUIC endpoint configured for use a client only.
///
/// ## Args
///
/// - server_certs: list of trusted certificates.
#[allow(unused)]
pub fn make_client_endpoint(
    bind_addr: SocketAddr,
    server_certs: Vec<CertificateDer<'static>>,
) -> Result<Endpoint, Box<dyn Error>> {
    let client_cfg = configure_client(server_certs.as_slice())?;
    let mut endpoint = Endpoint::client(bind_addr)?;
    endpoint.set_default_client_config(client_cfg);
    Ok(endpoint)
}

/// Constructs a QUIC endpoint configured to listen for incoming connections on a certain address
/// and port.
///
/// ## Returns
///
/// - a stream of incoming QUIC connections
/// - server certificate serialized into DER format
#[allow(unused)]
pub fn make_server_endpoint(bind_addr: SocketAddr, cert: CertificateDer<'static>, key: PrivatePkcs1KeyDer<'static>) -> Result<Endpoint, Box<dyn Error>> {
    let server_config = configure_server(cert, key)?;
    let endpoint = Endpoint::server(server_config, bind_addr)?;
    Ok(endpoint)
}

/// Builds default quinn client config and trusts given certificates.
///
/// ## Args
///
/// - server_certs: a list of trusted certificates in DER format.
fn configure_client(server_certs: &[CertificateDer<'static>]) -> Result<ClientConfig, Box<dyn Error>> {
    let mut certs = rustls::RootCertStore::empty();
    for cert in server_certs {
        certs.add(&Certificate(cert.as_bytes().to_vec()))?;
    }

    let client_config = ClientConfig::with_root_certificates(certs);
    Ok(client_config)
}

/// Returns default server configuration along with its certificate.
fn configure_server(cert: CertificateDer<'static>, key: PrivatePkcs1KeyDer<'static>) -> Result<ServerConfig, Box<dyn Error>> {
    let cert_der = Certificate(cert.as_bytes().to_vec());
    let priv_key = rustls::PrivateKey(key.secret_pkcs1_der().to_vec());
    
    let mut server_config = ServerConfig::with_single_cert(vec![cert_der], priv_key)?;
    let transport_config = Arc::get_mut(&mut server_config.transport).unwrap();
    transport_config.max_concurrent_uni_streams(0_u8.into());

    Ok(server_config)
}
