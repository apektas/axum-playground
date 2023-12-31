use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use strum_macros::AsRefStr;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, AsRefStr)]
pub enum Error {
    LoginFail,

    // Auth errors
    AuthFailCtxNotInRequestExtension,

    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,

    // Model Errors
    TicketDeleteFailIdNotFound{id: u64},
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES" );

        // Create a placeholder Axum response.
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();


        response.extensions_mut().insert(self);
        response

    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            Error::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),
            // Auth
            Error::AuthFailNoAuthTokenCookie
            | Error::AuthFailCtxNotInRequestExtension
            | Error::AuthFailTokenWrongFormat => {
                (StatusCode::FORBIDDEN, ClientError::NO_AUTH)
            }
            //Model
            Error::TicketDeleteFailIdNotFound { .. } => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            }
            // can be useful to have
            #[allow(unreachable_patterns)]
            _ => (
            	StatusCode::INTERNAL_SERVER_ERROR,
            	ClientError::SERVICE_ERROR,
            ),
        }
    }
}

#[derive(Debug, AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}