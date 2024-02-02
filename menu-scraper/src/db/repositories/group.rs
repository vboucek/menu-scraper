use crate::db::common::error::{
    BusinessLogicError, BusinessLogicErrorKind, DbError, DbResultMultiple, DbResultSingle,
};
use crate::db::common::{
    DbCreate, DbDelete, DbReadMany, DbReadOne, DbRepository, DbUpdate, PoolHandler,
};
use crate::db::models::{
    GetGroupUserByIds, Group, GroupCreate, GroupDelete, GroupGetById, GroupGetGroupsByUser,
    GroupPreview, GroupUpdate, GroupUser, GroupUserCreate, GroupUserDelete, UserGetById,
    UserPreview,
};
use crate::db::repositories::UserRepository;
use async_trait::async_trait;
use sqlx::{Postgres, QueryBuilder, Transaction};
use uuid::Uuid;

#[derive(Clone)]
pub struct GroupRepository {
    pool_handler: PoolHandler,
}

impl GroupRepository {
    /// Function which retrieves a group by its id, usable within a transaction
    ///
    /// # Params
    /// - params: structure containing the id of the menu
    /// - transaction_handle mutable reference to an ongoing transaction
    ///
    /// # Returns
    /// - Ok(group): on successful connection and retrieval
    /// - Err(_): otherwise
    pub async fn get_group<'a>(
        params: &GroupGetById,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<Option<Group>> {
        let group = sqlx::query_as!(
            Group,
            r#"
            SELECT *
            FROM "Group" G
            WHERE G.id = $1
            "#,
            params.id
        )
        .fetch_optional(transaction_handle.as_mut())
        .await?;

        Ok(group)
    }

    /// Function which checks if the group is correct (existing and not deleted)
    ///
    /// # Params
    /// - group: optional group retrieved from the database
    ///
    /// # Returns
    /// - Ok(group): when the group exists and is not deleted
    /// - Err(DbError): with appropriate error description otherwise
    pub fn group_is_correct(group: Option<Group>) -> DbResultSingle<Group> {
        match group {
            Some(
                group @ Group {
                    deleted_at: None, ..
                },
            ) => Ok(group),
            Some(_) => Err(DbError::from(BusinessLogicError::new(
                BusinessLogicErrorKind::GroupDeleted,
            ))),
            None => Err(DbError::from(BusinessLogicError::new(
                BusinessLogicErrorKind::GroupDoesNotExist,
            ))),
        }
    }

    /// Function which retrieves a group's user by its id, usable within a transaction
    ///
    /// # Params
    /// - params: structure containing the group_id and user_id
    /// - transaction_handle mutable reference to an ongoing transaction
    ///
    /// # Returns
    /// - Ok(GroupUser): on successful connection and retrieval
    /// - Err(_): otherwise
    pub async fn get_group_user<'a>(
        params: &GetGroupUserByIds,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<Option<GroupUser>> {
        let group_user = sqlx::query_as!(
            GroupUser,
            r#"
            SELECT *
            FROM "GroupUsers" G
            WHERE G.user_id = $1 AND G.group_id = $2
            "#,
            params.user_id,
            params.group_id
        )
        .fetch_optional(transaction_handle.as_mut())
        .await?;

        Ok(group_user)
    }

    /// Function which checks if the group membership is correct
    ///
    /// # Params
    /// - group_user: optional GroupUser retrieved from the database
    ///
    /// # Returns
    /// - Ok(GroupUser): when the user's membership exists and is not deleted
    /// - Err(DbError): with appropriate error description otherwise
    pub fn group_user_is_correct(group_user: Option<GroupUser>) -> DbResultSingle<GroupUser> {
        match group_user {
            Some(
                group_user @ GroupUser {
                    deleted_at: None, ..
                },
            ) => Ok(group_user),
            Some(_) => Err(DbError::from(BusinessLogicError::new(
                BusinessLogicErrorKind::GroupUsersDeleted,
            ))),
            None => Err(DbError::from(BusinessLogicError::new(
                BusinessLogicErrorKind::GroupUsersDoesNotExist,
            ))),
        }
    }

    /// Checks if user is correct, group is correct and user is a member of the group, returns error if not
    pub async fn check_user_is_member<'a>(
        tx: &mut Transaction<'a, Postgres>,
        user_id: &Uuid,
        group_id: &Uuid,
    ) -> DbResultSingle<()> {
        // Check that user is correct
        let user = UserRepository::get_user(&UserGetById::new(user_id), tx).await?;
        UserRepository::user_is_correct(user)?;

        // Check that group is correct
        let group = Self::get_group(&GroupGetById::new(group_id), tx).await?;
        let group = Self::group_is_correct(group)?;

        let group_user =
            Self::get_group_user(&GetGroupUserByIds::new(user_id, group_id), tx).await?;

        // Author is implicitly in the group
        if group.author_id == *user_id || Self::group_user_is_correct(group_user).is_ok() {
            return Ok(());
        }

        Err(DbError::from(BusinessLogicError::new(
            BusinessLogicErrorKind::UserNotMemberOfGroup,
        )))
    }
}

