use crate::app::errors::{ApiError, HtmxError};
use crate::app::forms::group_creation::GroupCreationFormData;
use crate::app::forms::user_add_in_group::UserAddInGroupForm;
use crate::app::templates::group::{GroupCreationTemplate, GroupsTemplate};
use crate::app::templates::user_group_preview::UserGroupPreview;
use crate::app::utils::picture::validate_and_save_picture;
use crate::app::utils::validation::Validation;
use crate::app::view_models::signed_user::SignedUser;
use crate::app::view_models::user_preview::UserPreviewView;
use actix_identity::Identity;
use actix_multipart::form::MultipartForm;
use actix_session::Session;
use actix_web::web::Data;
use actix_web::{web, HttpResponse};
use askama::Template;
use db::db::common::{DbCreate, DbReadMany};
use db::db::models::{Group, GroupCreate, GroupGetById, GroupGetGroupsByUser};
use db::db::repositories::{GroupRepository, GroupRepositoryListUsers};
use uuid::Uuid;

pub fn group_config(config: &mut web::ServiceConfig) {
    config
        .service(
            web::resource("/groups")
                .route(web::get().to(group_index))
                .route(web::post().to(post_group)),
        )
        .service(web::resource("/group-create").route(web::get().to(get_group_create_form)))
        .service(web::resource("/group-user").route(web::get().to(get_group_user_preview)));
}

async fn group_index(
    repo: Data<GroupRepository>,
    user: Identity,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    let signed_user = session.get::<SignedUser>("signed_user")?;

    let group_previews = repo
        .read_many(&GroupGetGroupsByUser {
            user_id: Uuid::parse_str(user.id()?.as_ref())?,
        })
        .await?;

    let template = GroupsTemplate {
        group_previews,
        signed_user,
    };

    let body = template.render()?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

// Gets empty group creation form
async fn get_group_create_form(_: Identity) -> Result<HttpResponse, ApiError> {
    let template = GroupCreationTemplate {};
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

/// Create a new group
async fn post_group(
    MultipartForm(form): MultipartForm<GroupCreationFormData>,
    group_repo: Data<GroupRepository>,
    user: Identity,
) -> Result<HttpResponse, HtmxError> {
    // Check inputs
    form.validate()?;

    // Handle group picture
    let picture = if let Some(file) = form.file {
        Some(validate_and_save_picture(file).await?)
    } else {
        None
    };

    // Handle group description
    let description = if !form.group_description.is_empty() {
        Some(form.group_description.0)
    } else {
        None
    };

    // Store group
    let group = group_repo
        .create(&GroupCreate {
            name: form.group_name.to_owned(),
            description,
            author_id: Uuid::parse_str(user.id()?.as_ref())?,
            picture,
            users: form.users.iter().map(|u| u.0).collect(),
        })
        .await?;

    // Go to created group if everything went well
    Ok(HttpResponse::Ok()
        .append_header(("HX-Redirect", format!("/groups/{}", group.id)))
        .finish())
}

// Get group users as user previews
async fn get_group_users_previews(
    group: Group,
    group_repo: Data<GroupRepository>,
) -> Result<HttpResponse, HtmxError> {
    let previews = group_repo
        .list_group_users(&GroupGetById { id: group.id })
        .await?;

    Ok(HttpResponse::Ok().body(""))
}

async fn get_group_user_preview(
    form: web::Query<UserAddInGroupForm>,
) -> Result<HttpResponse, HtmxError> {
    let profile_picture = if form.profile_picture.is_empty() {
        None
    } else {
        Some(form.profile_picture.clone())
    };

    let template = UserGroupPreview {
        user_preview: UserPreviewView {
            id: form.id,
            username: form.0.username,
            profile_picture,
        },
    };

    let body = template.render()?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
