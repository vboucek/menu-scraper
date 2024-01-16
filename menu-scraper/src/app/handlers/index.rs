use std::sync::Mutex;
use actix_web::{error::ErrorInternalServerError, HttpResponse, Result as ActixResult, web};
use actix_web::web::Data;
use askama::Template;
use chrono::NaiveDate;
use db::db::common::DbReadMany;
use db::db::models::{MenuReadMany, RestaurantOrderingMethod};
use db::db::repositories::{MenuRepository};
use crate::app::models::menu::{MenuWithRestaurantView};
use crate::app::templates::index::IndexTemplate;
use crate::app::utils::date::generate_date_with_day_of_week;

pub fn index_config(config: &mut web::ServiceConfig) {
    config
        .service(web::resource("").route(web::get().to(index)));
}

async fn index(repo: Data<Mutex<MenuRepository>>) -> ActixResult<HttpResponse> {
    //todo - Change date to today
    let menus = repo.lock().unwrap().read_many(&MenuReadMany {
        date_from: NaiveDate::parse_from_str("2024-01-15", "%Y-%m-%d").unwrap(),
        date_to: NaiveDate::parse_from_str("2024-01-15", "%Y-%m-%d").unwrap(),
        order_by: RestaurantOrderingMethod::Random, // Use random ordering for the main page
        limit: Some(3),
        offset: None,
    }).await.map_err(ErrorInternalServerError)?;

    // Convert menus to view models
    let menus_view: Vec<MenuWithRestaurantView> = menus.into_iter().map(MenuWithRestaurantView::from).collect();

    let template = IndexTemplate { menus: menus_view, date: generate_date_with_day_of_week() };
    let body = template.render().map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
