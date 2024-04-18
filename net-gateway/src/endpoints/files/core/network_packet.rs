use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;


// looks like a complete data strucutre which we need to send back to the client
#[derive(Debug, Clone, Deserialize)]
pub struct NetworkPacket {
    pub id: i64,
    // a DateTime<Utc> into timestamp format
    pub frametime: DateTime<Utc>,
    pub src: String,
    pub dst: String,
    pub protocols: Vec<String>,
    pub binary_data: serde_json::Value,
}
