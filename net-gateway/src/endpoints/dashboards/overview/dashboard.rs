use serde::Deserialize;
use serde::Serialize;

use net_proto_api::decoder_api::Decoder;
use net_proto_api::typed_api::Typed;

use net_timescale_api::api::network_graph::network_graph::NetworkGraphDTO;
use net_timescale_api::api::bandwidth_per_endpoint::bandwidth_per_endpoint::BandwidthPerEndpointDTO;
use net_timescale_api::api::network_bandwidth::network_bandwidth::NetworkBandwidthDTO;
use net_timescale_api::api::dashboard::dashboard::DashboardDTO;

use crate::endpoints::charts::bandwidth_per_endpoint::chart::BandwidthPerEndpoint;
use crate::endpoints::charts::network_bandwidth::chart::NetworkBandwidth;
use crate::endpoints::charts::network_graph::chart::NetworkGraph;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct OverviewDashboard {
    #[serde(rename = "networkBandwidth")]
    pub network_bandwidth: NetworkBandwidth,
    #[serde(rename = "bandwidthPerEndpoint")]
    pub bandwidth_per_endpoint: BandwidthPerEndpoint,
    #[serde(rename = "networkGraph")]
    pub network_graph: NetworkGraph,
    // TODO: add pie chart
}

impl OverviewDashboard {
    pub fn new(network_bandwidth: NetworkBandwidth, bandwidth_per_endpoint: BandwidthPerEndpoint, network_graph: NetworkGraph) -> Self {
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
                network_bandwidth = Some(NetworkBandwidth::from(NetworkBandwidthDTO::decode(chart.get_data())));
            } else if chart.get_type() == BandwidthPerEndpointDTO::get_data_type() {
                bandwidth_per_endpoint = Some(BandwidthPerEndpoint::from(BandwidthPerEndpointDTO::decode(chart.get_data())));
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