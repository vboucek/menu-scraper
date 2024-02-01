use crate::app::errors::{ApiError, HtmxError};
use crate::app::forms::registration::RegistrationFormData;
use crate::app::forms::user_edit::UserEditFormData;
use crate::app::forms::user_search::UserSearchQuery;
use crate::app::templates::user_edit::UserEditTemplate;
use crate::app::templates::user_preview_list::UserPreviewList;
use crate::app::utils::password::{hash_password, verify_password};
use crate::app::utils::picture::validate_and_save_picture;
use crate::app::utils::validation::Validation;
use crate::app::view_models::signed_user::SignedUser;
use crate::app::view_models::user_edit::UserEdit;
use crate::app::view_models::user_preview::UserPreviewView;
use actix_identity::Identity;
use actix_multipart::form::MultipartForm;
use actix_session::Session;
use actix_web::web::Data;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use askama::Template;
use db::db::common::{DbCreate, DbReadMany, DbReadOne, DbUpdate};
use db::db::models::{
    CheckEmailAndUsername, UserCreate, UserGetById, UserGetByUsername, UserUpdate,
};
use db::db::repositories::{UserCheckEmailAndPassword, UserRepository};
use uuid::Uuid;

pub fn user_config(config: &mut web::ServiceConfig) {
    config
        .service(web::resource("/user-edit").route(web::get().to(get_user_edit_form)))
        .service(
            web::resource("/users")
                .route(web::put().to(put_user))
                .route(web::post().to(post_user))
                .route(web::get().to(get_users)),
        );
}

/// Get form for editing user's details
async fn get_user_edit_form(
    user: Identity,
    user_repo: Data<UserRepository>,
) -> Result<HttpResponse, ApiError> {
    let user = user_repo
        .read_one(&UserGetById {
            id: Uuid::parse_str(user.id()?.as_ref())?,
        })
        .await?;

    // Fill form with signed in user's data
    let template = UserEditTemplate {
        user: UserEdit {
            id: user.id,
            username: user.username,
            email: user.email,
            profile_picture: user.profile_picture,
        },
    };
    let body = template.render()?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

/// Get user previews by username
async fn get_users(
    user: Identity,
    query: web::Query<UserSearchQuery>,
    user_repo: Data<UserRepository>,
) -> Result<HttpResponse, HtmxError> {
    let id = Uuid::parse_str(user.id()?.as_ref())?;

    query.validate()?;

    let users = user_repo
        .read_many(&UserGetByUsername {
            username: query.username.clone(),
        })
        .await?;

    let template = UserPreviewList {
        user_previews: users
            .into_iter()
            .map(UserPreviewView::from)
            .filter(|u| u.id != id)
            .collect(),
        group_id: query.group_id,
    };

    let body = template.render()?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
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

    // Add session info about user
    session.insert(
        "signed_user",
        SignedUser {
            username: user.username,
            profile_picture: user.profile_picture,
        },
    )?;

    // Redirect to main page if everything went well
    Ok(HttpResponse::Ok()
        .append_header(("HX-Redirect", "/"))
        .finish())
}

/// Update user details
async fn put_user(
    MultipartForm(form): MultipartForm<UserEditFormData>,
    user_repo: Data<UserRepository>,
    user: Identity, // User must be signed in to edit details
    session: Session,
) -> Result<HttpResponse, HtmxError> {
    // Check inputs
    form.validate()?;

    let id = Uuid::parse_str(user.id()?.as_ref())?;

    let user = user_repo.read_one(&UserGetById { id }).await?;

    // Check if old password is correct
    verify_password(&form.old_password, &user.password_hash)?;

    // Check email and username availability
    user_repo
        .check_email_and_password(&CheckEmailAndUsername {
            // We are editing, so we must add id of the edited user (there would be a collide
            // if user did not change email or username)
            edited_user_id: Some(id),
            username: form.username.to_owned(),
            email: form.email.to_owned(),
        })
        .await?;

    // Handle picture
    let profile_picture = if let Some(file) = form.file {
        Some(validate_and_save_picture(file).await?)
    } else {
        None // Keep old picture
    };

    // Generate new password hash if password was changed
    let password_hash = if !form.new_password.0.is_empty() {
        Some(hash_password(form.new_password.0.as_ref())?)
    } else {
        None // Keep old password
    };

    // Update user
    let updated_user = user_repo
        .update(&UserUpdate {
            id,
            username: Some(form.username.0),
            email: Some(form.email.0),
            profile_picture,
            password_hash,
        })
        .await?;

    if let Some(updated_user) = updated_user.first() {
        // Update user session data
        session.insert::<SignedUser>(
            "signed_user",
            SignedUser {
                username: updated_user.username.to_owned(),
                profile_picture: updated_user.profile_picture.to_owned(),
            },
        )?;
    } else {
        return Err(HtmxError::BannerErrorDefault);
    }

    // Redirect to main page if everything went well
    Ok(HttpResponse::Ok()
        .append_header(("HX-Redirect", "/"))
        .finish())
}
