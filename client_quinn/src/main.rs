use quic_core::common::make_client_endpoint;
use std::net::SocketAddr;

const SERVER_ADDR: &str = "127.0.0.1:5000";
const SERVER_CERT: &str = "core/certs/rootCA.crt";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let addr: SocketAddr = SERVER_ADDR.parse()?;
    // let certs = vec![core::certs::read_certificate_from_file(SERVER_CERT)?];
    let der_certs = vec![quic_core::certs::read_pem_cert(SERVER_CERT)?];

    let endpoint = make_client_endpoint("0.0.0.0:0".parse()?, der_certs)?;
    let connection = endpoint
        .connect(addr, "server")?
        .await?;
    println!("connected: addr={}", connection.remote_address());
    
    let (mut write, _) = connection.open_bi().await?;
    let mut counter = 0;
    loop {
        if counter < 10 {
            counter += 1;
        } else {
            break;
        }
        let message = format!("Hello from client {}", counter);
        println!("sending: {}", message);
        write.write_all(message.as_bytes()).await?;
        // let data = read.read_to_end(usize::MAX).await?;
        // let message = String::from_utf8(data)?;
        // println!("received message: {}", message);
    }
    Ok(())
}