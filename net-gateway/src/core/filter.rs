use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FiltersWrapper {
    pub filter: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Filter {
    pub mode: String,
    #[serde(rename = "filterEntity")]
    pub filter_entity: String,
    #[serde(rename = "filterValue")]
    pub filter_value: String,
}

#[derive(Debug, Clone, Default)]
pub struct Filters {
    pub filters: Vec<Filter>,
}

impl From<FiltersWrapper> for Filters {
    fn from(wrapper: FiltersWrapper) -> Self {
        Self { filters: wrapper.filter.split(";").map(|applied_filter| serde_json::from_str(applied_filter).unwrap()).collect() }
    }
}
