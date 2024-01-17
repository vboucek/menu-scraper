use std::sync::Mutex;
use actix_identity::Identity;
use actix_session::Session;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, web};
use actix_web::web::Data;
use anyhow::anyhow;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use askama::Template;
use db::db::common::DbReadOne;
use db::db::models::UserLogin;
use db::db::repositories::UserRepository;
use crate::app::errors::ApiError;
use crate::app::forms::login::LoginFormData;
use crate::app::handlers::error::{handle_error_template};
use crate::app::templates::login::LoginTemplate;
use crate::app::utils::validation::Validation;
use crate::app::view_models::signed_user::SignedUser;

pub fn auth_config(config: &mut web::ServiceConfig) {
    config
        .service(web::resource("/logout")
            .route(web::get().to(logout)))
        .service(web::resource("/login")
            .route(web::get().to(get_login))
            .route(web::post().to(post_login)));
}

/// Gets empty login form
async fn get_login(user: Option<Identity>) -> Result<HttpResponse, ApiError> {
    if user.is_some() {
        // Already signed in, redirect to main page
        return Ok(HttpResponse::Found().append_header(("Location", "/")).finish());
    }

    let template = LoginTemplate {};

    let body = template.render().map_err(ApiError::from)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

/// Submits login form
async fn post_login(form: web::Form<LoginFormData>, user_repo: Data<Mutex<UserRepository>>,
                    request: HttpRequest, session: Session) -> Result<HttpResponse, ApiError> {
    // Check inputs
    if let Err(err) = form.validate() {
        return handle_error_template(err);
    }

    // Get user by email
    let result = user_repo.lock().unwrap().read_one(&UserLogin {
        email: form.email.clone(),
    }).await;

    match result.map_err(|_| anyhow!("Chybný email nebo heslo.")) {
        Ok(user) => {
            // Check if passwords match
            let parsed_hash = PasswordHash::new(&user.password_hash)?;
            let password_match = Argon2::default().verify_password(form.password.as_ref(), &parsed_hash);

            if let Err(err) = password_match.map_err(|_| anyhow!("Chybný email nebo heslo.")) {
                return handle_error_template(err);
            }

            // Login user
            Identity::login(&request.extensions(), String::from(user.id)).map_err(ApiError::from)?;
            session.insert("signed_user", SignedUser {
                username: user.username,
                profile_picture: user.profile_picture,
            })?;
        }
        Err(err) => {
            return handle_error_template(err);
        }
    };

    Ok(HttpResponse::Ok()
        .append_header(("HX-Redirect", "/"))
        .finish())
}

/// Signs out user
async fn logout(user: Identity) -> Result<HttpResponse, ApiError> {
    user.logout();

    Ok(HttpResponse::Found().append_header(("Location", "/")).finish())
}