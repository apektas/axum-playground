use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFail,

    // Auth errors
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,

    // Model Errors
    TicketDeleteFailIdNotFound{id: u64},
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES" );
        (StatusCode::INTERNAL_SERVER_ERROR, "Un handled client error").into_response()
    }
}