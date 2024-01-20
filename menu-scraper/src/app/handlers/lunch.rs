use crate::app::errors::HtmxError;
use crate::app::forms::lunch::GetLunchPreviewsQuery;
use crate::app::templates::lunch::LunchPreviewListTemplate;
use crate::app::view_models::lunch::LunchPreviewView;
use actix_identity::Identity;
use actix_web::web::Data;
use actix_web::{web, HttpResponse};
use askama::Template;
use db::db::common::DbReadMany;
use db::db::models::LunchGetMany;
use db::db::repositories::LunchRepository;
use uuid::Uuid;

pub fn lunch_config(config: &mut web::ServiceConfig) {
    config.service(web::resource("/user-lunches").route(web::get().to(get_user_lunches)));
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
