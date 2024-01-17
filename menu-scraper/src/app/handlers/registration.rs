use std::sync::Mutex;
use actix_identity::Identity;
use actix_multipart::form::MultipartForm;
use actix_session::Session;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, web};
use actix_web::web::Data;
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use askama::Template;
use db::db::common::DbCreate;
use db::db::models::{CheckEmailAndUsername, UserCreate};
use db::db::repositories::{UserCheckEmailAndPassword, UserRepository};
use crate::app::errors::ApiError;
use crate::app::forms::registration::RegistrationFormData;
use crate::app::templates::registration::RegistrationTemplate;
use crate::app::handlers::error::{handle_db_error_template, handle_error_template};
use crate::app::utils::picture::validate_and_save_picture;
use crate::app::utils::validation::Validation;
use crate::app::view_models::signed_user::SignedUser;


pub fn registration_config(config: &mut web::ServiceConfig) {
    config
        .service(
            web::resource("/registration")
                .route(web::get().to(get_registration))
                .route(web::post().to(post_registration))
        );
}

/// Gets empty registration form
async fn get_registration(user: Option<Identity>) -> Result<HttpResponse, ApiError> {
    if user.is_some() {
        // Already signed in, redirect to main page
        return Ok(HttpResponse::Found().append_header(("Location", "/")).finish());
    }

    let template = RegistrationTemplate {};

    let body = template.render().map_err(ApiError::from)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

/// Register the user
async fn post_registration(
    MultipartForm(form): MultipartForm<RegistrationFormData>,
    user_repo: Data<Mutex<UserRepository>>,
    request: HttpRequest,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    // Check inputs
    if let Err(err) = form.validate() {
        return handle_error_template(err);
    }

    // Check email and username availability
    let result = user_repo.lock().unwrap().check_email_and_password(&CheckEmailAndUsername {
        edited_user_id: None,
        username: form.username.to_owned(),
        email: form.email.to_owned(),
    }).await;

    if let Err(err) = result {
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

    // Generate password hash
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default().hash_password(form.password.0.as_ref(), &salt).map_err(ApiError::from)?.to_string();

    // Store user
    let result = user_repo.lock().unwrap().create(&UserCreate {
        username: form.username.to_owned(),
        email: form.email.to_owned(),
        profile_picture: profile_pic,
        password_hash,
    }).await;

    // Sign in registered user
    match result {
        Ok(user) => {
            Identity::login(&request.extensions(), String::from(user.id)).map_err(ApiError::from)?;
            session.insert("signed_user", SignedUser {
                username: user.username,
                profile_picture: user.profile_picture,
            })?;
        }
        Err(err) => {
            return handle_db_error_template(err);
        }
    }

    // Redirect to main page if everything went well
    Ok(HttpResponse::Ok()
        .append_header(("HX-Redirect", "/"))
        .finish())
}


