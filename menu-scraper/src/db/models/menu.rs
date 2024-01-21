use crate::db::common::query_parameters::DbOrder;
use chrono::{DateTime, NaiveDate, Utc};
use sqlx::postgres::{PgHasArrayType, PgTypeInfo};
use uuid::Uuid;

/// Daily menu of a restaurant
#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq)]
pub struct Menu {
    pub id: Uuid,
    pub date: NaiveDate,
    pub restaurant_id: Uuid,
    pub deleted_at: Option<DateTime<Utc>>,
    pub items: Vec<MenuItem>,
}

/// One item from a menu
#[derive(sqlx::Type, Debug, Clone, PartialEq, Eq)]
pub struct MenuItem {
    pub id: Uuid,
    pub name: String,
    pub price: i32,
    pub size: String,
    pub is_soup: bool,
    pub menu_id: Uuid,
}

impl PgHasArrayType for MenuItem {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_MenuItem")
    }
}

/// Structure passed to the repository for creating a menu
#[derive(Debug, Clone)]
pub struct MenuCreate {
    pub date: NaiveDate,
    pub restaurant_id: Uuid,
    pub items: Vec<MenuItemCreate>,
}

/// Structure passed to the repository for creating a menu item
#[derive(Debug, Clone)]
pub struct MenuItemCreate {
    pub name: String,
    pub price: i32,
    pub size: String,
    pub is_soup: bool,
}

/// Structure passed to the repository when trying to delete a menu
#[derive(Debug, Clone)]
pub struct MenuDelete {
    pub id: Uuid,
}

impl MenuDelete {
    #[inline]
    pub const fn new(id: &Uuid) -> Self {
        Self { id: *id }
    }
}

/// Structure passed to the repository when trying to find a menu by its id
#[derive(Debug, Clone)]
pub struct MenuGetById {
    pub id: Uuid,
}

impl MenuGetById {
    #[inline]
    pub const fn new(id: &Uuid) -> Self {
        Self { id: *id }
    }
}

/// Structure passed to the repository for getting multiple menus, supporting pagination
#[derive(Debug, Clone)]
pub struct MenuReadMany {
    pub date_from: NaiveDate,
    pub date_to: NaiveDate,
    pub order_by: DbRestaurantOrderingMethod,
    pub restaurant_id: Option<Uuid>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Methods of ordering for retrieved restaurants/menus
#[derive(Debug, Clone)]
pub enum DbRestaurantOrderingMethod {
    Price(DbOrder),
    Range(DbOrder, (f64, f64)), // Location of the user - longitude + latitude
    Random,
    Date(DbOrder),
}

/// Structure for manipulating with only ID of the menu
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct MenuId {
    pub id: Uuid,
}

/// Structure for getting menu with preview of the corresponding restaurant
#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq)]
pub struct MenuWithRestaurant {
    pub restaurant_id: Uuid,
    pub name: String,
    pub street: String,
    pub house_number: String,
    pub zip_code: String,
    pub city: String,
    pub picture: Option<String>,
    pub menu_id: Uuid,
    pub date: NaiveDate,
    pub items: Vec<MenuItem>,
}

/// Structure passed to the repository for getting number of menus, used for pagination
#[derive(Debug, Clone)]
pub struct MenuGetCount {
    pub date_from: NaiveDate,
    pub date_to: NaiveDate,
}

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq)]
pub struct MenuCount {
    pub count: Option<i64>,
}
