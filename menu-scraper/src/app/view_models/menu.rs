use chrono::NaiveDate;
use db::db::models::{MenuItem, MenuWithRestaurant};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MenuWithRestaurantView {
    pub restaurant_id: Uuid,
    pub name: String,
    pub street: String,
    pub house_number: String,
    pub zip_code: String,
    pub city: String,
    pub picture: Option<String>,
    pub menu_id: Uuid,
    pub date: NaiveDate,
    pub items: Vec<MenuItemView>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MenuItemView {
    pub id: Uuid,
    pub name: String,
    pub price: i32,
    pub size: String,
    pub is_soup: bool,
    pub menu_id: Uuid,
}

impl From<MenuItem> for MenuItemView {
    fn from(item: MenuItem) -> Self {
        MenuItemView {
            id: item.id,
            name: item.name,
            price: item.price,
            size: item.size,
            is_soup: item.is_soup,
            menu_id: item.menu_id,
        }
    }
}

impl From<MenuWithRestaurant> for MenuWithRestaurantView {
    fn from(mut menu_with_restaurant: MenuWithRestaurant) -> Self {
        // Sort the soups first
        menu_with_restaurant
            .items
            .sort_by(|a, b| a.is_soup.cmp(&b.is_soup).reverse());

        MenuWithRestaurantView {
            restaurant_id: menu_with_restaurant.restaurant_id,
            name: menu_with_restaurant.name,
            street: menu_with_restaurant.street,
            house_number: menu_with_restaurant.house_number,
            zip_code: menu_with_restaurant.zip_code,
            city: menu_with_restaurant.city,
            picture: menu_with_restaurant.picture,
            menu_id: menu_with_restaurant.menu_id,
            date: menu_with_restaurant.date,
            items: menu_with_restaurant
                .items
                .into_iter()
                .map(MenuItemView::from)
                .collect(),
        }
    }
}
