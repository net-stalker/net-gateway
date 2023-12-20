use net_timescale_api::api::bandwidth_per_endpoint::bandwidth_per_endpoint::BandwidthPerEndpointDTO;
use serde::{Serialize, Deserialize};

use super::chart_endpoint::ChartEndpoint;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BandwidthPerEndpoint {
    endpoints: Vec<ChartEndpoint>
}

impl BandwidthPerEndpoint {
    pub fn new(endpoints: Vec<ChartEndpoint>) -> Self {
        Self {
            endpoints
        }
    }
}

impl Default for BandwidthPerEndpoint {
    fn default() -> Self {
        log::info!("warinng: default BandwidthPerEndpoint is being constructed");
        Self {
            endpoints: Vec::new()
        }
    }
}

impl From<BandwidthPerEndpointDTO> for BandwidthPerEndpoint {
    fn from(value: BandwidthPerEndpointDTO) -> Self {
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