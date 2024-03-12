use net_reporter_api::api::http_clients::http_clients_filters::HttpClientsFiltersDTO;
use crate::core::filter::Filters;

impl From<Filters> for HttpClientsFiltersDTO {

    fn from(value: Filters) -> Self {
        let mut http_methods: Vec<String> = Vec::new();
        let mut http_methods_mode: Option<bool> = None;

        let mut endpoints: Vec<String> = Vec::new();
        let mut endpoints_mode: Option<bool> = None;

        let mut bytes_lower_bound: Option<i64> = None;
        let mut bytes_upper_bound: Option<i64> = None;

        for filter in value.filters {
            match filter.filter_entity.as_str() {
                "http_method" => {
                    http_methods_mode = filter.get_mode();
                    http_methods.push(filter.filter_value);
                },
                "endpoint" => {
                    endpoints_mode = filter.get_mode();
                    endpoints.push(filter.filter_value);
                },
                "bytes" => {
                    match filter.mode {
                        Some(mode) => {
                            match mode.as_str() {
                                "<" => bytes_upper_bound = Some(filter.filter_value.parse::<i64>().unwrap()),
                                ">" => bytes_lower_bound = Some(filter.filter_value.parse::<i64>().unwrap()),
                                _ => { /* do nothing club */ }
                            }
                        },
                        _ => { /* do nothing club */ }
                    }
                },
                _ => { /* do nothing club */ }
            }
        }

        Self::new(
            &http_methods,
            http_methods_mode,
            &endpoints,
            endpoints_mode,
            bytes_lower_bound,
            bytes_upper_bound,
        )
    }
}
