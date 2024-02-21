use serde::Serialize;
use serde::Deserialize;

use net_core_api::typed_api::Typed;

use net_reporter_api::api::network_graph::network_graph::NetworkGraphDTO;

use crate::core::chart_management::chart_response::ChartResponse;

use super::graph_edge::GraphEdgeResponse;
use super::graph_node::GraphNodeResponse;

const JSON_TYPE: &'static str = "networkGraph";

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct NetworkGraphResponse {
    pub nodes: Vec<GraphNodeResponse>,
    pub links: Vec<GraphEdgeResponse>,
}

impl ChartResponse for NetworkGraphResponse {
    fn get_dto_type(&self) -> &'static str {
        NetworkGraphDTO::get_data_type()
    }

    fn get_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }

    fn get_json_type(&self) -> &'static str {
        JSON_TYPE
    }
}

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
