use net_reporter_api::api::network_bandwidth::network_bandwidth_filters::NetworkBandwidthFiltersDTO;

use crate::core::filter::{Filters, FiltersWrapper};


impl From<FiltersWrapper> for Filters {
    fn from(wrapper: FiltersWrapper) -> Self {
        Self { filters: wrapper.filter.split(";").map(|applied_filter| serde_json::from_str(applied_filter).unwrap()).collect() }
    }
}

impl From<Filters> for NetworkBandwidthFiltersDTO {
    fn from(value: Filters) -> Self {
        todo!()
    }
}
