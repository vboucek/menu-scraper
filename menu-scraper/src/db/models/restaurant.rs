use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq)]
pub struct Restaurant {
    pub id: Uuid,
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
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Structure added to some menu - only most important info about the restaurant
#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq)]
pub struct RestaurantPreview {
    pub id: Uuid,
    pub name: String,
    pub street: String,
    pub house_number: String,
    pub zip_code: String,
    pub city: String,
    pub picture: Option<String>,
}

/// Structure passed to the repository for Restaurant creation
#[derive(Debug, Clone)]
pub struct RestaurantCreate {
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

/// Structure passed to the repository when trying to update a Restaurant
#[derive(Debug, Clone)]
pub struct RestaurantUpdate {
    pub id: Uuid,
    pub name: Option<String>,
    pub street: Option<String>,
    pub house_number: Option<String>,
    pub zip_code: Option<String>,
    pub city: Option<String>,
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

/// Structure passed to the repository when trying to delete a Restaurant
#[derive(Debug, Clone)]
pub struct RestaurantDelete {
    pub id: Uuid,
}

impl RestaurantDelete {
    #[inline]
    pub const fn new(id: &Uuid) -> Self {
        Self { id: *id }
    }
}

/// Structure passed to the repository when trying to find a Restaurant
#[derive(Debug, Clone)]
pub struct RestaurantGetById {
    pub id: Uuid,
}

impl RestaurantGetById {
    #[inline]
    pub const fn new(id: &Uuid) -> Self {
        Self { id: *id }
    }
}

#[derive(Debug, Clone)]
pub struct RestaurantGetByNameAndAddress {
    pub name: String,
    pub street: String,
    pub house_number: String,
    pub zip_code: String,
    pub city: String,
}

#[derive(Debug, Clone)]
pub struct RestaurantId {
    pub id: Uuid,
}
