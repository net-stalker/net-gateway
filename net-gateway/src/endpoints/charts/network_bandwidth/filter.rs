use net_reporter_api::api::network_bandwidth::network_bandwidth_filters::NetworkBandwidthFiltersDTO;

use crate::core::filter::Filters;

impl From<Filters> for NetworkBandwidthFiltersDTO {
    fn from(value: Filters) -> Self {
        let mut protocols: Vec<String> = Vec::new();
        let mut protocols_mode: Option<bool> = None;

        let mut endpoints: Vec<String> = Vec::new();
        let mut endpoints_mode: Option<bool> = None;

        for filter in value.filters {
            match filter.filter_entity.as_str() {
                "protocol" => {
                    protocols_mode = filter.get_mode();
                    protocols.push(filter.filter_value);
                },
                "endpoint" => {
                    endpoints_mode = filter.get_mode();
                    endpoints.push(filter.filter_value);
                },
                _ => { /* do nothing club */ }
            }
        }

        Self::new(&protocols, protocols_mode, &endpoints, endpoints_mode)
    }
}
