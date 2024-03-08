use net_reporter_api::api::http_responses::http_responses_filters::HttpResponsesFiltersDTO;
use crate::core::filter::Filters;

impl From<Filters> for HttpResponsesFiltersDTO {

    fn from(value: Filters) -> Self {
        let mut http_responses: Vec<i64> = Vec::new();
        let mut http_response_mode: Option<bool> = None;

        let mut endpoints: Vec<String> = Vec::new();
        let mut endpoints_mode: Option<bool> = None;

        let mut bytes_lower_bound: Option<i64> = None;
        let mut bytes_upper_bound: Option<i64> = None;

        for filter in value.filters {
            match filter.filter_entity.as_str() {
                "http_response" => {
                    http_response_mode = filter.get_mode();
                    http_responses.push(filter.filter_value.parse().unwrap());
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
            &http_responses,
            http_response_mode,
            &endpoints,
            endpoints_mode,
            bytes_lower_bound,
            bytes_upper_bound,
        )
    }
}
