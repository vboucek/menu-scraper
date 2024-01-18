use crate::app::errors::ApiError;
use crate::app::templates::registration::RegistrationTemplate;
use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use askama::Template;

pub fn registration_config(config: &mut web::ServiceConfig) {
    config.service(
        web::resource("/registration")
            .route(web::get().to(get_registration))
    );
}

/// Gets empty registration form
async fn get_registration(user: Option<Identity>) -> Result<HttpResponse, ApiError> {
    if user.is_some() {
        // Already signed in, redirect to main page
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/"))
            .finish());
    }

    let template = RegistrationTemplate {};

    let body = template.render().map_err(ApiError::from)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
