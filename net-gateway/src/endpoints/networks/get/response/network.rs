use net_reporter_api::api::network::network::NetworkDTO;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Network {
    pub id: String,
    pub name: String,
    pub color: String,
}


impl From<&NetworkDTO> for Network {
    fn from(value: &NetworkDTO) -> Self {
        Self {
            id: value.get_id().to_string(),
            name: value.get_name().to_string(),
            color: value.get_color().to_string(),
        }
    }
}
