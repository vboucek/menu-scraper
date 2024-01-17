use actix_web::http::StatusCode;
use actix_web::{error, HttpResponse};
use argon2::password_hash;
use db::db::common::error::DbError;
use serde::Serialize;
use thiserror::Error;

/// User facing error type
#[derive(Error, Debug, Serialize)]
pub enum ApiError {
    #[error("internal server error")]
    InternalServerError,
    #[error("not found")]
    NotFound,
    #[error("bad request")]
    BadRequest,
}

impl From<DbError> for ApiError {
    fn from(_: DbError) -> Self {
        ApiError::InternalServerError
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(_: serde_json::Error) -> Self {
        ApiError::InternalServerError
    }
}

impl From<askama::Error> for ApiError {
    fn from(_: askama::Error) -> Self {
        ApiError::InternalServerError
    }
}

impl From<password_hash::Error> for ApiError {
    fn from(_: password_hash::Error) -> Self {
        ApiError::InternalServerError
    }
}

impl From<actix_identity::error::LoginError> for ApiError {
    fn from(_: actix_identity::error::LoginError) -> Self {
        ApiError::InternalServerError
    }
}

impl From<actix_session::SessionGetError> for ApiError {
    fn from(_: actix_session::SessionGetError) -> Self {
        ApiError::InternalServerError
    }
}

impl From<actix_session::SessionInsertError> for ApiError {
    fn from(_: actix_session::SessionInsertError) -> Self {
        ApiError::InternalServerError
    }
}

impl error::ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match *self {
            ApiError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::BadRequest => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).finish()
    }
}
