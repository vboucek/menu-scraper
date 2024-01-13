use chrono::{DateTime, NaiveDate, Utc};
use sqlx::postgres::{PgHasArrayType, PgTypeInfo};
use uuid::Uuid;
use crate::db::models::MenuItem;

/// Vote of a user in some lunch
#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq)]
pub struct Vote {
    pub id: Uuid,
    pub menu_id: Uuid,
    pub user_id: Uuid,
    pub lunch_id: Uuid,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(sqlx::Type, Debug, Clone, PartialEq, Eq)]
pub struct VotePreview {
    pub id: Uuid,
    pub user_id: Uuid,
}

/// Structure passed to the repository for creating a vote
#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq)]
pub struct VoteCreate {
    pub menu_id: Uuid,
    pub user_id: Uuid,
    pub lunch_id: Uuid,
}

/// Structure passed to the repository for removing a vote
#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq)]
pub struct VoteDelete {
    pub id: Uuid,
}

/// Structure passed to the repository for getting votes for some lunch
#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq)]
pub struct VoteGetMany {
    pub lunch_id: Uuid,
}

/// Structure passed to the repository when trying to find a vote by its id
#[derive(Debug, Clone)]
pub struct VoteGetById {
    pub id: Uuid,
}

impl VoteGetById {
    #[inline]
    pub const fn new(id: &Uuid) -> Self {
        Self { id: *id }
    }
}

/// Structure for getting menu with preview of the corresponding restaurant and votes (usable in
/// showing detail of a lunch)
#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq)]
pub struct MenuWithRestaurantAndVotes {
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
    pub votes: Vec<VotePreview>,
}