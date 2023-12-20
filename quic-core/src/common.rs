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
    server_certs: Option<Vec<CertificateDer<'static>>>,
) -> Result<Endpoint, Box<dyn Error>> {
    let client_cfg = match server_certs {
        Some(certs) => configure_client(certs.as_slice())?,
        None => configure_client_inseccure(),
    };
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
pub fn make_server_endpoint(
    bind_addr: SocketAddr,
    cert: Option<Vec<CertificateDer<'static>>>,
    key: Option<PrivatePkcs1KeyDer<'static>>
) -> Result<Endpoint, Box<dyn Error>> {
    let server_config = match (cert, key) {
        (Some(cert), Some(key)) => configure_server(cert, key)?,
        (None, None) => configure_server_inseccure()?,
        _ => panic!("Both certificate and key must be provided or none of them"),
    };
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
fn configure_server(certs: Vec<CertificateDer<'static>>, key: PrivatePkcs1KeyDer<'static>) -> Result<ServerConfig, Box<dyn Error>> {
    let cert_der: Vec<Certificate> = certs
        .into_iter()
        .map(|cert| Certificate(cert.as_bytes().to_vec()))
        .collect();
    let priv_key = rustls::PrivateKey(key.secret_pkcs1_der().to_vec());
    
    let mut server_config = ServerConfig::with_single_cert(cert_der, priv_key)?;
    let transport_config = Arc::get_mut(&mut server_config.transport).unwrap();
    // TODO: make transport config configurable from connector builder
    transport_config.max_concurrent_uni_streams(0_u8.into());

    Ok(server_config)
}

/// Dummy certificate verifier that treats any certificate as valid.
/// NOTE, such verification is vulnerable to MITM attacks, but convenient for testing.
struct SkipServerVerification;

impl SkipServerVerification {
    fn new() -> Arc<Self> {
        Arc::new(Self)
    }
}

impl rustls::client::ServerCertVerifier for SkipServerVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::Certificate,
        _intermediates: &[rustls::Certificate],
        _server_name: &rustls::ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: std::time::SystemTime,
    ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::ServerCertVerified::assertion())
    }
}

fn configure_client_inseccure() -> ClientConfig {
    let crypto = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_custom_certificate_verifier(SkipServerVerification::new())
        .with_no_client_auth();

    ClientConfig::new(Arc::new(crypto))
}

fn configure_server_inseccure() -> Result<ServerConfig, Box<dyn Error>> {
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
    let cert_der = cert.serialize_der().unwrap();
    let priv_key = cert.serialize_private_key_der();
    let priv_key = rustls::PrivateKey(priv_key);
    let cert_chain = vec![rustls::Certificate(cert_der.clone())];

    let mut server_config = ServerConfig::with_single_cert(cert_chain, priv_key)?;
    let transport_config = Arc::get_mut(&mut server_config.transport).unwrap();
    transport_config.max_idle_timeout(None);

    Ok(server_config)
}
