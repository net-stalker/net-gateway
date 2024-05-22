use actix_web::post;
use actix_web::web;
use actix_web::HttpRequest;
use actix_multipart::Multipart;

use futures::StreamExt;
use futures::TryStreamExt;

use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::api::result::result::ResultDTO;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::typed_api::Typed;
use net_core_api::core::encoder_api::Encoder;

use net_inserter_api::api::pcap_file::InsertPcapFileDTO;
use net_token_verifier::fusion_auth::fusion_auth_verifier::FusionAuthVerifier;

use crate::core::quinn_client_endpoint_manager::QuinnClientEndpointManager;
use crate::core::user_facing_error::UserFacingError;
use crate::{authorization, config::Config};

#[post("/pcap-file")]
async fn pcap_file(
    config: web::Data<Config>,
    req: HttpRequest,
    mut payload: Multipart
) -> Result<String, UserFacingError> {
    //Auth stuff
    let token_verifier = FusionAuthVerifier::new(
        &config.fusion_auth_server_address.addr,
        Some(config.fusion_auth_api_key.key.clone())
    );

    let authorization_result = authorization::authorize(
        req,
        Box::new(token_verifier)
    ).await;

    if authorization_result.is_err() { return Err(UserFacingError::Unauthorized); }

    let token = authorization_result.unwrap();

    let tenant_id = token.get_tenant_id();
    if let Err(e) = tenant_id {
        return Err(UserFacingError::InternalErrorWithDescription(e.to_string()));
    }
    let tenant_id = tenant_id.unwrap();
    let mut field = if let Ok(Some(field)) = payload.try_next().await {
        field
    } else {
        return Err(UserFacingError::InternalErrorWithDescription("didn't receive a file to insert".to_string()));
    };
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
        Err(_) => return Err(UserFacingError::Timeout),
    };
    let packet_data = InsertPcapFileDTO::new(&file_bytes);

    let request = Envelope::new(
        tenant_id,
        packet_data.get_type(),
        &packet_data.encode()
    );
    
    match server_connection.send_all_reliable(&request.encode()).await {
        Ok(_) => (),
        Err(_) => return Err(UserFacingError::Timeout),
    };

    let response = match server_connection.receive_reliable().await {
        Ok(response) => ResultDTO::decode(Envelope::decode(&response).get_data()),
        Err(err) => return Err(UserFacingError::InternalErrorWithDescription(err.to_string())),
    };
    let packet_id = match response.is_ok() {
        true => {
            String::from_utf8(response.into_inner().unwrap().get_data().to_vec()).unwrap()
        },
        false => {
            if response.get_description().is_err() {
                return Err(UserFacingError::InternalError);
            }
            return Err(UserFacingError::InternalErrorWithDescription(response.get_description().unwrap().to_string()));
        },
    };

    Ok(packet_id)
}
