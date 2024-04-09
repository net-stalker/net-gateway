use actix_web::http::header;
use actix_web::HttpRequest;
use actix_web::HttpResponse;

use net_token_verifier::core::verifier::token::Token;
use net_token_verifier::core::verifier::Verifier;

/// Authorizes the request by checking the presence and validity of the authorization token.
/// Returns `Ok(Box<dyn net_token_verifier::core::verifier::token::Token>)` if the token is valid, otherwise returns an `Err` with an `HttpResponse` indicating the reason for authorization failure.
pub async fn authorize(
	req: HttpRequest,
	verifier: Box<dyn Verifier>
) -> Result<Box<dyn Token>, HttpResponse> {
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
		Ok(token) => Ok(token),
		Err(message) => Err(HttpResponse::Unauthorized().body(format!("Unauthorized: {}", message))),
	}
}