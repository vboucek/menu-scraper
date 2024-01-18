use crate::app::errors::{HtmxError};
use crate::app::forms::registration::RegistrationFormData;
use crate::app::utils::password::hash_password;
use crate::app::utils::picture::validate_and_save_picture;
use crate::app::utils::validation::Validation;
use crate::app::view_models::signed_user::SignedUser;
use actix_identity::Identity;
use actix_multipart::form::MultipartForm;
use actix_session::Session;
use actix_web::web::Data;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use db::db::common::DbCreate;
use db::db::models::{CheckEmailAndUsername, UserCreate};
use db::db::repositories::{UserCheckEmailAndPassword, UserRepository};

pub fn user_config(config: &mut web::ServiceConfig) {
    config.service(
        web::resource("/user")
            //.route(web::put().to(put_user))
            .route(web::post().to(post_user)),
    );
}

/// Register the user
async fn post_user(
    MultipartForm(form): MultipartForm<RegistrationFormData>,
    user_repo: Data<UserRepository>,
    request: HttpRequest,
    session: Session,
) -> Result<HttpResponse, HtmxError> {
    // Check inputs
    form.validate()?;

    // Check email and username availability
    user_repo
        .check_email_and_password(&CheckEmailAndUsername {
            edited_user_id: None,
            username: form.username.to_owned(),
            email: form.email.to_owned(),
        })
        .await?;

    // Handle picture
    let profile_pic = if let Some(file) = form.file {
        Some(validate_and_save_picture(file).await?)
    } else {
        None
    };

    // Generate password hash
    let password_hash = hash_password(form.password.0.as_ref())
        .map_err(HtmxError::from)?
        .to_string();

    // Store user
    let user = user_repo
        .create(&UserCreate {
            username: form.username.to_owned(),
            email: form.email.to_owned(),
            profile_picture: profile_pic,
            password_hash,
        })
        .await?;

    // Sign in registered user
    Identity::login(&request.extensions(), String::from(user.id))
        .map_err(|_| HtmxError::BannerErrorDefault)?;
    session
        .insert(
            "signed_user",
            SignedUser {
                username: user.username,
                profile_picture: user.profile_picture,
            },
        )
        .map_err(|_| HtmxError::BannerErrorDefault)?;

    // Redirect to main page if everything went well
    Ok(HttpResponse::Ok()
        .append_header(("HX-Redirect", "/"))
        .finish())
}
