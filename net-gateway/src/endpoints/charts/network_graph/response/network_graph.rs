use net_reporter_api::api::network_graph::network_graph::NetworkGraphDTO;

use serde::Serialize;
use serde::Deserialize;

use crate::core::chart_management::chart_request_manager::ChartResponse;

use super::graph_edge::GraphEdgeResponse;
use super::graph_node::GraphNodeResponse;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct NetworkGraphResponse {
    pub nodes: Vec<GraphNodeResponse>,
    pub links: Vec<GraphEdgeResponse>,
}

impl ChartResponse for NetworkGraphResponse {}

impl NetworkGraphResponse {
    pub fn new(nodes: Vec<GraphNodeResponse>, links: Vec<GraphEdgeResponse>) -> Self {
        NetworkGraphResponse {
            nodes,
            links,
        }
    }
}

impl From<NetworkGraphDTO> for NetworkGraphResponse {
    fn from(value: NetworkGraphDTO) -> Self {
        // TODO: need to remove these clones
        let edges_dto = value.get_graph_edges().to_vec();
        let nodes_dto = value.get_graph_nodes().to_vec();

        let mut links = Vec::with_capacity(edges_dto.len());
        let mut nodes = Vec::with_capacity(nodes_dto.len());

        for edge in edges_dto {
            links.push(GraphEdgeResponse::from(edge));
        }

        for node in nodes_dto {
            nodes.push(GraphNodeResponse::from(node));
        }

        NetworkGraphResponse { nodes, links }
    }
}
