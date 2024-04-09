use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Packet {
    pub id: String,
    #[serde(rename = "frameTime")]
    pub frame_time: u64,
    pub src: Option<String>,
    pub dst: Option<String>,
    pub protocols: Option<String>,
    pub json: Option<String>,
}
