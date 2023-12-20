use net_timescale_api::api::network_graph::graph_node;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Node {
    id: String,
}

impl Node {
    pub fn new(id: String) -> Self {
        Node { id }
    }
}

impl From<graph_node::GraphNodeDTO> for Node {
    fn from(value: graph_node::GraphNodeDTO) -> Self {
        Node {
            id: value.get_node_id().to_string(),
        }
    }
}
