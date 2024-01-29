use crate::app::errors::{ApiError, HtmxError};
use crate::app::forms::group_creation::GroupCreationFormData;
use crate::app::templates::group::{GroupCreationTemplate, GroupsTemplate};
use crate::app::utils::picture::validate_and_save_picture;
use crate::app::utils::validation::Validation;
use crate::app::view_models::signed_user::SignedUser;
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
        .service(web::resource("/groups").route(web::get().to(group_index)))
        .service(web::resource("/group-create").route(web::get().to(get_group_create_form)))
        .service(web::resource("/group").route(web::post().to(post_group)));
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
async fn get_group_create_form(session: Session, user: Identity) -> Result<HttpResponse, ApiError> {
    let signed_user = session.get::<SignedUser>("signed_user")?;

    let template = GroupCreationTemplate {
        signed_user,
        group: GroupCreate {
            name: String::new(),
            description: None,
            author_id: Uuid::parse_str(user.id()?.as_ref())?,
            picture: None,
        },
    };
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

    // Handle group profile picture
    let picture = if let Some(file) = form.file {
        Some(validate_and_save_picture(file).await?)
    } else {
        None
    };

    // Handle group description
    let description = if let Some(description) = form.group_description {
        Some(description.to_string())
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
        })
        .await?;

    // Redirect back to groups if everything went well
    Ok(HttpResponse::Ok()
        .append_header(("HX-Redirect", "/groups"))
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
