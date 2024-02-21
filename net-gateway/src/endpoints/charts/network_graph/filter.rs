use net_reporter_api::api::network_graph::network_graph_filters::NetworkGraphFiltersDTO;

use crate::core::filter::Filters;

impl From<Filters> for NetworkGraphFiltersDTO {
    fn from(_value: Filters) -> Self {
        todo!()
    }
}
