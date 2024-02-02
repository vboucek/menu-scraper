use crate::app::errors::{ApiError, HtmxError};
use crate::app::forms::group_creation::GroupCreationFormData;
use crate::app::forms::lunch::CreateLunchFormData;
use crate::app::forms::vote::AddVoteFormData;
use crate::app::templates::group::{
    GroupCreateLunchFormTemplate, GroupCreateLunchTemplate, GroupCreationTemplate,
    GroupDetailsTemplate, GroupLunchMenusTemplate, GroupsTemplate,
};
use crate::app::utils::picture::validate_and_save_picture;
use crate::app::utils::validation::Validation;
use crate::app::view_models::lunch::MenuWithRestaurantAndVotesView;
use crate::app::view_models::signed_user::SignedUser;
use actix_identity::Identity;
use actix_multipart::form::MultipartForm;
use actix_session::Session;
use actix_web::web::Data;
use actix_web::{web, HttpResponse};
use askama::Template;
use chrono::Local;
use db::db::common::{DbCreate, DbReadMany, DbReadOne};
use db::db::models::{
    GetGroupUserByIds, GroupCreate, GroupGetById, GroupGetGroupsByUser, LunchCreate, LunchGetById,
    LunchGetMany, VoteCreate, VoteGetMany,
};
use db::db::repositories::{
    GroupRepository, GroupRepositoryCheckUser, GroupRepositoryListUsers, LunchRepository,
    VoteRepository,
};
use uuid::Uuid;

pub fn group_config(config: &mut web::ServiceConfig) {
    config
        .service(web::resource("/groups").route(web::get().to(group_index)))
        .service(web::resource("/group-create").route(web::get().to(get_group_create_form)))
        .service(web::resource("/group").route(web::post().to(post_group)))
        .service(web::resource("/group-details/{id}").route(web::get().to(group_details)))
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
    group_repo
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
            from: None,
            to: None,
        })
        .await?;

    let template = GroupDetailsTemplate {
        group,
        signed_user,
        group_members: members,
        group_lunches: lunches,
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
    form: web::Form<CreateLunchFormData>,
    group_id: web::Path<Uuid>,
) -> Result<HttpResponse, HtmxError> {
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
    json_data: web::Json<AddVoteFormData>,
    user_id: Identity,
    session: Session,
) -> Result<HttpResponse, HtmxError> {
    let signed_user = session.get::<SignedUser>("signed_user")?;
    let user_id = user_id.id()?.parse()?;
    let menu_id = json_data.menu_id;
    let lunch_id = json_data.lunch_id;

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
