use db::db::models::Restaurant;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestaurantView {
    pub name: String,
    pub street: String,
    pub house_number: String,
    pub zip_code: String,
    pub city: String,
    pub picture: Option<String>,
    pub phone_number: Option<String>,
    pub website: Option<String>,
    pub email: Option<String>,
    pub monday_open: Option<String>,
    pub tuesday_open: Option<String>,
    pub wednesday_open: Option<String>,
    pub thursday_open: Option<String>,
    pub friday_open: Option<String>,
    pub saturday_open: Option<String>,
    pub sunday_open: Option<String>,
    pub lunch_served: Option<String>,
}

impl From<Restaurant> for RestaurantView {
    fn from(restaurant: Restaurant) -> Self {
        RestaurantView {
            name: restaurant.name,
            street: restaurant.street,
            house_number: restaurant.house_number,
            zip_code: restaurant.zip_code,
            city: restaurant.city,
            picture: restaurant.picture,
            phone_number: restaurant.phone_number,
            website: restaurant.website,
            email: restaurant.email,
            monday_open: restaurant.monday_open,
            tuesday_open: restaurant.tuesday_open,
            wednesday_open: restaurant.wednesday_open,
            thursday_open: restaurant.thursday_open,
            friday_open: restaurant.friday_open,
            saturday_open: restaurant.saturday_open,
            sunday_open: restaurant.sunday_open,
            lunch_served: restaurant.lunch_served,
        }
    }
}
