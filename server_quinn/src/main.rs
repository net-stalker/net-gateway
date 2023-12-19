use quic_core::certs;
use quic_core::common::make_server_endpoint;

const CERT_PATH: &str = "quic_core/certs/cert.pem"; 
const KEY_PATH: &str = "quic_core/certs/private.pem";

#[tokio::main]
async fn main() {
    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let certificate = certs::read_pem_cert(CERT_PATH).unwrap();
    let key = certs::read_pem_key(KEY_PATH).unwrap();
    let endpoint = make_server_endpoint(server_addr, vec![certificate], key).unwrap();
    // accept a single connection
    // tokio::spawn(async move {
    println!("waiting for incoming connection");
    let incoming_conn = endpoint.accept().await.unwrap();
    println!("received a new connection");
    let conn = incoming_conn.await.unwrap();
    let (mut write, mut read) = conn.accept_bi().await.unwrap();
    let mut counter = 0;
    loop {
        if counter < 10 {
            counter += 1;
        } else {
            break;
        }
        let mut buffer = [0; 1024];
        let bytes = read.read(&mut buffer).await.unwrap().unwrap();
        let message = format!("received message: {:?}", buffer);
        println!("{}", message);
        println!("received {} bytes", bytes);
        write.write_all(message.as_bytes()).await.unwrap();
    }
}
