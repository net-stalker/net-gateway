use net_reporter_api::api::network_bandwidth_per_endpoint::network_bandwidth_per_endpoint::NetworkBandwidthPerEndpointDTO;

use serde::Deserialize;
use serde::Serialize;

use crate::core::chart_management::chart_request_manager::ChartResponse;

use super::endpoint::EndpointResponse;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct NetworkBandwidthPerEndpointResponse {
    endpoints: Vec<EndpointResponse>
}

impl ChartResponse for NetworkBandwidthPerEndpointResponse {}

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