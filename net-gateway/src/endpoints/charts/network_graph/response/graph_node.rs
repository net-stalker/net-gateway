use net_reporter_api::api::network_graph::graph_node;

use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GraphNodeResponse {
    id: String,
}

impl GraphNodeResponse {
    pub fn new(id: String) -> Self {
        GraphNodeResponse { id }
    }
}

impl From<graph_node::GraphNodeDTO> for GraphNodeResponse {
    fn from(value: graph_node::GraphNodeDTO) -> Self {
        GraphNodeResponse {
            id: value.get_node_id().to_string(),
        }
    }
}
