use net_reporter_api::api::http_responses_dist::http_responses_filters::HttpResponsesDistFiltersDTO;
use crate::core::filter::Filters;

impl From<Filters> for HttpResponsesDistFiltersDTO {
    fn from(value: Filters) -> Self {
        let mut endpoints: Vec<String> = Vec::new();
        let mut endpoints_mode: Option<bool> = None;

        let mut bytes_lower_bound: Option<i64> = None;
        let mut bytes_upper_bound: Option<i64> = None;

        for filter in value.filters {
            match filter.filter_entity.as_str() {
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
            &endpoints,
            endpoints_mode,
            bytes_lower_bound,
            bytes_upper_bound,
        )
    }
}