#[async_trait]
impl DbRepository for GroupRepository {
    #[inline]
    fn new(pool_handler: PoolHandler) -> Self {
        Self { pool_handler }
    }
}

#[async_trait]
impl DbReadOne<GroupGetById, Group> for GroupRepository {
    /// Gets one group from the database
    async fn read_one(&self, params: &GroupGetById) -> DbResultSingle<Group> {
        let mut tx = self.pool_handler.pool.begin().await?;

        let group = Self::get_group(params, &mut tx).await?;
        let group = Self::group_is_correct(group)?;
        tx.commit().await?;

        Ok(group)
    }
}

#[async_trait]
impl DbCreate<GroupCreate, Group> for GroupRepository {
    /// Create a new group with specified data
    async fn create(&self, data: &GroupCreate) -> DbResultSingle<Group> {
        let mut tx = self.pool_handler.pool.begin().await?;

        // Check if the author of the group is correct
        let author = UserRepository::get_user(&UserGetById::new(&data.author_id), &mut tx).await?;
        UserRepository::user_is_correct(author)?;

        let group = sqlx::query_as!(
            Group,
            r#"
            INSERT INTO "Group" (
                name, description, author_id, picture
            )
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
            data.name,
            data.description,
            data.author_id,
            data.picture
        )
        .fetch_one(tx.as_mut())
        .await?;

        // Add users
        for user_id in data.users.iter() {
            let user = UserRepository::get_user(&UserGetById::new(user_id), &mut tx).await?;
            UserRepository::user_is_correct(user)?;

            sqlx::query!(
                r#"
                INSERT INTO "GroupUsers" (user_id, group_id)
                VALUES ($1, $2)
                "#,
                user_id,
                group.id
            )
            .execute(tx.as_mut())
            .await?;
        }

        tx.commit().await?;

        Ok(group)
    }
}

#[async_trait]
impl DbReadMany<GroupGetGroupsByUser, GroupPreview> for GroupRepository {
    /// Returns list of group previews for a user (groups he created or is a member)
    async fn read_many(&self, params: &GroupGetGroupsByUser) -> DbResultMultiple<GroupPreview> {
        let groups = sqlx::query_as!(
            GroupPreview,
            r#"
            SELECT DISTINCT G.id AS id, name, G.picture AS picture
            FROM "Group" G LEFT OUTER JOIN "GroupUsers" U ON G.id = U.group_id
            WHERE (G.author_id = $1 OR U.user_id = $1) AND G.deleted_at IS NULL AND U.deleted_at IS NULL
            "#,
            params.user_id
        )
        .fetch_all(&*self.pool_handler.pool)
        .await?;

        Ok(groups)
    }
}

#[async_trait]
impl DbDelete<GroupDelete, Group> for GroupRepository {
    /// Deletes one group from the database by its id
    async fn delete(&self, params: &GroupDelete) -> DbResultMultiple<Group> {
        let mut tx = self.pool_handler.pool.begin().await?;

        let group = Self::get_group(&GroupGetById::new(&params.id), &mut tx).await?;
        Self::group_is_correct(group)?;

        let deleted_group = sqlx::query_as!(
            Group,
            r#"
            UPDATE "Group"
            SET deleted_at = now()
            WHERE id = $1
            RETURNING *
            "#,
            params.id
        )
        .fetch_one(tx.as_mut())
        .await?;

        sqlx::query!(
            r#"
            UPDATE "GroupUsers"
            SET deleted_at = now()
            WHERE group_id = $1
            "#,
            params.id
        )
        .execute(tx.as_mut())
        .await?;

        tx.commit().await?;

        Ok(vec![deleted_group])
    }
}

