use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Network {
    id: Option<String>,
    name: String,
    color: String,
}
