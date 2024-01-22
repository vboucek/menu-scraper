use crate::app::errors::ApiError;
use crate::app::templates::restaurant::RestaurantTemplate;
use crate::app::view_models::menu::MenuView;
use crate::app::view_models::restaurant::RestaurantView;
use crate::app::view_models::signed_user::SignedUser;
use actix_session::Session;
use actix_web::web::Data;
use actix_web::{web, HttpResponse};
use askama::Template;
use chrono::{Duration, Local};
use db::db::common::query_parameters::DbOrder;
use db::db::common::{DbReadMany, DbReadOne};
use db::db::models::{DbRestaurantOrderingMethod, MenuReadMany, RestaurantGetById};
use db::db::repositories::{MenuRepository, RestaurantRepository};
use uuid::Uuid;

pub fn restaurant_config(config: &mut web::ServiceConfig) {
    config.service(web::resource("/restaurants/{id}").route(web::get().to(get_restaurant)));
}

/// Get available lunches for given user
async fn get_restaurant(
    id: web::Path<Uuid>,
    menu_repo: Data<MenuRepository>,
    restaurant_repo: Data<RestaurantRepository>,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    let restaurant_id = id.into_inner();

    let restaurant = restaurant_repo
        .read_one(&RestaurantGetById { id: restaurant_id })
        .await?;

    let signed_user = session.get::<SignedUser>("signed_user")?;

    let menus = menu_repo
        .read_many(&MenuReadMany {
            // Menus for the next 7 days (if available)
            date_from: Local::now().date_naive(),
            date_to: (Local::now() + Duration::days(6)).date_naive(),
            order_by: DbRestaurantOrderingMethod::Date(DbOrder::Asc),
            restaurant_id: Some(restaurant_id),
            limit: Some(7),
            offset: None,
        })
        .await?;

    let template = RestaurantTemplate {
        restaurant: RestaurantView::from(restaurant),
        signed_user,
        menus: menus.into_iter().map(MenuView::from).collect(),
    };

    let body = template.render()?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
