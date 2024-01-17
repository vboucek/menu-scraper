use crate::app::view_models::menu::MenuWithRestaurantView;
use askama::Template;

#[derive(Template)]
#[template(path = "menu.html")]
pub struct MenuTemplate {
    pub menu: MenuWithRestaurantView,
}
