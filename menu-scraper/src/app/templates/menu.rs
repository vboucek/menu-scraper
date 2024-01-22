use crate::app::view_models::menu::{MenuView, MenuWithRestaurantView};
use crate::app::view_models::signed_user::SignedUser;
use askama::Template;
use chrono::NaiveDate;

#[derive(Template)]
#[template(path = "menu_with_restaurant.html")]
pub struct MenuWithRestaurantTemplate {
    pub menu: MenuWithRestaurantView,
    pub signed_user: Option<SignedUser>,
}

#[derive(Template)]
#[template(path = "menu.html")]
pub struct MenuTemplate {
    pub menu: MenuView,
    pub signed_user: Option<SignedUser>,
}

#[derive(Template)]
#[template(path = "menu_index.html")]
pub struct MenuIndexTemplate {
    pub signed_user: Option<SignedUser>,
    pub date: NaiveDate,
}

#[derive(Template)]
#[template(path = "menu_list.html")]
pub struct MenuListTemplate {
    pub signed_user: Option<SignedUser>,
    pub menus: Vec<MenuWithRestaurantView>,
    pub pages: usize,
}
