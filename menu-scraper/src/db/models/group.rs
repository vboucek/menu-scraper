use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Group (friends, colleagues...) for creating lunches
#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq)]
pub struct Group {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub picture: Option<String>,
    pub author_id: Uuid,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Group preview for listing
#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq)]
pub struct GroupPreview {
    pub id: Uuid,
    pub name: String,
    pub picture: Option<String>,
}

/// Structure passed to the repository for creating a group
#[derive(Debug, Clone)]
pub struct GroupCreate {
    pub name: String,
    pub description: Option<String>,
    pub author_id: Uuid,
    pub picture: Option<String>,
    pub users: Vec<Uuid>,
}

/// Structure passed to the repository for editing a group
#[derive(Debug, Clone)]
pub struct GroupUpdate {
    pub id: Uuid,
    pub name: Option<String>,
    pub description: Option<String>,
    pub picture: Option<String>,
}

/// Structure passed to the repository for deleting a group
#[derive(Debug, Clone)]
pub struct GroupDelete {
    pub id: Uuid,
}

/// Structure passed to the repository when trying to find a group by its id
#[derive(Debug, Clone)]
pub struct GroupGetById {
    pub id: Uuid,
}

impl GroupGetById {
    #[inline]
    pub const fn new(id: &Uuid) -> Self {
        Self { id: *id }
    }
}

/// Connection between group and its members
pub struct GroupUser {
    pub id: Uuid,
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Structure passed to the repository for getting user membership by user_id and group_id
#[derive(Debug, Clone)]
pub struct GetGroupUserByIds {
    pub user_id: Uuid,
    pub group_id: Uuid,
}

impl GetGroupUserByIds {
    #[inline]
    pub const fn new(user_id: &Uuid, group_id: &Uuid) -> Self {
        Self {
            user_id: *user_id,
            group_id: *group_id,
        }
    }
}

/// Structure passed to the repository for adding a user to a group
#[derive(Debug, Clone)]
pub struct GroupUserCreate {
    pub user_id: Uuid,
    pub group_id: Uuid,
}

/// Structure passed to the repository for deleting a user from a group
#[derive(Debug, Clone)]
pub struct GroupUserDelete {
    pub user_id: Uuid,
    pub group_id: Uuid,
}

impl GroupUserDelete {
    #[inline]
    pub const fn new(user_id: &Uuid, group_id: &Uuid) -> Self {
        Self {
            user_id: *user_id,
            group_id: *group_id,
        }
    }
}

/// Structure passed to the repository for getting groups of a user
#[derive(Debug, Clone)]
pub struct GroupGetGroupsByUser {
    pub user_id: Uuid,
}

impl GroupGetGroupsByUser {
    #[inline]
    pub const fn new(user_id: &Uuid) -> Self {
        Self { user_id: *user_id }
    }
}
