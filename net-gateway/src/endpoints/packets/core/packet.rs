use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Packet {
    pub id: String,
    #[serde(rename = "frameTime")]
    pub frame_time: Option<u64>,
    pub src: Option<String>,
    pub dst: Option<String>,
    pub protocols: Option<String>,
    pub json: Option<String>,
}

impl Packet {
    pub fn from_id(id: String) -> Self {
        Self {
            id,
            frame_time: None,
            src: None,
            dst: None,
            protocols: None,
            json: None,
        }
    }
}
