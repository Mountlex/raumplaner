use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::fmt;

#[derive(Debug)]
pub enum CustomError {
    FaultySetup(String),
    Database(String),
    Authorization(String),
}

// Allow the use of "{}" format specifier
impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CustomError::FaultySetup(ref cause) => write!(f, "Setup Error: {cause}"),
            //CustomError::Unauthorized(ref cause) => write!(f, "Setup Error: {}", cause),
            CustomError::Database(ref cause) => {
                write!(f, "Database Error: {cause}")
            }
            CustomError::Authorization(ref err) => write!(f, "Authorization Error: {err}"),
        }
    }
}

// So that errors get printed to the browser?
impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            CustomError::Database(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
            CustomError::FaultySetup(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
            CustomError::Authorization(message) => (StatusCode::FORBIDDEN, message),
        };

        format!("status = {status}, message = {error_message}").into_response()
    }
}

impl From<axum::http::uri::InvalidUri> for CustomError {
    fn from(err: axum::http::uri::InvalidUri) -> CustomError {
        CustomError::FaultySetup(err.to_string())
    }
}

impl From<sea_orm::DbErr> for CustomError {
    fn from(err: sea_orm::DbErr) -> CustomError {
        CustomError::Database(err.to_string())
    }
}