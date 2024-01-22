use crate::app::view_models::menu::MenuView;
use crate::app::view_models::restaurant::RestaurantView;
use crate::app::view_models::signed_user::SignedUser;
use askama::Template;

#[derive(Template)]
#[template(path = "restaurant.html")]
pub struct RestaurantTemplate {
    pub restaurant: RestaurantView,
    pub signed_user: Option<SignedUser>,
    pub menus: Vec<MenuView>,
}
