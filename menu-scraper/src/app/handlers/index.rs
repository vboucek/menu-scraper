use crate::app::errors::ApiError;
use crate::app::templates::index::IndexTemplate;
use crate::app::utils::date::format_date_with_day_of_week;
use crate::app::view_models::menu::MenuWithRestaurantView;
use crate::app::view_models::signed_user::SignedUser;
use actix_session::Session;
use actix_web::web::Data;
use actix_web::{web, HttpResponse};
use askama::Template;
use chrono::Local;
use db::db::common::DbReadMany;
use db::db::models::{DbRestaurantOrderingMethod, MenuReadMany};
use db::db::repositories::MenuRepository;

pub fn index_config(config: &mut web::ServiceConfig) {
    config.service(web::resource("/").route(web::get().to(index)));
}

async fn index(repo: Data<MenuRepository>, session: Session) -> Result<HttpResponse, ApiError> {
    let menus = repo
        .read_many(&MenuReadMany {
            date_from: Local::now().date_naive(),
            date_to: Local::now().date_naive(),
            order_by: DbRestaurantOrderingMethod::Random, // Use random ordering for the main page
            restaurant_id: None,
            limit: Some(3),
            offset: None,
        })
        .await?;

    // Convert menus to view models
    let menus_view: Vec<MenuWithRestaurantView> = menus
        .into_iter()
        .map(MenuWithRestaurantView::from)
        .collect();

    let signed_user = session.get::<SignedUser>("signed_user")?;

    let template = IndexTemplate {
        menus: menus_view,
        date: format!(
            "Dnes je {}",
            format_date_with_day_of_week(Local::now().date_naive())
        ),
        signed_user,
    };
    let body = template.render()?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
