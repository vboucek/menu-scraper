use askama::Template;
use crate::app::view_models::menu::MenuWithRestaurantView;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub menus: Vec<MenuWithRestaurantView>,
    pub date: String,
}