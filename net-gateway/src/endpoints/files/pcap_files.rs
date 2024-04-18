use actix_web::post;
use actix_web::web;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_multipart::Multipart;

use futures::StreamExt;
use futures::TryStreamExt;

use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::typed_api::Typed;
use net_core_api::core::encoder_api::Encoder;

use net_inserter_api::api::network_packet::network_packet::NetworkPacketDTO;
use net_inserter_api::api::pcap_file::InsertPcapFileDTO;
use net_token_verifier::fusion_auth::fusion_auth_verifier::FusionAuthVerifier;

use crate::core::quinn_client_endpoint_manager::QuinnClientEndpointManager;
use crate::endpoints::files::core::network_packet::NetworkPacket;
use crate::{authorization, config::Config};

#[post("/pcap-files")]
async fn pcap_files(
    config: web::Data<Config>,
    req: HttpRequest,
    mut payload: Multipart
) -> impl Responder {
    //Auth stuff
    let token_verifier = FusionAuthVerifier::new(
        &config.fusion_auth_server_address.addr,
        Some(config.fusion_auth_api_key.key.clone())
    );

    let authorization_result = authorization::authorize(
        req,
        Box::new(token_verifier)
    ).await;

    if let Err(e) = authorization_result {
        return e;
    }
    let token = authorization_result.unwrap();

    let tenant_id = token.get_tenant_id();
    if let Err(e) = tenant_id {
        return HttpResponse::InternalServerError().body(e.to_string());
    }
    let tenant_id = tenant_id.unwrap();
    let mut packets: Vec<NetworkPacket> = Vec::default(); 
    
    while let Ok(Some(mut field)) = payload.try_next().await {
        // read the whole pcap file in bytes
        let mut file_bytes = web::BytesMut::new();
        while let Some(chunk) = field.next().await {
            let chunk = chunk.unwrap();
            file_bytes.extend_from_slice(&chunk);
        }
        let server_connection_result = QuinnClientEndpointManager::start_server_connection(
            &config.quin_client_address.addr,
            &config.quin_inserter.addr,
            &config.quin_server_application.app,
        ).await;
        let mut server_connection = match server_connection_result {
            Ok(server_connection) => server_connection,
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
        };
        let packet_data = InsertPcapFileDTO::new(&file_bytes);

        let request = Envelope::new(
            tenant_id,
            packet_data.get_type(),
            &packet_data.encode()
        );
        
        match server_connection.send_all_reliable(&request.encode()).await {
            Ok(_) => (),
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
        };

        match server_connection.receive_reliable().await {
            Ok(response) => {
                let enveloped_response = Envelope::decode(&response);
                if enveloped_response.get_envelope_type() != NetworkPacketDTO::get_data_type() {
                    return HttpResponse::InternalServerError().body("Received wrong data type after decoding, double check the data you want to decode");
                }
                packets.push(NetworkPacketDTO::decode(enveloped_response.get_data()).into());     
            },
            Err(err) => {
                return HttpResponse::InternalServerError().body(err.to_string());
            }
        }
    }

    HttpResponse::Ok().json(packets)
}
