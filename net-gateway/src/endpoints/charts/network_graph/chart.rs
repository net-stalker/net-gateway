use net_timescale_api::api::network_graph::network_graph::NetworkGraphDTO;
use serde::{Deserialize, Serialize};
use super::{node::Node, link::Link};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetworkGraph {
    pub nodes: Vec<Node>,
    pub links: Vec<Link>,
}

impl NetworkGraph {
    pub fn new(nodes: Vec<Node>, links: Vec<Link>) -> Self {
        NetworkGraph {
            nodes,
            links,
        }
    }
}

impl From<NetworkGraphDTO> for NetworkGraph {
    fn from(value: NetworkGraphDTO) -> Self {
        // TODO: need to remove these clones
        let edges_dto = value.get_graph_edges().to_vec();
        let nodes_dto = value.get_graph_nodes().to_vec();

        let mut links = Vec::with_capacity(edges_dto.len());
        let mut nodes = Vec::with_capacity(nodes_dto.len());

        for edge in edges_dto {
            links.push(Link::from(edge));
        }

        for node in nodes_dto {
            nodes.push(Node::from(node));
        }

        NetworkGraph { nodes, links }
    }
}
