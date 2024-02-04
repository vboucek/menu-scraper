use crate::app::errors::{ApiError, HtmxError};
use crate::app::forms::group_creation::GroupCreationFormData;
use crate::app::forms::group_edit::GroupEditFormData;
use crate::app::forms::lunch::CreateLunchFormData;
use crate::app::forms::user_add_in_group::UserAddInGroupForm;
use crate::app::forms::user_delete_from_group::UserDeleteFromGroup;
use crate::app::forms::vote::AddVoteFormData;
use crate::app::templates::group::GroupEditTemplate;
use crate::app::templates::group::{
    GroupCreateLunchFormTemplate, GroupCreateLunchTemplate, GroupCreationTemplate,
    GroupDetailsTemplate, GroupLunchMenusTemplate, GroupsTemplate,
};
use crate::app::templates::user_group::{UserGroup, UserGroupPreview};
use crate::app::utils::picture::validate_and_save_picture;
use crate::app::utils::validation::Validation;
use crate::app::view_models::group::GroupView;
use crate::app::view_models::lunch::MenuWithRestaurantAndVotesView;
use crate::app::view_models::signed_user::SignedUser;
use crate::app::view_models::user_preview::UserPreviewView;
use actix_identity::Identity;
use actix_multipart::form::MultipartForm;
use actix_session::Session;
use actix_web::web::Data;
use actix_web::{web, HttpResponse};
use askama::Template;
use chrono::Local;
use db::db::common::{DbCreate, DbDelete, DbReadMany, DbReadOne, DbUpdate};
use db::db::models::{
    GetGroupUserByIds, LunchCreate, LunchGetById, LunchGetMany, VoteCreate, VoteGetMany,
};
use db::db::models::{
    GroupCreate, GroupDelete, GroupGetById, GroupGetGroupsByUser, GroupUpdate, GroupUserCreate,
    GroupUserDelete,
};
use db::db::repositories::{
    GroupRepository, GroupRepositoryAddUser, GroupRepositoryListUsers, GroupRepositoryRemoveUser,
};
use db::db::repositories::{GroupRepositoryCheckUser, LunchRepository, VoteRepository};
use uuid::Uuid;

