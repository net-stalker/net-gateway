use serde::Deserialize;
use serde::Serialize;

use net_core_api::typed_api::Typed;

use net_reporter_api::api::network_bandwidth_per_protocol::network_bandwidth_per_protocol::NetworkBandwidthPerProtocolDTO;

use crate::core::service_request_management::service_response::ServiceResponse;

use super::protocol::ProtocolResponse;

const JSON_TYPE: &str = "networkBandwidthPerProtocol";

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct NetworkBandwidthPerProtocolResponse {
    protocols: Vec<ProtocolResponse>
}

impl ServiceResponse for NetworkBandwidthPerProtocolResponse {
    fn get_dto_type(&self) -> &'static str {
        NetworkBandwidthPerProtocolDTO::get_data_type()
    }

    fn get_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
    
    fn get_json_type(&self) -> &'static str {
        JSON_TYPE
    }
}

impl NetworkBandwidthPerProtocolResponse {
    pub fn new(protocols: Vec<ProtocolResponse>) -> Self {
        Self {
            protocols
        }
    }
}

impl From<NetworkBandwidthPerProtocolDTO> for NetworkBandwidthPerProtocolResponse {
    fn from(value: NetworkBandwidthPerProtocolDTO) -> Self {
        let protocols = value
            .get_protocols()
            .iter()
            .map(|protocol| ProtocolResponse::from(protocol.clone()))
            .collect::<Vec<ProtocolResponse>>();
        Self {
            protocols
        }
    }
}