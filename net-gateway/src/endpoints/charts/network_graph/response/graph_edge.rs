use net_reporter_api::api::network_graph::graph_edge;

use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GraphEdgeResponse {
    pub source: String,
    pub target: String,
}

impl GraphEdgeResponse {
    pub fn new(source: String, target: String) -> Self {
        GraphEdgeResponse { source, target }
    }
}

impl From<graph_edge::GraphEdgeDTO> for GraphEdgeResponse {
    fn from(value: graph_edge::GraphEdgeDTO) -> Self {
        GraphEdgeResponse {
            source: value.get_src_id().to_string(),
            target: value.get_dst_id().to_string(),
        }
    }
}