pub fn group_config(config: &mut web::ServiceConfig) {
    config
        .service(
            web::resource("/groups")
                .route(web::get().to(group_index))
                .route(web::post().to(post_group)),
        )
        .service(
            web::resource("/groups/{id}")
                .route(web::get().to(group_details))
                .route(web::put().to(put_group))
                .route(web::delete().to(delete_group)),
        )
        .service(web::resource("/group-create").route(web::get().to(get_group_create_form)))
        .service(web::resource("/group-edit/{id}").route(web::get().to(get_group_edit_form)))
        .service(
            web::resource("/group-user")
                .route(web::get().to(get_group_user))
                .route(web::post().to(post_group_user))
                .route(web::delete().to(delete_group_user)),
        )
        .service(web::resource("/group-leave").route(web::delete().to(user_leave)))
        .service(web::resource("/group-create-lunch/{id}").route(web::post().to(create_lunch)))
        .service(
            web::resource("/group-create-lunch-form/{id}").route(web::post().to(create_lunch_form)),
        )
        .service(web::resource("/group-lunch/{id}").route(web::get().to(group_lunch_menus)))
        .service(web::resource("/menu-vote").route(web::post().to(menu_vote)));
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
    let group = group_repo.read_one(&GroupGetById { id: *id }).await?;

    // Check if signed user is the author of this group
    if group.author_id != Uuid::parse_str(user.id()?.as_ref())? {
        return Err(ApiError::Unauthorized);
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

async fn group_details(
    id: web::Path<Uuid>,
    group_repo: Data<GroupRepository>,
    lunch_repo: Data<LunchRepository>,
    session: Session,
    identity: Identity,
) -> Result<HttpResponse, ApiError> {
    let group_id = id.into_inner();
    let user_id = Uuid::parse_str(identity.id()?.as_ref())?;
    let signed_user = session.get::<SignedUser>("signed_user")?;
    group_repo
        .check_user_is_member(&GetGroupUserByIds { user_id, group_id })
        .await
        .map_err(|_| ApiError::Unauthorized)?;

    let group_by_id = GroupGetById { id: group_id };

    let group = group_repo.read_one(&group_by_id).await?;
    let members = group_repo.list_group_users(&group_by_id).await?;

    let lunches = lunch_repo
        .read_many(&LunchGetMany {
            group_id: Some(group_id),
            user_id: Some(user_id),
            from: Some(Local::now().date_naive()),
            to: None,
        })
        .await?;

    let template = GroupDetailsTemplate {
        is_author: user_id == group.author_id,
        group,
        signed_user,
        group_members: members,
        group_lunches: lunches,
        user_id,
    };

    let body = template.render()?;

    Ok(HttpResponse::Ok().body(body))
}

async fn create_lunch(group_id: web::Path<Uuid>) -> Result<HttpResponse, HtmxError> {
    let template = GroupCreateLunchTemplate {
        group_id: group_id.into_inner(),
        min_selection_date: Local::now().date_naive(),
    };
    let body = template.render()?;
    Ok(HttpResponse::Ok().body(body))
}

async fn create_lunch_form(
    lunch_repo: Data<LunchRepository>,
    group_repo: Data<GroupRepository>,
    form: web::Form<CreateLunchFormData>,
    group_id: web::Path<Uuid>,
    identity: Identity,
) -> Result<HttpResponse, HtmxError> {
    let user_id = Uuid::parse_str(identity.id()?.as_ref())?;

    group_repo
        .check_user_is_member(&GetGroupUserByIds {
            user_id,
            group_id: *group_id,
        })
        .await
        .map_err(|_| {
            HtmxError::BannerError(
                "Tento uživatel nemůže vytvořit oběd, protože není ve skupině.".to_string(),
            )
        })?;

    let date = form.date;
    let group_id = group_id.into_inner();
    let lunch = lunch_repo.create(&LunchCreate { date, group_id }).await?;

    let template = GroupCreateLunchFormTemplate {
        group_id,
        lunch,
        date,
    };
    let body = template.render()?;

    Ok(HttpResponse::Ok().body(body))
}

// Displaying the lunch menus
async fn group_lunch_menus(
    lunch_id: web::Path<Uuid>,
    vote_repo: Data<VoteRepository>,
    lunch_repo: Data<LunchRepository>,
    session: Session,
    user_id: Identity,
) -> Result<HttpResponse, HtmxError> {
    let lunch_id = lunch_id.into_inner();
    let menus = vote_repo.read_many(&VoteGetMany { lunch_id }).await?;
    let signed_user = session.get::<SignedUser>("signed_user")?;
    let lunch = lunch_repo.read_one(&LunchGetById { id: lunch_id }).await?;
    let user_id = user_id.id()?.parse()?;

    let template = GroupLunchMenusTemplate {
        signed_user,
        lunch,
        menus: menus
            .into_iter()
            .map(|m| MenuWithRestaurantAndVotesView::new(m, user_id))
            .collect(),
    };
    let body = template.render()?;
    Ok(HttpResponse::Ok().body(body))
}

// Voting for a specific menu, returning the updated menu
async fn menu_vote(
    vote_repo: Data<VoteRepository>,
    lunch_repo: Data<LunchRepository>,
    form: web::Form<AddVoteFormData>,
    user_id: Identity,
    session: Session,
) -> Result<HttpResponse, HtmxError> {
    let signed_user = session.get::<SignedUser>("signed_user")?;
    let user_id = user_id.id()?.parse()?;
    let menu_id = form.menu_id;
    let lunch_id = form.lunch_id;

    vote_repo
        .create(&VoteCreate {
            menu_id,
            user_id,
            lunch_id,
        })
        .await?;

    let lunch = lunch_repo.read_one(&LunchGetById { id: lunch_id }).await?;
    let menus = vote_repo.read_many(&VoteGetMany { lunch_id }).await?;
    let template = GroupLunchMenusTemplate {
        signed_user,
        lunch,
        menus: menus
            .into_iter()
            .map(|m| MenuWithRestaurantAndVotesView::new(m, user_id))
            .collect(),
    };
    let body = template.render()?;
    Ok(HttpResponse::Ok().body(body))
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
        .read_one(&GroupGetById { id: form.group_id })
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

async fn user_leave(
    form: web::Form<UserDeleteFromGroup>,
    group_repo: Data<GroupRepository>,
    user: Identity,
) -> Result<HttpResponse, HtmxError> {
    let signed_user = Uuid::parse_str(user.id()?.as_ref())?;

    // Check that signed user si the one leaving group
    if form.user_id != signed_user {
        return Err(HtmxError::BannerError(
            "Tento uživatel nemůže odstranit uživatele ze skupiny.".to_string(),
        ));
    }

    group_repo
        .remove_user_from_group(&GroupUserDelete {
            user_id: form.user_id,
            group_id: form.group_id,
        })
        .await?;

    Ok(HttpResponse::Ok()
        .append_header(("HX-Redirect", "/groups"))
        .finish())
}

/// Deletes user from the group
async fn delete_group(
    group_repo: Data<GroupRepository>,
    user: Identity,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, HtmxError> {
    let group = group_repo
        .read_one(&GroupGetById {
            id: id.into_inner(),
        })
        .await?;

    let signed_user = Uuid::parse_str(user.id()?.as_ref())?;

    // Check if signed user is the author of this group
    if group.author_id != signed_user {
        return Err(HtmxError::BannerError(
            "Tento uživatel nemůže odstranit tuto skupinu.".to_string(),
        ));
    }

    // Remove group
    group_repo.delete(&GroupDelete { id: group.id }).await?;

    Ok(HttpResponse::Ok()
        .append_header(("HX-Redirect", "/groups"))
        .finish())
}
