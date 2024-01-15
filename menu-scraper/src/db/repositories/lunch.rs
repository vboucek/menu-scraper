use async_trait::async_trait;
use sqlx::{Postgres, QueryBuilder, Transaction};
use crate::db::common::error::{BusinessLogicError, BusinessLogicErrorKind, DbError, DbResultMultiple, DbResultSingle};
use crate::db::common::{DbCreate, DbDelete, DbPoolHandler, DbReadMany, DbRepository, PoolHandler};
use crate::db::common::error::BusinessLogicErrorKind::LunchForDateAlreadyExists;
use crate::db::models::{GroupGetById, Lunch, LunchCreate, LunchDelete, LunchGetById, LunchGetMany, UserGetById};
use crate::db::repositories::{GroupRepository, UserRepository};

#[derive(Clone)]
pub struct LunchRepository {
    pool_handler: PoolHandler,
}

impl LunchRepository {
    /// Function which retrieves a lunch by its id, usable within a transaction
    ///
    /// # Params
    /// - params: structure containing the id of the lunch
    /// - transaction_handle mutable reference to an ongoing transaction
    ///
    /// # Returns
    /// - Ok(lunch): on successful connection and retrieval
    /// - Err(_): otherwise
    pub async fn get_lunch<'a>(
        params: &LunchGetById,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<Option<Lunch>> {
        let lunch = sqlx::query_as!(
            Lunch,
            r#"
            SELECT *
            FROM "Lunch" L
            WHERE L.id = $1
            "#,
            params.id
        )
            .fetch_optional(transaction_handle.as_mut())
            .await?;

        Ok(lunch)
    }

    /// Function which checks if the lunch is correct (existing and not deleted)
    ///
    /// # Params
    /// - lunch: optional lunch retrieved from the database
    ///
    /// # Returns
    /// - Ok(lunch): when the lunch exists and is not deleted
    /// - Err(DbError): with appropriate error description otherwise
    pub fn lunch_is_correct(lunch: Option<Lunch>) -> DbResultSingle<Lunch> {
        match lunch {
            Some(
                lunch @ Lunch {
                    deleted_at: None, ..
                },
            ) => Ok(lunch),
            Some(_) => Err(DbError::from(BusinessLogicError::new(BusinessLogicErrorKind::LunchDeleted))),
            None => Err(DbError::from(BusinessLogicError::new(BusinessLogicErrorKind::LunchDoesNotExist))),
        }
    }
}

#[async_trait]
impl DbRepository for LunchRepository {
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
impl DbCreate<LunchCreate, Lunch> for LunchRepository {
    /// Creates a new lunch for some group
    async fn create(&mut self, data: &LunchCreate) -> DbResultSingle<Lunch> {
        let mut tx = self.pool_handler.pool.begin().await?;

        // Check if given group is correct
        let group = GroupRepository::get_group(&GroupGetById::new(&data.group_id), &mut tx).await?;
        GroupRepository::group_is_correct(group)?;

        // Check if lunch for given day doesn't already exist
        let lunch = sqlx::query_as!(
            Lunch,
            r#"
            SELECT *
            FROM "Lunch"
            WHERE date = $1 AND group_id = $2 AND deleted_at IS NULL
            "#,
            data.date,
            data.group_id
        )
            .fetch_optional(tx.as_mut())
            .await?;

        if lunch.is_some() {
            return Err(DbError::from(BusinessLogicError::new(LunchForDateAlreadyExists)));
        }

        let lunch = sqlx::query_as!(
            Lunch,
            r#"
            INSERT INTO "Lunch" (date, group_id)
            VALUES ($1, $2)
            ON CONFLICT (date, group_id) DO UPDATE SET deleted_at = NULL
            RETURNING *;
            "#,
            data.date,
            data.group_id
        )
            .fetch_one(tx.as_mut())
            .await?;

        tx.commit().await?;

        Ok(lunch)
    }
}

#[async_trait]
impl DbDelete<LunchDelete, Lunch> for LunchRepository {
    /// Deletes a lunch
    async fn delete(&mut self, params: &LunchDelete) -> DbResultMultiple<Lunch> {
        let mut tx = self.pool_handler.pool.begin().await?;

        // Check that lunch exists and is not already deleted
        let lunch = Self::get_lunch(&LunchGetById::new(&params.id), &mut tx).await?;
        Self::lunch_is_correct(lunch)?;

        // Delete lunch
        let deleted_lunch = sqlx::query_as!(
            Lunch,
            r#"
            UPDATE "Lunch"
            SET deleted_at = now()
            WHERE id = $1
            RETURNING *
            "#,
            params.id
        )
            .fetch_one(tx.as_mut())
            .await?;

        // Delete corresponding votes
        sqlx::query!(
            r#"
            UPDATE "Vote"
            SET deleted_at = now()
            WHERE lunch_id = $1
            "#,
            params.id
        )
            .execute(tx.as_mut())
            .await?;

        tx.commit().await?;

        Ok(vec![deleted_lunch])
    }
}

#[async_trait]
impl DbReadMany<LunchGetMany, Lunch> for LunchRepository {
    /// Gets lunches for a group or user between dates
    async fn read_many(&mut self, params: &LunchGetMany) -> DbResultMultiple<Lunch> {
        let mut tx = self.pool_handler.pool.begin().await?;

        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        r#"
            SELECT L.id, L.date, L.group_id, L.deleted_at
            FROM "Lunch" L
            JOIN "Group" G ON L.group_id = G.id
            LEFT OUTER JOIN "GroupUsers" GU ON G.id = GU.group_id
            WHERE G.deleted_at IS NULL
            "#
        );

        if let Some(user_id) = params.user_id {
            let user = UserRepository::get_user(&UserGetById::new(&user_id), &mut tx).await?;
            UserRepository::user_is_correct(user)?;

            query_builder.push(" AND (GU.user_id = ");
            query_builder.push_bind(user_id);
            query_builder.push(" OR G.author_id =  ");
            query_builder.push_bind(user_id);
            query_builder.push(")");
        }

        if let Some(group_id) = params.group_id {
            let group = GroupRepository::get_group(&GroupGetById::new(&group_id), &mut tx).await?;
            GroupRepository::group_is_correct(group)?;

            query_builder.push(" AND G.id = ");
            query_builder.push_bind(group_id);
        }

        if let Some(from) = params.from {
            query_builder.push(" AND date >= ");
            query_builder.push_bind(from);
        }

        if let Some(to) = params.to {
            query_builder.push(" AND date <= ");
            query_builder.push_bind(to);
        }

        let lunches = query_builder
            .build_query_as()
            .fetch_all(tx.as_mut())
            .await?;

        tx.commit().await?;

        Ok(lunches)
    }
}