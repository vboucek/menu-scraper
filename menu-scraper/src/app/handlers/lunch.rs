use crate::app::errors::HtmxError;
use crate::app::forms::lunch::GetLunchPreviewsQuery;
use crate::app::templates::lunch::LunchPreviewListTemplate;
use crate::app::view_models::lunch::LunchPreviewView;
use actix_identity::Identity;
use actix_web::web::Data;
use actix_web::{web, HttpResponse};
use askama::Template;
use db::db::common::{DbDelete, DbReadMany, DbReadOne};
use db::db::models::{GetGroupUserByIds, LunchDelete, LunchGetById, LunchGetMany};
use db::db::repositories::{GroupRepository, GroupRepositoryCheckUser, LunchRepository};
use uuid::Uuid;

pub fn lunch_config(config: &mut web::ServiceConfig) {
    config.service(web::resource("/user-lunches").route(web::get().to(get_user_lunches)));
    config.service(web::resource("/lunches/{id}").route(web::delete().to(delete_lunch)));
}

/// Get available lunches for given user
async fn get_user_lunches(
    query: web::Query<GetLunchPreviewsQuery>,
    user: Identity,
    lunch_repo: Data<LunchRepository>,
) -> Result<HttpResponse, HtmxError> {
    let user_id = Uuid::parse_str(user.id()?.as_ref())?;

    let lunches = lunch_repo
        .read_many(&LunchGetMany {
            group_id: None,
            // We want lunches for a specified user for a specified date
            user_id: Some(user_id),
            from: Some(query.date),
            to: Some(query.date),
        })
        .await?;

    let template = LunchPreviewListTemplate {
        lunches: lunches
            .into_iter()
            .map(|l| LunchPreviewView::new(l, query.menu_id))
            .collect(),
    };

    let body = template.render()?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

async fn delete_lunch(
    lunch_repo: Data<LunchRepository>,
    group_repo: Data<GroupRepository>,
    lunch_id: web::Path<Uuid>,
    identity: Identity,
) -> Result<HttpResponse, HtmxError> {
    let user_id = Uuid::parse_str(identity.id()?.as_ref())?;
    let lunch = lunch_repo
        .read_one(&LunchGetById {
            id: lunch_id.into_inner(),
        })
        .await?;

    group_repo
        .check_user_is_member(&GetGroupUserByIds {
            user_id,
            group_id: lunch.group_id,
        })
        .await
        .map_err(|_| {
            HtmxError::BannerError(
                "Tento uživatel nemůže odstranit oběd, protože není ve skupině.".to_string(),
            )
        })?;

    lunch_repo.delete(&LunchDelete { id: lunch.id }).await?;

    Ok(HttpResponse::Ok().finish())
}
