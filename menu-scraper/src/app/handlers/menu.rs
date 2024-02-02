use crate::app::errors::{ApiError, HtmxError};
use crate::app::forms::menu::MenuListQuery;
use crate::app::forms::ordering::{Ordering, RestaurantOrderingMethod};
use crate::app::templates::menu::{MenuIndexTemplate, MenuListTemplate};
use crate::app::view_models::menu::MenuWithRestaurantView;
use crate::app::view_models::signed_user::SignedUser;
use actix_session::Session;
use actix_web::web::Data;
use actix_web::{web, HttpResponse};
use askama::Template;
use chrono::Local;
use db::db::common::query_parameters::DbOrder;
use db::db::common::DbReadMany;
use db::db::models::{DbRestaurantOrderingMethod, MenuGetCount, MenuReadMany};
use db::db::repositories::{GetNumberOfMenus, MenuRepository};

const PAGE_SIZE: usize = 10;

pub fn menu_config(config: &mut web::ServiceConfig) {
    config
        .service(web::resource("/menus").route(web::get().to(menu_index)))
        .service(web::resource("/menu-list").route(web::get().to(get_menu_list)));
}

async fn menu_index(session: Session) -> Result<HttpResponse, ApiError> {
    let signed_user = session.get::<SignedUser>("signed_user")?;

    let template = MenuIndexTemplate {
        date: Local::now().date_naive(),
        signed_user,
    };
    let body = template.render()?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

/// Get list of menus
async fn get_menu_list(
    query: web::Query<MenuListQuery>,
    repo: Data<MenuRepository>,
    session: Session,
) -> Result<HttpResponse, HtmxError> {
    let order = match query.ordering {
        Ordering::Asc => DbOrder::Asc,
        Ordering::Desc => DbOrder::Desc,
    };

    let method = match query.method {
        RestaurantOrderingMethod::Price => DbRestaurantOrderingMethod::Price(order),
        RestaurantOrderingMethod::Range => {
            if let (Some(longitude), Some(latitude)) = (query.longitude, query.latitude) {
                Ok(DbRestaurantOrderingMethod::Range(
                    order,
                    (longitude, latitude),
                ))
            } else {
                Err(HtmxError::BannerErrorDefault)
            }
        }?,
    };

    let menu_count = repo
        .get_number_of_menus(&MenuGetCount {
            date_from: query.date,
            date_to: query.date,
        })
        .await?;

    let menus = repo
        .read_many(&MenuReadMany {
            date_from: query.date,
            date_to: query.date,
            order_by: method,
            restaurant_id: None,
            limit: Some(PAGE_SIZE as i64),
            offset: Some((PAGE_SIZE * (query.page - 1)) as i64),
        })
        .await?;

    // Convert menus to view models
    let menus_view: Vec<MenuWithRestaurantView> = menus
        .into_iter()
        .map(MenuWithRestaurantView::from)
        .collect();

    let signed_user = session.get::<SignedUser>("signed_user")?;

    let template = MenuListTemplate {
        menus: menus_view,
        signed_user,
        pages: (menu_count as f64 / PAGE_SIZE as f64).ceil() as usize,
    };

    let body = template.render()?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
