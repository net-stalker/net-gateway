#[async_trait::async_trait]
pub trait Authenticator {
    async fn authenticate(&self, token: &str) -> Result<(), &'static str>;
}
