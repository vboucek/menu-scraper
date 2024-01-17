use async_trait::async_trait;
use sqlx::{Postgres, Transaction};
use crate::db::common::error::{BusinessLogicError, BusinessLogicErrorKind, DbError, DbResultMultiple, DbResultSingle};
use crate::db::common::{DbCreate, DbDelete, DbPoolHandler, DbReadMany, DbRepository, PoolHandler};
use crate::db::common::error::BusinessLogicErrorKind::{UserAlreadyVoted};
use crate::db::models::{LunchGetById, MenuGetById, MenuItem, MenuWithRestaurantAndVotes, UserGetById, Vote, VoteCreate, VoteDelete, VoteGetById, VoteGetMany, VotePreview};
use crate::db::repositories::{LunchRepository, MenuRepository, UserRepository};

#[derive(Clone)]
pub struct VoteRepository {
    pool_handler: PoolHandler,
}

impl VoteRepository {
    /// Function which retrieves a vote by its id, usable within a transaction
    ///
    /// # Params
    /// - params: structure containing the id of the vote
    /// - transaction_handle mutable reference to an ongoing transaction
    ///
    /// # Returns
    /// - Ok(vote): on successful connection and retrieval
    /// - Err(_): otherwise
    pub async fn get_vote<'a>(
        params: &VoteGetById,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<Option<Vote>> {
        let vote = sqlx::query_as!(
            Vote,
            r#"
            SELECT *
            FROM "Vote" V
            WHERE V.id = $1
            "#,
            params.id
        )
            .fetch_optional(transaction_handle.as_mut())
            .await?;

        Ok(vote)
    }

    /// Function which checks if the vote is correct (existing and not deleted)
    ///
    /// # Params
    /// - vote: optional vote retrieved from the database
    ///
    /// # Returns
    /// - Ok(vote): when the vote exists and is not deleted
    /// - Err(DbError): with appropriate error description otherwise
    pub fn vote_is_correct(vote: Option<Vote>) -> DbResultSingle<Vote> {
        match vote {
            Some(
                vote @ Vote {
                    deleted_at: None, ..
                },
            ) => Ok(vote),
            Some(_) => Err(DbError::from(BusinessLogicError::new(BusinessLogicErrorKind::VoteDeleted))),
            None => Err(DbError::from(BusinessLogicError::new(BusinessLogicErrorKind::VoteDoesNotExist))),
        }
    }
}

#[async_trait]
impl DbRepository for VoteRepository {
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
impl DbCreate<VoteCreate, Vote> for VoteRepository {
    /// Creates a new vote for some lunch
    async fn create(&mut self, data: &VoteCreate) -> DbResultSingle<Vote> {
        let mut tx = self.pool_handler.pool.begin().await?;

        // Check if user, menu and lunch are correct
        let menu = MenuRepository::get_menu(&MenuGetById::new(&data.menu_id), &mut tx).await?;
        let menu = MenuRepository::menu_is_correct(menu)?;

        let user = UserRepository::get_user(&UserGetById::new(&data.user_id), &mut tx).await?;
        UserRepository::user_is_correct(user)?;

        let lunch = LunchRepository::get_lunch(&LunchGetById::new(&data.lunch_id), &mut tx).await?;
        let lunch = LunchRepository::lunch_is_correct(lunch)?;

        // Check if lunch and menu have the same date
        if lunch.date != menu.date {
            return Err(DbError::from(BusinessLogicError::new(BusinessLogicErrorKind::LunchDateDoesntMatchMenuDate)));
        }

        // Check if user didn't already vote
        let vote = sqlx::query_as!(
            Vote,
            r#"
            SELECT *
            FROM "Vote"
            WHERE user_id = $1 AND lunch_id = $2 AND deleted_at IS NULL
            "#,
            data.user_id,
            data.lunch_id
        )
            .fetch_optional(tx.as_mut())
            .await?;

        if vote.is_some() {
            return Err(DbError::from(BusinessLogicError::new(UserAlreadyVoted)));
        }

        let vote = sqlx::query_as!(
            Vote,
            r#"
            INSERT INTO "Vote" (user_id, lunch_id, menu_id)
            VALUES ($1, $2, $3)
            ON CONFLICT (user_id, lunch_id) DO UPDATE SET deleted_at = NULL, menu_id = $3
            RETURNING *;
            "#,
            data.user_id,
            data.lunch_id,
            data.menu_id
        )
            .fetch_one(tx.as_mut())
            .await?;

        tx.commit().await?;

        Ok(vote)
    }
}

#[async_trait]
impl DbDelete<VoteDelete, Vote> for VoteRepository {
    async fn delete(&mut self, params: &VoteDelete) -> DbResultMultiple<Vote> {
        let mut tx = self.pool_handler.pool.begin().await?;

        // Check that vote exists and is not already deleted
        let vote = Self::get_vote(&VoteGetById::new(&params.id), &mut tx).await?;
        Self::vote_is_correct(vote)?;

        // Delete vote
        let deleted_vote = sqlx::query_as!(
            Vote,
            r#"
            UPDATE "Vote"
            SET deleted_at = now()
            WHERE id = $1
            RETURNING *
            "#,
            params.id
        )
            .fetch_one(tx.as_mut())
            .await?;

        tx.commit().await?;

        Ok(vec![deleted_vote])
    }
}

#[async_trait]
impl DbReadMany<VoteGetMany, MenuWithRestaurantAndVotes> for VoteRepository {
    /// Gets votes a lunch grouped by corresponding menu
    async fn read_many(&mut self, params: &VoteGetMany) -> DbResultMultiple<MenuWithRestaurantAndVotes> {
        let mut tx = self.pool_handler.pool.begin().await?;

        // Check that lunch exists and is not deleted
        let lunch = LunchRepository::get_lunch(&LunchGetById::new(&params.lunch_id), &mut tx).await?;
        LunchRepository::lunch_is_correct(lunch)?;

        let menu_with_votes = sqlx::query_as!(
            MenuWithRestaurantAndVotes,
            r#"
            SELECT
                R.id AS restaurant_id,
                R.name AS name,
                R.street AS street,
                R.house_number AS house_number,
                R.zip_code AS zip_code,
                R.city AS city,
                R.picture AS picture,
                M.id AS menu_id,
                M.date AS date,
                ARRAY_AGG(DISTINCT I.*) AS "items!: Vec<MenuItem>",
                ARRAY_AGG(DISTINCT (V.id, V.user_id)) AS "votes!: Vec<VotePreview>"
            FROM "Restaurant" AS R
            JOIN "Menu" AS M ON R.id = M.restaurant_id
            JOIN "MenuItem" AS I ON M.id = I.menu_id
            JOIN "Vote" AS V ON V.menu_id = M.id
            WHERE V.lunch_id = $1
            GROUP BY R.id, R.name, R.street, R.house_number, R.zip_code, R.city, R.picture, M.id, M.date
            ORDER BY R.name;
            "#,
            params.lunch_id
        )
            .fetch_all(tx.as_mut())
            .await?;

        tx.commit().await?;

        Ok(menu_with_votes)
    }
}