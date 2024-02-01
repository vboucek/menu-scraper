use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

/// Lunch of a group
#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq)]
pub struct Lunch {
    pub id: Uuid,
    pub date: NaiveDate,
    pub group_id: Uuid,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Lunch of a group with name, usable for listing available lunches
#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq)]
pub struct LunchWithGroup {
    pub id: Uuid,
    pub date: NaiveDate,
    pub group_id: Uuid,
    pub group_name: String,
    pub group_picture: Option<String>,
}

/// Structure passed to the repository for creating a lunch
#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq)]
pub struct LunchCreate {
    pub date: NaiveDate,
    pub group_id: Uuid,
}

/// Structure passed to the repository for removing a lunch
#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq)]
pub struct LunchDelete {
    pub id: Uuid,
}

/// Structure passed to the repository for getting lunches of some group
#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq)]
pub struct LunchGetMany {
    pub group_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
}

/// Structure passed to the repository when trying to find a lunch by its id
#[derive(Debug, Clone)]
pub struct LunchGetById {
    pub id: Uuid,
}

impl LunchGetById {
    #[inline]
    pub const fn new(id: &Uuid) -> Self {
        Self { id: *id }
    }
}
