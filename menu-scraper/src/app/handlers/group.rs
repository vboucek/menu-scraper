use crate::app::errors::{ApiError, HtmxError};
use crate::app::forms::group_creation::GroupCreationFormData;
use crate::app::forms::group_edit::GroupEditFormData;
use crate::app::forms::user_add_in_group::UserAddInGroupForm;
use crate::app::forms::user_delete_from_group::UserDeleteFromGroup;
use crate::app::templates::group::{GroupCreationTemplate, GroupEditTemplate, GroupsTemplate};
use crate::app::templates::user_group::{UserGroup, UserGroupPreview};
use crate::app::utils::picture::validate_and_save_picture;
use crate::app::utils::validation::Validation;
use crate::app::view_models::group::GroupView;
use crate::app::view_models::signed_user::SignedUser;
use crate::app::view_models::user_preview::UserPreviewView;
use actix_identity::Identity;
use actix_multipart::form::MultipartForm;
use actix_session::Session;
use actix_web::web::Data;
use actix_web::{web, HttpResponse};
use askama::Template;
use db::db::common::{DbCreate, DbReadMany, DbReadOne, DbUpdate};
use db::db::models::{
    Group, GroupCreate, GroupGetById, GroupGetGroupsByUser, GroupUpdate, GroupUserCreate,
    GroupUserDelete,
};
use db::db::repositories::{
    GroupRepository, GroupRepositoryAddUser, GroupRepositoryListUsers, GroupRepositoryRemoveUser,
};
use uuid::Uuid;

pub fn group_config(config: &mut web::ServiceConfig) {
    config
        .service(
            web::resource("/groups")
                .route(web::get().to(group_index))
                .route(web::post().to(post_group)),
        )
        .service(web::resource("/groups/{id}").route(web::put().to(put_group)))
        .service(web::resource("/group-create").route(web::get().to(get_group_create_form)))
        .service(web::resource("/group-edit/{id}").route(web::get().to(get_group_edit_form)))
        .service(
            web::resource("/group-user")
                .route(web::get().to(get_group_user))
                .route(web::post().to(post_group_user))
                .route(web::delete().to(delete_group_user)),
        );
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

/// Gets empty group creation form
async fn get_group_create_form(_: Identity) -> Result<HttpResponse, ApiError> {
    let template = GroupCreationTemplate {};
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

/// Gets group edit form
async fn get_group_edit_form(
    user: Identity,
    id: web::Path<Uuid>,
    group_repo: Data<GroupRepository>,
) -> Result<HttpResponse, ApiError> {
    let group = group_repo
        .read_one(&GroupGetById { id: id.clone() })
        .await?;

    // Check if signed user is the author of this group
    if group.author_id != Uuid::parse_str(user.id()?.as_ref())? {
        //TODO change to unauthorized
        return Err(ApiError::InternalServerError);
    }

    let users = group_repo
        .list_group_users(&GroupGetById {
            id: id.into_inner(),
        })
        .await?;

    let template = GroupEditTemplate {
        group: GroupView {
            id: group.id,
            name: group.name,
            description: group.description,
            picture: group.picture,
            users: users
                .into_iter()
                .map(UserPreviewView::from)
                // Don't list group author
                .filter(|u| u.id != group.author_id)
                .collect(),
        },
        group_id: group.id, // Workaround, because askama does not support passing attributes to includes, for some reason
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

/// Edit already existing group
async fn put_group(
    id: web::Path<Uuid>,
    MultipartForm(form): MultipartForm<GroupEditFormData>,
    group_repo: Data<GroupRepository>,
    user: Identity,
) -> Result<HttpResponse, HtmxError> {
    let group = group_repo
        .read_one(&GroupGetById {
            id: id.into_inner(),
        })
        .await?;

    // Check if signed user is the author of this group
    if group.author_id != Uuid::parse_str(user.id()?.as_ref())? {
        return Err(HtmxError::BannerError(
            "Tento uživatel nemůže měnit tuto skupinu.".to_string(),
        ));
    }

    // Check inputs
    form.validate()?;

    // Handle group picture
    let picture = if let Some(file) = form.file {
        Some(validate_and_save_picture(file).await?)
    } else {
        group.picture
    };

    // Handle group description
    let description = if !form.group_description.is_empty() {
        Some(form.group_description.0)
    } else {
        None
    };

    // Update group
    group_repo
        .update(&GroupUpdate {
            id: group.id,
            name: Some(form.group_name.0),
            description,
            picture,
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

/// Gets user preview, does not persist anything - usable for creating a new group
async fn get_group_user(form: web::Query<UserAddInGroupForm>) -> Result<HttpResponse, HtmxError> {
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

/// Adds user to the group and persists change in the db
async fn post_group_user(
    form: web::Form<UserAddInGroupForm>,
    group_repo: Data<GroupRepository>,
) -> Result<HttpResponse, HtmxError> {
    let profile_picture = if form.profile_picture.is_empty() {
        None
    } else {
        Some(form.profile_picture.clone())
    };

    let group_id = form.group_id.ok_or(HtmxError::BannerErrorDefault)?;

    // Add user to group
    group_repo
        .add_user_to_group(&GroupUserCreate {
            user_id: form.id,
            group_id,
        })
        .await?;

    let template = UserGroup {
        user_preview: UserPreviewView {
            id: form.id,
            username: form.0.username,
            profile_picture,
        },
        group_id,
    };

    let body = template.render()?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

/// Deletes user from the group
async fn delete_group_user(
    form: web::Form<UserDeleteFromGroup>,
    group_repo: Data<GroupRepository>,
    user: Identity,
) -> Result<HttpResponse, HtmxError> {
    let group = group_repo
        .read_one(&GroupGetById {
            id: form.group_id.clone(),
        })
        .await?;

    let signed_user = Uuid::parse_str(user.id()?.as_ref())?;

    // Check if signed user is the author of this group or user being removed
    if group.author_id != signed_user && form.user_id != signed_user {
        return Err(HtmxError::BannerError(
            "Tento uživatel nemůže odstranit uživatele ze skupiny.".to_string(),
        ));
    }

    // Remove user from the group
    group_repo
        .remove_user_from_group(&GroupUserDelete {
            user_id: form.user_id,
            group_id: form.group_id,
        })
        .await?;

    Ok(HttpResponse::Ok().finish())
}
