use crate::app::errors::ApiError;
use crate::app::templates::user_edit::UserEditTemplate;
use crate::app::view_models::user_edit::UserEdit;
use actix_identity::Identity;
use actix_web::web::Data;
use actix_web::{web, HttpResponse};
use askama::Template;
use db::db::common::DbReadOne;
use db::db::models::UserGetById;
use db::db::repositories::UserRepository;
use uuid::Uuid;

pub fn user_edit_config(config: &mut web::ServiceConfig) {
    config.service(web::resource("/user-edit").route(web::get().to(get_user_edit_form)));
}

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
