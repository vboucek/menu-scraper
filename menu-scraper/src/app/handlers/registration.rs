use std::sync::Mutex;
use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_web::{HttpResponse, web};
use actix_web::web::Data;
use anyhow::Error;
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use askama::Template;
use db::db::common::DbCreate;
use db::db::models::{CheckEmailAndUsername, User, UserCreate};
use db::db::repositories::{UserCheckEmailAndPassword, UserRepository};
use crate::app::errors::ApiError;
use crate::app::templates::registration::RegistrationTemplate;
use crate::app::handlers::error::{handle_db_error_template, handle_error_template};
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
async fn get_registration() -> Result<HttpResponse, ApiError> {
    let template = RegistrationTemplate {};

    let body = template.render().map_err(ApiError::from)?;

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
    MultipartForm(form): MultipartForm<RegistrationFormData>,
    user_repo: Data<Mutex<UserRepository>>,
) -> Result<HttpResponse, ApiError> {
    // Check inputs
    if let Err(err) = form.validate() {
        return handle_error_template(err);
    }

    // Check email and username availability
    let err = user_repo.lock().unwrap().check_email_and_password(&CheckEmailAndUsername {
        edited_user_id: None,
        username: form.username.to_owned(),
        email: form.email.to_owned(),
    }).await;

    if let Err(err) = err {
        return handle_db_error_template(err);
    }

    // Handle picture
    let profile_pic = if let Some(file) = form.file {
        let picture_result = validate_and_save_picture(file).await;

        match picture_result {
            Ok(picture) => { Some(picture) }
            Err(err) => { return handle_error_template(err); }
        }
    } else {
        None
    };

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default().hash_password(form.password.0.as_ref(), &salt)?.to_string();

    let result = user_repo.lock().unwrap().create(&UserCreate {
        username: form.username.to_owned(),
        email: form.email.to_owned(),
        profile_picture: profile_pic,
        password_hash
    }).await;

    // Redirect to main page if everything went well
    Ok(HttpResponse::Ok()
        .append_header(("HX-Redirect", "/"))
        .finish())
}


