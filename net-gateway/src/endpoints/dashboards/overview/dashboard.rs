use serde::Deserialize;
use serde::Serialize;

use net_core_api::decoder_api::Decoder;
use net_core_api::typed_api::Typed;

use net_reporter_api::api::network_graph::network_graph::NetworkGraphDTO;
use net_reporter_api::api::network_bandwidth_per_endpoint::network_bandwidth_per_endpoint::NetworkBandwidthPerEndpointDTO;
use net_reporter_api::api::network_bandwidth::network_bandwidth::NetworkBandwidthDTO;
use net_reporter_api::api::dashboard::dashboard::DashboardDTO;

use crate::endpoints::charts::network_bandwidth_per_endpoint::response::network_bandwidth_per_endpoint::NetworkBandwidthPerEndpointResponse;
use crate::endpoints::charts::network_bandwidth::response::network_bandwidth::NetworkBandwidthResponse;
use crate::endpoints::charts::network_graph::chart::NetworkGraph;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct OverviewDashboard {
    #[serde(rename = "networkBandwidth")]
    pub network_bandwidth: NetworkBandwidthResponse,
    #[serde(rename = "bandwidthPerEndpoint")]
    pub bandwidth_per_endpoint: NetworkBandwidthPerEndpointResponse,
    #[serde(rename = "networkGraph")]
    pub network_graph: NetworkGraph,
    // TODO: add pie chart
}

impl OverviewDashboard {
    pub fn new(network_bandwidth: NetworkBandwidthResponse, bandwidth_per_endpoint: NetworkBandwidthPerEndpointResponse, network_graph: NetworkGraph) -> Self {
        Self { network_bandwidth, bandwidth_per_endpoint, network_graph }
    }
}

impl From<DashboardDTO> for OverviewDashboard {
    fn from(value: DashboardDTO) -> Self {
        let charts = value.get_charts();
        let mut network_bandwidth = None;
        let mut bandwidth_per_endpoint = None;
        let mut network_graph = None;

        for chart in charts {
            if chart.get_type() == NetworkBandwidthDTO::get_data_type() {
                network_bandwidth = Some(NetworkBandwidthResponse::from(NetworkBandwidthDTO::decode(chart.get_data())));
            } else if chart.get_type() == NetworkBandwidthPerEndpointDTO::get_data_type() {
                bandwidth_per_endpoint = Some(NetworkBandwidthPerEndpointResponse::from(NetworkBandwidthPerEndpointDTO::decode(chart.get_data())));
            } else if chart.get_type() == NetworkGraphDTO::get_data_type() {
                network_graph = Some(NetworkGraph::from(NetworkGraphDTO::decode(chart.get_data())));
            } else {
                log::error!("received unknown chart type: {}", chart.get_type());
            }
        }

        OverviewDashboard::new(
            network_bandwidth.unwrap_or_default(),
            bandwidth_per_endpoint.unwrap_or_default(),
            network_graph.unwrap_or_default(),
        )
    }
}