use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate, Clone)]
pub struct ClientData {
    #[serde(rename = "groupId")]
    pub group_id: String
}