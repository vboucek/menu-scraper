use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub profile_picture: Option<String>,
    pub password_hash: String,
    pub password_salt: String,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// User structure for obtaining information about other users (for adding users to some group)
#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq)]
pub struct UserPreview {
    pub id: Uuid,
    pub username: String,
    pub profile_picture: Option<String>,
}

/// Structure passed to the repository for User creation
#[derive(Debug, Clone)]
pub struct UserCreate {
    pub username: String,
    pub email: String,
    pub profile_picture: Option<String>,
    pub password_hash: String,
    pub password_salt: String,
}

/// Structure passed to the repository when trying to log in (read one == login)
#[derive(Debug, Clone)]
pub struct UserLogin {
    pub email: String,
    pub password_hash: String,
}

impl UserLogin {
    #[inline]
    pub fn new(email: &str, password_hash: &str) -> Self {
        Self {
            email: email.to_owned(),
            password_hash: password_hash.to_owned(),
        }
    }
}

/// Structure passed to the repository when trying to update a user
#[derive(Debug, Clone)]
pub struct UserUpdate {
    pub id: Uuid,
    pub username: Option<String>,
    pub email: Option<String>,
    pub profile_picture: Option<String>,
    pub password_hash: Option<String>,
    pub password_salt: Option<String>,
}

/// Structure passed to the repository when trying to delete a user
#[derive(Debug, Clone)]
pub struct UserDelete {
    pub id: Uuid,
}

impl UserDelete {
    #[inline]
    pub const fn new(id: &Uuid) -> Self {
        Self { id: *id }
    }
}

/// Structure passed to the repository when trying to find a user (generic function) for
/// transactions which check whether the specified user exists
#[derive(Debug, Clone)]
pub struct UserGetById {
    pub id: Uuid,
}

impl UserGetById {
    #[inline]
    pub const fn new(id: &Uuid) -> Self {
        Self { id: *id }
    }
}

/// Structure passed to the repository when trying to find a user by his name
#[derive(Debug, Clone)]
pub struct UserGetByUsername {
    pub username: String,
}

impl UserGetByUsername {
    #[inline]
    pub fn new(username: &str) -> Self {
        Self { username: username.to_owned() }
    }
}

/// Structure passed to the repository when checking availability of email and username
#[derive(Debug, Clone)]
pub struct CheckEmailAndUsername {
    pub edited_user_id: Option<Uuid>,
    pub username: String,
    pub email: String,
}

/// Result retrieved from the database when checking for the availability of email or username
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct CheckEmailOrUsernameResult {
    pub id: Uuid,
}

/// Structure passed to the repository when trying to get salt of the user's password
#[derive(Debug, Clone)]
pub struct UserGetPasswordSalt {
    pub email: String,
}

impl UserGetPasswordSalt {
    #[inline]
    pub fn new(email: &str) -> Self {
        Self { email: email.to_owned() }
    }
}

/// User's password salt
#[derive(Debug, Clone)]
pub struct UserPasswordSalt {
    pub password_salt: String,
}

impl UserPasswordSalt {
    #[inline]
    pub fn new(password_salt: &str) -> Self {
        Self { password_salt: password_salt.to_owned() }
    }
}
