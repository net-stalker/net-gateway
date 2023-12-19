use async_trait::async_trait;

#[async_trait]
pub trait Handler: Send + Sync {
    // async fn handle(&self) -> Vec<u8>;
    async fn gun(&self);
}