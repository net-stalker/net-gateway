use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Network {
    pub id: Option<String>,
    pub name: String,
    pub color: String,
}
