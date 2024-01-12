use async_trait::async_trait;
use sqlx::{Postgres, QueryBuilder, Transaction};

use crate::db::common::error::{BusinessLogicError, BusinessLogicErrorKind, DbError, DbResultMultiple, DbResultSingle};
use crate::db::common::{DbCreate, DbDelete, DbPoolHandler, DbReadMany, DbReadOne, DbRepository, DbUpdate, PoolHandler};
use crate::db::models::{UserGetByUsername, UserLogin, UserPreview};
use crate::db::models::{User, UserCreate, UserDelete, UserGetById, UserUpdate};

pub struct UserRepository {
    pool_handler: PoolHandler,
}

impl UserRepository {
    /// Function which retrieves a user by their id, usable within a transaction
    ///
    /// # Params
    /// - params: structure containing the id of the user
    /// - transaction_handle mutable reference to an ongoing transaction
    ///
    /// # Returns
    /// - Ok(user): on successful connection and retrieval
    /// - Err(_): otherwise
    pub async fn get_user<'a>(
        params: UserGetById,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT *
            FROM "User"
            WHERE id = $1
            "#,
            params.id
        )
            .fetch_optional(transaction_handle.as_mut())
            .await?;

        Ok(user)
    }

    /// Function which checks if the user is correct (existing and not deleted)
    ///
    /// # Params
    /// - user: optional user retrieved from the database
    ///
    /// # Returns
    /// - Ok(user): when the user exists and is not deleted
    /// - Err(DbError): with appropriate error description otherwise
    pub fn user_is_correct(user: Option<User>) -> DbResultSingle<User> {
        match user {
            Some(
                user @ User {
                    deleted_at: None, ..
                },
            ) => Ok(user),
            Some(_) => Err(DbError::from(BusinessLogicError::new(BusinessLogicErrorKind::UserDeleted))),
            None => Err(DbError::from(BusinessLogicError::new(BusinessLogicErrorKind::UserDoesNotExist))),
        }
    }
}

#[async_trait]
impl DbRepository for UserRepository {
    #[inline]
    fn new(pool_handler: PoolHandler) -> Self {
        Self { pool_handler }
    }

    #[inline]
    async fn disconnect(&mut self) -> () {
        self.pool_handler.disconnect().await;
    }
}

#[async_trait]
impl DbReadOne<UserLogin, User> for UserRepository {
    async fn read_one(&mut self, params: &UserLogin) -> DbResultSingle<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT *
            FROM "User"
            WHERE email = $1 AND password_hash = $2
            "#,
            params.email,
            params.password_hash
        )
            .fetch_optional(&*self.pool_handler.pool)
            .await?;

        Self::user_is_correct(user).map_err(|_| {
            DbError::from(BusinessLogicError::new(BusinessLogicErrorKind::UserPasswordDoesNotMatch))
        })
    }
}

#[async_trait]
impl DbReadMany<UserGetByUsername, UserPreview> for UserRepository {
    /// Finds users in the database with username containing given substring
    async fn read_many(&mut self, params: &UserGetByUsername) -> DbResultMultiple<UserPreview> {
        let users = sqlx::query_as!(
            UserPreview,
            r#"
            SELECT id, username, profile_picture
            FROM "User"
            WHERE LOWER(username) LIKE $1 AND deleted_at IS NULL
            "#,
            params.username.to_lowercase()
        )
            .fetch_all(&*self.pool_handler.pool)
            .await?;

        Ok(users)
    }
}

#[async_trait]
impl DbCreate<UserCreate, User> for UserRepository {
    /// Create a new user with the specified data
    async fn create(&mut self, data: &UserCreate) -> DbResultSingle<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO "User" (username, email, profile_picture, password_hash, password_salt)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
            data.username,
            data.email,
            data.profile_picture,
            data.password_hash,
            data.password_salt
        )
            .fetch_one(&*self.pool_handler.pool)
            .await?;

        Ok(user)
    }
}

#[async_trait]
impl DbUpdate<UserUpdate, User> for UserRepository {
    async fn update(&mut self, params: &UserUpdate) -> DbResultMultiple<User> {
        let columns_and_params = [
            ("username", &params.username),
            ("email", &params.email),
            ("profile_picture", &params.profile_picture),
            ("password_hash", &params.password_hash),
            ("password_salt", &params.password_salt),
        ];

        // Check if all parameters are none
        if columns_and_params.map(|x| x.1).iter().all(|x| x.is_none()) {
            return Err(DbError::from(BusinessLogicError::new(BusinessLogicErrorKind::UpdateParametersEmpty)));
        }

        let mut tx = self.pool_handler.pool.begin().await?;

        let user = Self::get_user(UserGetById::new(&params.id), &mut tx).await?;
        Self::user_is_correct(user)?;

        // Start building the query
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(r#"UPDATE "User" SET "#);

        // Use seperated to separate changed columns by comma
        let mut seperated = query_builder.separated(", ");

        for (column, value) in columns_and_params {
            if let Some(value) = value {
                seperated.push(format!("{column} = "));
                seperated.push_bind_unseparated(value);
            }
        }

        // Bind id of the user
        query_builder.push(" WHERE id = ");
        query_builder.push_bind(params.id);
        query_builder.push(" RETURNING *");

        // Construct the query and run it
        let user = query_builder
            .build_query_as()
            .fetch_all(tx.as_mut())
            .await?;

        tx.commit().await?;

        Ok(user)
    }
}

#[async_trait]
impl DbDelete<UserDelete, User> for UserRepository {
    async fn delete(&mut self, params: &UserDelete) -> DbResultMultiple<User> {
        let mut tx = self.pool_handler.pool.begin().await?;

        let user = Self::get_user(UserGetById::new(&params.id), &mut tx).await?;
        Self::user_is_correct(user)?;

        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE "User"
            SET deleted_at = now(), email = $1, username = $1
            WHERE id = $1
            RETURNING *
            "#,
            params.id
        )
            .fetch_all(tx.as_mut())
            .await?;

        tx.commit().await?;

        Ok(user)
    }
}