#[async_trait]
impl DbUpdate<GroupUpdate, Group> for GroupRepository {
    /// Updates one group in the database
    async fn update(&self, params: &GroupUpdate) -> DbResultMultiple<Group> {
        let columns_and_params = [
            ("name", &params.name),
            ("description", &params.description),
            ("picture", &params.picture),
        ];

        // Check if all parameters are none
        if columns_and_params.map(|x| x.1).iter().all(|x| x.is_none()) {
            return Err(DbError::from(BusinessLogicError::new(
                BusinessLogicErrorKind::UpdateParametersEmpty,
            )));
        }

        let mut tx = self.pool_handler.pool.begin().await?;

        let group = Self::get_group(&GroupGetById::new(&params.id), &mut tx).await?;
        Self::group_is_correct(group)?;

        // Start building the query
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(r#"UPDATE "Group" SET "#);

        // Use separated to separate changed columns by comma
        let mut separated = query_builder.separated(", ");

        for (column, value) in columns_and_params {
            if let Some(value) = value {
                separated.push(format!("{column} = "));
                separated.push_bind_unseparated(value);
            }
        }

        // Bind id of the group
        query_builder.push(" WHERE id = ");
        query_builder.push_bind(params.id);
        query_builder.push(" RETURNING *");

        // Construct the query and run it
        let updated_group = query_builder
            .build_query_as()
            .fetch_all(tx.as_mut())
            .await?;

        tx.commit().await?;

        Ok(updated_group)
    }
}

#[async_trait]
pub trait GroupRepositoryListUsers {
    /// List previews of users which are members of a group
    async fn list_group_users(&self, params: &GroupGetById) -> DbResultMultiple<UserPreview>;
}

#[async_trait]
impl GroupRepositoryListUsers for GroupRepository {
    async fn list_group_users(&self, params: &GroupGetById) -> DbResultMultiple<UserPreview> {
        let mut tx = self.pool_handler.pool.begin().await?;

        let group = Self::get_group(&GroupGetById::new(&params.id), &mut tx).await?;
        Self::group_is_correct(group)?;

        let mut users = sqlx::query_as!(
            UserPreview,
            r#"
            SELECT U.id AS id, U.username AS username, U.profile_picture AS profile_picture
            FROM "Group" G
            JOIN "GroupUsers" GU ON G.id = GU.group_id
            JOIN "User" U ON U.id = GU.user_id
            WHERE G.id = $1 AND GU.deleted_at IS NULL AND U.deleted_at IS NULL
            "#,
            params.id
        )
        .fetch_all(tx.as_mut())
        .await?;

        let author = sqlx::query_as!(
            UserPreview,
            r#"
            SELECT U.id AS id, U.username AS username, U.profile_picture AS profile_picture
            FROM "Group" G
            JOIN "User" U ON G.author_id = U.id
            WHERE G.id = $1 AND U.deleted_at IS NULL
            "#,
            params.id
        )
        .fetch_all(tx.as_mut())
        .await?;

        tx.commit().await?;

        users.extend(author);

        users.sort_by(|a, b| a.username.cmp(&b.username));

        Ok(users)
    }
}

#[async_trait]
pub trait GroupRepositoryAddUser {
    /// Add user to a group
    async fn add_user_to_group(&self, params: &GroupUserCreate) -> DbResultSingle<()>;
}

#[async_trait]
impl GroupRepositoryAddUser for GroupRepository {
    async fn add_user_to_group(&self, params: &GroupUserCreate) -> DbResultSingle<()> {
        let mut tx = self.pool_handler.pool.begin().await?;

        // Check that group is correct and user is a member
        if Self::check_user_is_member(&mut tx, &params.user_id, &params.group_id)
            .await
            .is_ok()
        {
            return Err(DbError::from(BusinessLogicError::new(
                BusinessLogicErrorKind::UserAlreadyInGroup,
            )));
        };

        sqlx::query!(
            r#"
            INSERT INTO "GroupUsers" (user_id, group_id)
            VALUES ($1, $2)
            ON CONFLICT(user_id, group_id) DO UPDATE SET deleted_at = NULL;
            "#,
            params.user_id,
            params.group_id
        )
        .execute(tx.as_mut())
        .await?;

        tx.commit().await?;

        Ok(())
    }
}

#[async_trait]
pub trait GroupRepositoryRemoveUser {
    /// Remove user from a group
    async fn remove_user_from_group(&self, params: &GroupUserDelete) -> DbResultSingle<()>;
}

#[async_trait]
impl GroupRepositoryRemoveUser for GroupRepository {
    async fn remove_user_from_group(&self, params: &GroupUserDelete) -> DbResultSingle<()> {
        let mut tx = self.pool_handler.pool.begin().await?;

        // Check that user really is in the group
        let group_user = Self::get_group_user(
            &GetGroupUserByIds::new(&params.user_id, &params.group_id),
            &mut tx,
        )
        .await?;
        Self::group_user_is_correct(group_user)?;

        sqlx::query!(
            r#"
            UPDATE "GroupUsers" SET deleted_at = now()
            WHERE user_id = $1 AND group_id = $2
            "#,
            params.user_id,
            params.group_id
        )
        .execute(tx.as_mut())
        .await?;

        // Remove votes by deleted user
        sqlx::query!(
            r#"
            UPDATE "Vote" SET deleted_at = now()
            FROM "Vote" V JOIN "Lunch" L ON V.lunch_id = L.id
            WHERE V.user_id = $1 AND L.group_id = $2
            "#,
            params.user_id,
            params.group_id
        )
            .execute(tx.as_mut())
            .await?;

        tx.commit().await?;

        Ok(())
    }
}
