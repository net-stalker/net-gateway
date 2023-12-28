use net_timescale_api::api::network_graph::graph_edge;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Link {
    pub source: String,
    pub target: String,
}

impl Link {
    pub fn new(source: String, target: String) -> Self {
        Link { source, target }
    }
}

impl From<graph_edge::GraphEdgeDTO> for Link {
    fn from(value: graph_edge::GraphEdgeDTO) -> Self {
        Link {
            source: value.get_src_id().to_string(),
            target: value.get_dst_id().to_string(),
        }
    }
}
