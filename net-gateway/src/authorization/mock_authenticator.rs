pub struct MockAuthenticator {}

#[async_trait::async_trait]
impl net_token_verifier::verifier::Verifier for MockAuthenticator {
    type R = ();
    async fn verify_token(&self, token: &str) -> Result<Self::R, String> {
        if token == "valid_token" {
            Ok(())
        } else {
            Err("Invalid token".to_string())
        }
    }
}
