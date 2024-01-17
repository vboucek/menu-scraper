use std::fmt::{Debug, Display, Formatter};

use BusinessLogicErrorKind::{*};

#[derive(Debug)]
pub enum BusinessLogicErrorKind {
    // User errors
    UserDoesNotExist,
    UserDeleted,
    UserPasswordDoesNotMatch,
    EmailAlreadyUsed,
    UsernameAlreadyUsed,

    // Restaurant errors
    // --------------------------
    RestaurantDoesNotExist,
    RestaurantDeleted,

    // Menu errors
    // --------------------------
    MenuDoesNotExist,
    MenuDeleted,

    // Group errors
    // --------------------------
    GroupDoesNotExist,
    GroupDeleted,

    // GroupUsers errors
    // --------------------------
    GroupUsersDoesNotExist,
    GroupUsersDeleted,
    UserAlreadyInGroup,

    // Lunch errors
    // --------------------------
    LunchDoesNotExist,
    LunchDeleted,
    LunchForDateAlreadyExists,

    // Vote errors
    // --------------------------
    VoteDoesNotExist,
    VoteDeleted,
    UserAlreadyVoted,
    LunchDateDoesntMatchMenuDate,

    // Generic errors
    UpdateParametersEmpty,
}

#[derive(Debug)]
pub enum DbErrorType {
    BusinessLogic(BusinessLogicErrorKind),
    SqlxError,
}

impl Display for BusinessLogicErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let does_not_exist = |name: &str| format!("The specified {name} does not exist!");
        let deleted = |name: &str| format!("The specified {name} has been deleted!");

        match self {
            UserDoesNotExist => f.write_str(does_not_exist("user").as_str()),
            UserDeleted => f.write_str(deleted("user").as_str()),
            RestaurantDoesNotExist => f.write_str(does_not_exist("restaurant").as_str()),
            RestaurantDeleted => f.write_str(deleted("restaurant").as_str()),
            MenuDoesNotExist => f.write_str(does_not_exist("menu").as_str()),
            MenuDeleted => f.write_str(deleted("menu").as_str()),
            GroupDoesNotExist => f.write_str(does_not_exist("group").as_str()),
            GroupDeleted => f.write_str(deleted("group").as_str()),
            UserPasswordDoesNotMatch => {
                write!(
                    f,
                    "The provided email and password combination is incorrect."
                )
            }
            UpdateParametersEmpty => {
                write!(
                    f,
                    concat!(
                    "The provided parameters for update query are incorrect",
                    " (no field would be changed)."
                    )
                )
            }
            UserAlreadyInGroup => {
                write!(
                    f,
                    "User is already in this group or is the author of the group."
                )
            }
            GroupUsersDoesNotExist => {
                write!(
                    f,
                    "Given user is not in the group."
                )
            }
            GroupUsersDeleted => {
                write!(
                    f,
                    "User is already deleted from this group."
                )
            }
            LunchDoesNotExist => f.write_str(does_not_exist("lunch").as_str()),
            LunchDeleted => f.write_str(deleted("lunch").as_str()),
            LunchForDateAlreadyExists => {
                write!(
                    f,
                    "Lunch for given day already exists."
                )
            }
            VoteDoesNotExist => f.write_str(does_not_exist("vote").as_str()),
            VoteDeleted => f.write_str(deleted("vote").as_str()),
            UserAlreadyVoted => {
                write!(
                    f,
                    "Given user already voted in this lunch."
                )
            }
            LunchDateDoesntMatchMenuDate => {
                write!(
                    f,
                    "Menu must be for the same day as lunch."
                )
            }
            EmailAlreadyUsed => {
                write!(
                    f,
                    "This email is already used."
                )
            }
            UsernameAlreadyUsed => {
                write!(
                    f,
                    "This username is already used."
                )
            }
        }
    }
}

/// Error type representing a Business Logic Error in the database layer ->
/// usually a problem with missing records, insufficient rights for operation, and so on.
pub struct BusinessLogicError {
    error: BusinessLogicErrorKind,
}

impl BusinessLogicError {
    /// Business Logic Error constructor
    #[inline]
    pub const fn new(error: BusinessLogicErrorKind) -> Self {
        Self { error }
    }

    /// Formatted business logic error
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl Display for BusinessLogicError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}

impl Debug for BusinessLogicError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}

pub struct DbError {
    /// Error type to distinguish type of the the error - either sqlx error or business logic error
    pub error_type: DbErrorType,
    description: String,
}

/// Error encapsulating errors from `sqlx` and our own `BusinessLogicError`, unifying errors from
/// the database without the need of `anyhow` library.
impl DbError {
    /// Database Error constructor
    #[inline]
    pub fn new(description: &str, error_type: DbErrorType) -> Self {
        Self {
            error_type,
            description: description.to_owned(),
        }
    }
    /// Formatted database error
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl Display for DbError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}

impl Debug for DbError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}

/// The database error can be assigned to `dyn Error`
impl std::error::Error for DbError {
    fn description(&self) -> &str {
        &self.description
    }
}

/// Conversion from sqlx error, useful when using `?` operator
impl From<sqlx::Error> for DbError {
    fn from(value: sqlx::Error) -> Self {
        Self::new(&format!("sqlx error: {value}"), DbErrorType::SqlxError)
    }
}

/// Conversion from sqlx migrate error, useful when using `?` operator
impl From<sqlx::migrate::MigrateError> for DbError {
    fn from(value: sqlx::migrate::MigrateError) -> Self {
        Self::new(&format!("Migration error: {value}"), DbErrorType::SqlxError)
    }
}

/// Conversion from business logic error
impl From<BusinessLogicError> for DbError {
    fn from(value: BusinessLogicError) -> Self {
        Self::new(value.to_string().as_str(), DbErrorType::BusinessLogic(value.error))
    }
}

/// generic database result
pub type DbResult<T> = Result<T, DbError>;

/// Syntax sugar type denoting a singular result from the database
pub type DbResultSingle<T> = DbResult<T>;
/// Syntax sugar type denoting multiple results from the database
pub type DbResultMultiple<T> = DbResult<Vec<T>>;
