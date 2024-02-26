use net_reporter_api::api::network_bandwidth_per_protocol::network_bandwidth_per_protocol_filters::NetworkBandwidthPerProtocolFiltersDTO;
use crate::core::filter::Filters;
use std::num::ParseIntError;
use regex::Regex;


#[derive(Debug)]
enum ParseBytesFilterError {
    RegexMatchFailed,
    ParseIntError(ParseIntError),
}

impl From<ParseIntError> for ParseBytesFilterError {
    fn from(error: ParseIntError) -> Self {
        ParseBytesFilterError::ParseIntError(error)
    }
}

// TODO: think of adding traits for each fitler type which are possbiel for charts
// this will eliminate the need of code duplication
fn parse_bytes_filter(bytes: &str) -> Result<(String, i64), ParseBytesFilterError> {
    let re = Regex::new(r"([<>])\s*(\d+)").unwrap();
    let caps = re.captures(bytes).ok_or(ParseBytesFilterError::RegexMatchFailed)?;

    let sign: String = caps[1].to_string();
    let number = caps[2].parse::<i64>()?;

    Ok((sign, number))
}

impl From<Filters> for NetworkBandwidthPerProtocolFiltersDTO {

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
                    let parse_res = parse_bytes_filter(&filter.filter_value);
                    match parse_res {
                        Ok((sign, number)) => {
                            match sign.as_str() {
                                "<" => bytes_upper_bound = Some(number),
                                ">" => bytes_lower_bound = Some(number),
                                _ => { /* do nothing club */ }
                            }
                        }
                        _ => { /* do nothing club */ }
                    }
                }
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
