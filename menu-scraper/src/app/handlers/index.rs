use std::sync::Mutex;
use actix_web::{HttpResponse, web};
use actix_web::web::Data;
use askama::Template;
use chrono::NaiveDate;
use db::db::common::DbReadMany;
use db::db::models::{MenuReadMany, RestaurantOrderingMethod};
use db::db::repositories::{MenuRepository};
use crate::app::errors::ApiError;
use crate::app::view_models::menu::{MenuWithRestaurantView};
use crate::app::templates::index::IndexTemplate;
use crate::app::utils::date::generate_date_with_day_of_week;

pub fn index_config(config: &mut web::ServiceConfig) {
    config
        .service(web::resource("/").route(web::get().to(index)));
}

async fn index(repo: Data<Mutex<MenuRepository>>) -> Result<HttpResponse, ApiError> {
    //todo - Change date to today
    let menus = repo.lock().unwrap().read_many(&MenuReadMany {
        date_from: NaiveDate::parse_from_str("2024-01-15", "%Y-%m-%d").unwrap(),
        date_to: NaiveDate::parse_from_str("2024-01-15", "%Y-%m-%d").unwrap(),
        order_by: RestaurantOrderingMethod::Random, // Use random ordering for the main page
        limit: Some(3),
        offset: None,
    }).await.map_err(ApiError::from)?;

    // Convert menus to view models
    let menus_view: Vec<MenuWithRestaurantView> = menus.into_iter().map(MenuWithRestaurantView::from).collect();

    let template = IndexTemplate { menus: menus_view, date: generate_date_with_day_of_week() };
    let body = template.render().map_err(ApiError::from)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
