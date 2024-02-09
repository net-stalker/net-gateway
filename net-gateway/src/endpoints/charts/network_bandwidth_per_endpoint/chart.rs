use net_reporter_api::api::network_bandwidth_per_endpoint::network_bandwidth_per_endpoint::NetworkBandwidthPerEndpointDTO;

use serde::Deserialize;
use serde::Serialize;

use super::chart_endpoint::ChartEndpoint;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetworkBandwidthPerEndpoint {
    endpoints: Vec<ChartEndpoint>
}

impl NetworkBandwidthPerEndpoint {
    pub fn new(endpoints: Vec<ChartEndpoint>) -> Self {
        Self {
            endpoints
        }
    }
}

impl Default for NetworkBandwidthPerEndpoint {
    fn default() -> Self {
        log::info!("warinng: default BandwidthPerEndpoint is being constructed");
        Self {
            endpoints: Vec::new()
        }
    }
}

impl From<NetworkBandwidthPerEndpointDTO> for NetworkBandwidthPerEndpoint {
    fn from(value: NetworkBandwidthPerEndpointDTO) -> Self {
        let endpoints = value
            .get_endpoints()
            .iter()
            .map(|endpoint| ChartEndpoint::from(endpoint.clone()))
            .collect::<Vec<ChartEndpoint>>();
        Self {
            endpoints
        }
    }
}