use actix_web::{HttpResponse, HttpRequest, http::header};
pub mod mock_authenticator;

/// Authorizes the request by checking the presence and validity of the authorization token.
/// Returns `Ok(())` if the token is valid, otherwise returns an `Err` with an `HttpResponse` indicating the reason for authorization failure.
pub async fn authorize(req: HttpRequest, verifier: impl net_token_verifier::verifier::Verifier) -> Result<String, HttpResponse> {
	let header = if let Some(header) = req.headers().get(header::AUTHORIZATION) {
		header
	} else {
		return Err(HttpResponse::Unauthorized().body("Unauthorized: No Authorization header"));
	};
	let auth_str = header.to_str().unwrap_or_default();
	let token = if let Some(token) = auth_str.strip_prefix("Bearer ") {
		token
	} else {
		return Err(HttpResponse::Unauthorized().body("Unauthorized: Bearer token expected"));
	};
	match verifier.verify_token(token).await {
		Ok(_) => Ok(token.to_string()),
		Err(message) => Err(HttpResponse::Unauthorized().body(format!("Unauthorized: {}", message))),
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use actix_web::{test, rt::pin, body::MessageBody, web};
	use futures::future;
    use mock_authenticator::MockAuthenticator;

	
	#[actix_web::test]
	async fn test_authorize_valid_token() {
		let req = test::TestRequest::default()
			.append_header((header::AUTHORIZATION, "Bearer valid_token"))
			.to_http_request();

		let result = authorize(req, MockAuthenticator {}).await;

		assert!(result.is_ok());
	}

    #[actix_web::test]
	async fn test_authorize_valid_token_with_other_headers() {
        let req = test::TestRequest::default()
			.append_header((header::AUTHORIZATION, "Bearer valid_token"))
			.append_header((header::USER_AGENT, "user agent"))
			.to_http_request();

		let result = authorize(req, MockAuthenticator {}).await;

		assert!(result.is_ok());
	}

	#[actix_web::test]
	async fn test_authorize_missing_header() {
		let req = test::TestRequest::default().to_http_request();

		let result = authorize(req, MockAuthenticator {}).await;

		assert!(result.is_err());
		let response = result.unwrap_err();
		assert_eq!(response.status(), actix_web::http::StatusCode::UNAUTHORIZED);
		
		let body = response.into_body();
        pin!(body);
		let bytes = future::poll_fn(|cx| body.as_mut().poll_next(cx)).await;
        assert_eq!(
            bytes.unwrap().unwrap(),
            web::Bytes::from_static(b"Unauthorized: No Authorization header")
        );
	}

	#[actix_web::test]
	async fn test_authorize_invalid_token() {
		let req = test::TestRequest::default()
			.append_header((header::AUTHORIZATION, "Bearer invalid_token"))
			.to_http_request();

		let result = authorize(req, MockAuthenticator {}).await;

		assert!(result.is_err());
		let response = result.unwrap_err();
		assert_eq!(response.status(), actix_web::http::StatusCode::UNAUTHORIZED);
		
		let body = response.into_body();
        pin!(body);
		let bytes = future::poll_fn(|cx| body.as_mut().poll_next(cx)).await;
        assert_eq!(
            bytes.unwrap().unwrap(),
            web::Bytes::from_static(b"Unauthorized: Invalid token")
        );
	}

	#[actix_web::test]
	async fn test_authorize_missing_bearer_token() {
		let req = test::TestRequest::default()
			.append_header((header::AUTHORIZATION, "invalid_token"))
			.to_http_request();

        let result = authorize(req, MockAuthenticator {}).await;
		
        assert!(result.is_err());
		let response = result.unwrap_err();
		assert_eq!(response.status(), actix_web::http::StatusCode::UNAUTHORIZED);

		let body = response.into_body();
        pin!(body);
		let bytes = future::poll_fn(|cx| body.as_mut().poll_next(cx)).await;
        assert_eq!(
            bytes.unwrap().unwrap(),
            web::Bytes::from_static(b"Unauthorized: Bearer token expected")
        );
	}
}
