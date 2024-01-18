use crate::app::templates::error::ErrorBannerTemplate;
use actix_web::error;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use argon2::password_hash;
use askama::Template;
use db::db::common::error::DbError;
use db::db::common::error::DbErrorType::BusinessLogic;
use serde::Serialize;
use std::fmt;
use thiserror::Error;

/// User facing error type
#[derive(Error, Debug, Serialize)]
pub enum ApiError {
    InternalServerError,
    NotFound,
    BadRequest,
    // Returns error banner (suitable for htmx response)
    BannerError(String),
    // Returns error banner with default error
    BannerErrorDefault,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::InternalServerError => write!(f, "Internal Server Error"),
            ApiError::NotFound => write!(f, "Not found"),
            ApiError::BadRequest => write!(f, "Bad request"),
            ApiError::BannerError(message) => write!(f, "{}", message),
            ApiError::BannerErrorDefault => {
                write!(f, "Interní chyba serveru, zkuste to prosím později.")
            }
        }
    }
}

// DbError is implicitly mapped to banner error to reduce code replication, otherwise map_err must be used!
impl From<DbError> for ApiError {
    fn from(err: DbError) -> Self {
        match &err.error_type {
            // Business logic error, return error banner
            BusinessLogic(_) => ApiError::BannerError(err.to_string()),
            // Database error, return only internal server error, not presenting any details about error
            _ => ApiError::BannerErrorDefault,
        }
    }
}

// password_hash::Error is implicitly mapped to banner error, otherwise map_err must be used
impl From<password_hash::Error> for ApiError {
    fn from(_: password_hash::Error) -> Self {
        ApiError::BannerErrorDefault
    }
}

// password_hash::Error is implicitly mapped to banner error, otherwise map_err must be used
impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        ApiError::BannerError(err.to_string())
    }
}

// askama error is implicitly mapped to just internal server error, otherwise map_err must be used
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
            // Htmx requires OK, otherwise wont print the error banner
            ApiError::BannerError(_) => StatusCode::OK,
            ApiError::BannerErrorDefault => StatusCode::OK,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::InternalServerError => HttpResponse::build(self.status_code()).finish(),
            ApiError::NotFound => HttpResponse::build(self.status_code()).finish(),
            ApiError::BadRequest => HttpResponse::build(self.status_code()).finish(),
            ApiError::BannerError(_) => {
                handle_error_template(self.to_string(), self.status_code())
            }
            ApiError::BannerErrorDefault => {
                handle_error_template(self.to_string(), self.status_code())
            }
        }
    }
}

pub fn handle_error_template(err: String, code: StatusCode) -> HttpResponse {
    let template = ErrorBannerTemplate {
        error: err.to_string(),
    };

    if let Ok(body) = template.render() {
        return HttpResponse::build(code.clone())
            .content_type("text/html")
            .body(body);
    }

    HttpResponse::build(code.clone())
        .content_type("text/html")
        .body(err.to_string())
}
