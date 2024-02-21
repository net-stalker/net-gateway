use net_reporter_api::api::network_bandwidth::network_bandwidth_filters::NetworkBandwidthFiltersDTO;

use crate::core::filter::Filters;


impl From<Filters> for NetworkBandwidthFiltersDTO {
    fn from(_value: Filters) -> Self {
        todo!()
    }
}
