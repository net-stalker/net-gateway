use super::authenticator;

pub struct MockAuthenticator {}

#[async_trait::async_trait]
impl authenticator::Authenticator for MockAuthenticator {
  async fn authenticate(&self, token: &str) -> Result<(), &'static str> {
    if token == "valid_token" {
      Ok(())
    } else {
      Err("Invalid token")
    }
  }
}
