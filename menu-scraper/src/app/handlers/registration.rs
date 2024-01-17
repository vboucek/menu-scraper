use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_web::{error::ErrorInternalServerError, HttpResponse, Result as ActixResult, web};
use anyhow::Error;
use askama::Template;
use crate::app::templates::registration::RegistrationTemplate;
use crate::app::utils::error::handle_error_template;
use crate::app::utils::picture::validate_and_save_picture;
use crate::app::utils::validation::Validation;


pub fn registration_config(config: &mut web::ServiceConfig) {
    config
        .service(
            web::resource("/registration")
                .route(web::get().to(get_registration))
                .route(web::post().to(post_registration))
        );
}

/// Gets empty registration form
async fn get_registration() -> ActixResult<HttpResponse> {
    let template = RegistrationTemplate {};

    let body = template.render().map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

/// Multipart form data to upload profile picture
#[derive(MultipartForm, Debug)]
struct RegistrationFormData {
    username: Text<String>,
    password: Text<String>,
    email: Text<String>,
    #[multipart(rename = "profile-picture")]
    file: Option<TempFile>,
}

impl Validation for RegistrationFormData {
    fn validate(&self) -> Result<(), Error> {
        if self.username.is_empty() {
            return Err(anyhow::anyhow!("Username cannot be empty."));
        }

        self.is_valid_email(&self.email)?;

        if self.password.len() < 12 {
            return Err(anyhow::anyhow!("Password must be at least 12 characters long."));
        }

        Ok(())
    }
}

/// Register the user
async fn post_registration(
    MultipartForm(mut form): MultipartForm<RegistrationFormData>,
) -> ActixResult<HttpResponse> {
    // Check inputs
    if let Err(err) = form.validate() {
        return handle_error_template(err);
    }

    // Handle picture
    if let Some(file) = form.file {
        if let Err(err) = validate_and_save_picture(file).await {
            return handle_error_template(err);
        }
    }

    // Redirect to main page if everything went well
    Ok(HttpResponse::Ok()
        .append_header(("HX-Redirect", "/"))
        .finish())
}


