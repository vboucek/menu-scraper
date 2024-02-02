use chrono::NaiveDate;
use db::db::models::{LunchWithGroup, MenuItem, MenuWithRestaurantAndVotes};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LunchPreviewView {
    pub id: Uuid,
    pub date: NaiveDate,
    pub group_id: Uuid,
    pub menu_id: Uuid,
    pub group_name: String,
}

impl LunchPreviewView {
    pub fn new(lunch: LunchWithGroup, menu_id: Uuid) -> Self {
        LunchPreviewView {
            id: lunch.id,
            date: lunch.date,
            group_id: lunch.group_id,
            group_name: lunch.group_name,
            menu_id,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MenuWithRestaurantAndVotesView {
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
    pub votes: usize,
    pub is_voted_for: bool,
}

impl MenuWithRestaurantAndVotesView {
    pub fn new(menu: MenuWithRestaurantAndVotes, user_id: Uuid) -> Self {
        let is_voted_for = menu.votes.iter().any(|vote| vote.user_id == user_id);

        MenuWithRestaurantAndVotesView {
            restaurant_id: menu.restaurant_id,
            name: menu.name,
            street: menu.street,
            house_number: menu.house_number,
            zip_code: menu.zip_code,
            city: menu.city,
            picture: menu.picture,
            menu_id: menu.menu_id,
            date: menu.date,
            items: menu.items,
            votes: menu.votes.len(),
            is_voted_for,
        }
    }
}
