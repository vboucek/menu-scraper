use askama::Template;
use crate::app::view_models::menu::MenuWithRestaurantView;
use crate::app::view_models::signed_user::SignedUser;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub signed_user: Option<SignedUser>,
    pub menus: Vec<MenuWithRestaurantView>,
    pub date: String,
}