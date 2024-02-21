use serde::Deserialize;
use serde::Serialize;

use net_core_api::typed_api::Typed;

use net_reporter_api::api::network_bandwidth_per_endpoint::network_bandwidth_per_endpoint::NetworkBandwidthPerEndpointDTO;

use crate::core::chart_management::chart_response::ChartResponse;

use super::endpoint::EndpointResponse;

const JSON_TYPE: &'static str = "networkBandwidthPerEndpoint";

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct NetworkBandwidthPerEndpointResponse {
    endpoints: Vec<EndpointResponse>
}

impl ChartResponse for NetworkBandwidthPerEndpointResponse {
    fn get_dto_type(&self) -> &'static str {
        NetworkBandwidthPerEndpointDTO::get_data_type()
    }

    fn get_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
    
    fn get_json_type(&self) -> &'static str {
        JSON_TYPE
    }
}

impl NetworkBandwidthPerEndpointResponse {
    pub fn new(endpoints: Vec<EndpointResponse>) -> Self {
        Self {
            endpoints
        }
    }
}

impl From<NetworkBandwidthPerEndpointDTO> for NetworkBandwidthPerEndpointResponse {
    fn from(value: NetworkBandwidthPerEndpointDTO) -> Self {
        let endpoints = value
            .get_endpoints()
            .iter()
            .map(|endpoint| EndpointResponse::from(endpoint.clone()))
            .collect::<Vec<EndpointResponse>>();
        Self {
            endpoints
        }
    }
}