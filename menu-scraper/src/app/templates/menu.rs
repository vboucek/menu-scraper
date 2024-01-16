use askama::Template;
use crate::app::models::menu::{MenuWithRestaurantView};

#[derive(Template)]
#[template(path = "menu.html")]
pub struct MenuTemplate {
    pub menu: MenuWithRestaurantView,
}