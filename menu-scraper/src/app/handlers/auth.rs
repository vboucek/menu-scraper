use crate::app::errors::ApiError;
use crate::app::forms::login::LoginFormData;
use crate::app::templates::login::LoginTemplate;
use crate::app::utils::password::verify_password;
use crate::app::utils::validation::Validation;
use crate::app::view_models::signed_user::SignedUser;
use actix_identity::Identity;
use actix_session::Session;
use actix_web::web::Data;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use askama::Template;
use db::db::models::UserLogin;
use db::db::repositories::{GetUserByEmail, UserRepository};

pub fn auth_config(config: &mut web::ServiceConfig) {
    config
        .service(web::resource("/logout").route(web::get().to(logout)))
        .service(
            web::resource("/login")
                .route(web::get().to(get_login))
                .route(web::post().to(post_login)),
        );
}

/// Gets empty login form
async fn get_login(user: Option<Identity>) -> Result<HttpResponse, ApiError> {
    if user.is_some() {
        // Already signed in, redirect to main page
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/"))
            .finish());
    }

    let template = LoginTemplate {};

    let body = template.render().map_err(ApiError::from)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

/// Submits login form
async fn post_login(
    form: web::Form<LoginFormData>,
    user_repo: Data<UserRepository>,
    request: HttpRequest,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    // Check inputs
    form.validate()?;

    // Get user by email
    let user = user_repo
        .login(&UserLogin {
            email: form.email.clone(),
        })
        .await
        // Map error to same error to reduce info retrieved (whether email or password is wrong)
        .map_err(|_| ApiError::BannerError("Chybný email nebo heslo.".to_string()))?;

    // Check if password match
    verify_password(form.password.as_ref(), &user.password_hash)
        .map_err(|_| ApiError::BannerError("Chybný email nebo heslo.".to_string()))?;

    // Login user
    Identity::login(&request.extensions(), String::from(user.id)).map_err(|_| ApiError::BannerErrorDefault)?;
    session.insert(
        "signed_user",
        SignedUser {
            username: user.username,
            profile_picture: user.profile_picture,
        },
    ).map_err(|_| ApiError::BannerErrorDefault)?;

    Ok(HttpResponse::Ok()
        .append_header(("HX-Redirect", "/"))
        .finish())
}

/// Signs out user
async fn logout(user: Identity) -> Result<HttpResponse, ApiError> {
    user.logout();

    Ok(HttpResponse::Found()
        .append_header(("Location", "/"))
        .finish())
}
