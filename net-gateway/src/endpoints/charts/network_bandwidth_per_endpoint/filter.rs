use net_reporter_api::api::network_bandwidth_per_endpoint::network_bandwidth_per_endpoint_filters::NetworkBandwidthPerEndpointFiltersDTO;

use crate::core::filter::Filters;

impl From<Filters> for NetworkBandwidthPerEndpointFiltersDTO {
    fn from(_value: Filters) -> Self {
        todo!()
    }
}
