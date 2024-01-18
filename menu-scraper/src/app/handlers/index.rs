use crate::app::errors::ApiError;
use crate::app::templates::index::IndexTemplate;
use crate::app::utils::date::generate_date_with_day_of_week;
use crate::app::view_models::menu::MenuWithRestaurantView;
use crate::app::view_models::signed_user::SignedUser;
use actix_session::Session;
use actix_web::web::Data;
use actix_web::{web, HttpResponse};
use askama::Template;
use chrono::NaiveDate;
use db::db::common::DbReadMany;
use db::db::models::{MenuReadMany, RestaurantOrderingMethod};
use db::db::repositories::MenuRepository;

pub fn index_config(config: &mut web::ServiceConfig) {
    config.service(web::resource("/").route(web::get().to(index)));
}

async fn index(
    repo: Data<MenuRepository>,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    //todo - Change date to today
    let menus = repo
        .read_many(&MenuReadMany {
            date_from: NaiveDate::parse_from_str("2024-01-15", "%Y-%m-%d").unwrap(),
            date_to: NaiveDate::parse_from_str("2024-01-15", "%Y-%m-%d").unwrap(),
            order_by: RestaurantOrderingMethod::Random, // Use random ordering for the main page
            limit: Some(3),
            offset: None,
        })
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    // Convert menus to view models
    let menus_view: Vec<MenuWithRestaurantView> = menus
        .into_iter()
        .map(MenuWithRestaurantView::from)
        .collect();

    let signed_user = session
        .get::<SignedUser>("signed_user")
        .map_err(|_| ApiError::InternalServerError)?;

    let template = IndexTemplate {
        menus: menus_view,
        date: generate_date_with_day_of_week(),
        signed_user,
    };
    let body = template.render().map_err(ApiError::from)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
