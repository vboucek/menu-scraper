use crate::app::errors::HtmxError;
use crate::app::forms::vote::AddVoteFormData;
use actix_identity::Identity;
use actix_web::web::Data;
use actix_web::{web, HttpResponse};
use db::db::common::DbCreate;
use db::db::models::VoteCreate;
use db::db::repositories::VoteRepository;
use uuid::Uuid;

pub fn vote_config(config: &mut web::ServiceConfig) {
    config.service(web::resource("/vote").route(web::post().to(post_vote)));
}

/// Add vote in some lunch
async fn post_vote(
    form: web::Form<AddVoteFormData>,
    vote_repo: Data<VoteRepository>,
    user: Identity,
) -> Result<HttpResponse, HtmxError> {
    let id = Uuid::parse_str(user.id()?.as_ref())?;

    vote_repo
        .create(&VoteCreate {
            menu_id: form.menu_id,
            user_id: id,
            lunch_id: form.lunch_id,
        })
        .await?;

    // Redirect to the lunch
    Ok(HttpResponse::Ok()
        .append_header(("HX-Redirect", format!("/lunch/{}", form.lunch_id)))
        .finish())
}
