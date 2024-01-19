use std::fmt::{Debug, Display, Formatter};

use BusinessLogicErrorKind::*;

#[derive(Debug)]
pub enum BusinessLogicErrorKind {
    // User errors
    UserDoesNotExist,
    UserDeleted,
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
    UserNotMemberOfGroup,
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
        match self {
            UserDoesNotExist => {
                write!(f, "Tento uživatel neexistuje.")
            }
            UserDeleted => {
                write!(f, "Tento uživatel byl odstraněn.")
            }
            RestaurantDoesNotExist => {
                write!(f, "Tato restaurace neexistuje.")
            }
            RestaurantDeleted => {
                write!(f, "Tato restaurace byla odstraněna.")
            }
            MenuDoesNotExist => {
                write!(f, "Toto menu neexistuje.")
            }
            MenuDeleted => {
                write!(f, "Toto menu bylo odstraněno.")
            }
            GroupDoesNotExist => {
                write!(f, "Taková skupina neexistuje.")
            }
            GroupDeleted => {
                write!(f, "Tato skupina byla odstraněna.")
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
                write!(f, "Tento uživatel již ve skupině je.")
            }
            GroupUsersDoesNotExist => {
                write!(f, "Tento uživatel není ve skupině.")
            }
            GroupUsersDeleted => {
                write!(f, "Uživatel je ze skupiny odstraněn.")
            }
            UserNotMemberOfGroup => {
                write!(f, "Nejste členem této skupiny.")
            }
            LunchDateDoesntMatchMenuDate => {
                write!(f, "Oběd musí být ve stejný den jako menu.")
            }
            EmailAlreadyUsed => {
                write!(f, "Tento email je již používán.")
            }
            UsernameAlreadyUsed => {
                write!(f, "Toto uživatelské jméno je již zabrané.")
            }
            LunchDoesNotExist => {
                write!(f, "Tento oběd neexistuje.")
            }
            LunchDeleted => {
                write!(f, "Tento oběd byl odstraněn.")
            }
            LunchForDateAlreadyExists => {
                write!(f, "Pro zadaný den již byl vytvořen oběd.")
            }
            VoteDoesNotExist => {
                write!(f, "Tento hlas neesixtuje.")
            }
            VoteDeleted => {
                write!(f, "Tento hlas byl odstraněn.")
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
        Self::new(
            value.to_string().as_str(),
            DbErrorType::BusinessLogic(value.error),
        )
    }
}

/// generic database result
pub type DbResult<T> = Result<T, DbError>;

/// Syntax sugar type denoting a singular result from the database
pub type DbResultSingle<T> = DbResult<T>;
/// Syntax sugar type denoting multiple results from the database
pub type DbResultMultiple<T> = DbResult<Vec<T>>;
