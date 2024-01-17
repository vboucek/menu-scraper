use anyhow::Error;
use actix_web::{error::ErrorInternalServerError, HttpResponse, Result as ActixResult};
use crate::app::templates::error::ErrorBannerTemplate;
use askama::Template;

/// Returns error banner with given error.
pub fn handle_error_template(err: Error) -> ActixResult<HttpResponse> {
    let template = ErrorBannerTemplate { error: err.to_string() };
    let body = template.render().map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
