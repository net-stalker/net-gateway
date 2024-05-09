use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum UserFacingError {
    #[display(fmt = "An internal error occurred. Please try again later.")]
    InternalError,
    #[display(fmt = "An internal error occurred. Please try again later: {}", _0)]
    InternalErrorWithDescription(String),
    #[display(fmt = "Took too long to respond")]
    Timeout,
    #[display(fmt = "User is unauthorized. Check your credentials")]
    Unauthorized,
}

impl error::ResponseError for UserFacingError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            UserFacingError::InternalError | UserFacingError::InternalErrorWithDescription(_) => StatusCode::INTERNAL_SERVER_ERROR,
            UserFacingError::Timeout => StatusCode::GATEWAY_TIMEOUT,
            UserFacingError::Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }
}
