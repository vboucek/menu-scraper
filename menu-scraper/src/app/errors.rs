use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{error, HttpResponse};
use serde::Serialize;
use serde_json::json;
use thiserror::Error;
use db::db::common::error::DbError;


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
    fn from(error: DbError) -> Self {
        match error {
            DbError { .. } => { ApiError::InternalServerError }
        }
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

impl error::ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match *self {
            ApiError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::BadRequest => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(json!({ "error": self.to_string() }))
    }
}
