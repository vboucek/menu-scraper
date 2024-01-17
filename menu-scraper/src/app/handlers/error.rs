use crate::app::errors::ApiError;
use crate::app::templates::error::ErrorBannerTemplate;
use actix_web::HttpResponse;
use anyhow::Error;
use askama::Template;
use db::db::common::error::DbError;
use db::db::common::error::DbErrorType::BusinessLogic;

/// Returns error banner with given error.
pub fn handle_error_template(err: Error) -> Result<HttpResponse, ApiError> {
    let template = ErrorBannerTemplate {
        error: err.to_string(),
    };
    let body = template.render().map_err(ApiError::from)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

pub fn handle_db_error_template(err: DbError) -> Result<HttpResponse, ApiError> {
    match err.error_type {
        // Business logic error, return error banner
        BusinessLogic(_) => {
            let template = ErrorBannerTemplate {
                error: err.to_string(),
            };
            let body = template.render().map_err(ApiError::from)?;
            Ok(HttpResponse::Ok().content_type("text/html").body(body))
        }
        // Database error, return only internal server error, not presenting any details about error
        _ => Err(ApiError::from(err)),
    }
}
