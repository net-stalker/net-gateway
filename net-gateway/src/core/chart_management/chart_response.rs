use std::fmt::Debug;

pub trait ChartResponse : Debug + Send + Sync {
    fn get_dto_type(&self) -> &'static str;
    fn get_json_value(&self) -> serde_json::Value;
    fn get_json_type(&self) -> &'static str;
}