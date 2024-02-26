use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FiltersWrapper {
    pub filter: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Filter {
    mode: Option<String>,
    #[serde(rename = "filterEntity")]
    pub filter_entity: String,
    #[serde(rename = "filterValue")]
    pub filter_value: String,
}

impl Filter {
    pub fn get_mode(&self) -> Option<bool> {
        match self.mode.as_ref() {
            Some(value) => {
                match value.as_str() {
                    "include" => Some(true),
                    "exclude" => Some(false),
                    _ => None
                }
            },
            _ => None
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Filters {
    pub filters: Vec<Filter>,
}

impl From<FiltersWrapper> for Filters {
    fn from(wrapper: FiltersWrapper) -> Self {
        match wrapper.filter.is_empty() {
            true => Self { filters: vec![] },
            false => Self { filters: wrapper.filter.split(';').map(|applied_filter| serde_json::from_str(applied_filter).unwrap()).collect() }
        }
    }
}
