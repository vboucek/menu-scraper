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

/// User facing error types
/// Api error
#[derive(Error, Debug, Serialize)]
pub enum ApiError {
    InternalServerError,
    NotFound,
    BadRequest,
}

/// User facing error type
/// Htmx error (returns error banner)
#[derive(Error, Debug, Serialize)]
pub enum HtmxError {
    // Returns error banner
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
        }
    }
}

impl fmt::Display for HtmxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HtmxError::BannerError(message) => write!(f, "{}", message),
            HtmxError::BannerErrorDefault => {
                write!(f, "Interní chyba serveru, zkuste to prosím později.")
            }
        }
    }
}

impl From<DbError> for ApiError {
    fn from(_: DbError) -> Self {
        ApiError::InternalServerError
    }
}

impl From<DbError> for HtmxError {
    fn from(err: DbError) -> Self {
        match &err.error_type {
            // Business logic error, return error banner
            BusinessLogic(_) => HtmxError::BannerError(err.to_string()),
            // Database error, return only internal server error, not presenting any details about error
            _ => HtmxError::BannerErrorDefault,
        }
    }
}

impl From<password_hash::Error> for HtmxError {
    fn from(_: password_hash::Error) -> Self {
        HtmxError::BannerErrorDefault
    }
}

impl From<anyhow::Error> for HtmxError {
    fn from(err: anyhow::Error) -> Self {
        HtmxError::BannerError(err.to_string())
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(_: anyhow::Error) -> Self {
        ApiError::InternalServerError
    }
}

impl From<askama::Error> for ApiError {
    fn from(_: askama::Error) -> Self {
        ApiError::InternalServerError
    }
}

impl From<askama::Error> for HtmxError {
    fn from(_: askama::Error) -> Self {
        HtmxError::BannerErrorDefault
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
        match self {
            ApiError::InternalServerError => HttpResponse::build(self.status_code()).finish(),
            ApiError::NotFound => HttpResponse::build(self.status_code()).finish(),
            ApiError::BadRequest => HttpResponse::build(self.status_code()).finish(),
        }
    }
}

impl error::ResponseError for HtmxError {
    fn status_code(&self) -> StatusCode {
        // Htmx requires OK, otherwise wont print the error banner
        match *self {
            HtmxError::BannerError(_) => StatusCode::OK,
            HtmxError::BannerErrorDefault => StatusCode::OK,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            HtmxError::BannerError(_) => {
                handle_error_template(self.to_string(), self.status_code())
            }
            HtmxError::BannerErrorDefault => {
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
