use crate::db::common::error::{
    BusinessLogicError, BusinessLogicErrorKind, DbError, DbResultMultiple, DbResultSingle,
};
use crate::db::common::query_parameters::DbOrder;
use crate::db::common::{DbCreate, DbDelete, DbReadMany, DbReadOne, DbRepository, PoolHandler};
use crate::db::models::{
    DbRestaurantOrderingMethod, Menu, MenuCreate, MenuDelete, MenuGetById, MenuId, MenuReadMany,
    MenuWithRestaurant, RestaurantGetById,
};
use crate::db::models::{MenuCount, MenuGetCount, MenuItem};
use crate::db::repositories::restaurant::RestaurantRepository;
use async_trait::async_trait;
use sqlx::{Postgres, Transaction};

#[derive(Clone)]
pub struct MenuRepository {
    pool_handler: PoolHandler,
}

impl MenuRepository {
    /// Function which retrieves a menu by its id, usable within a transaction
    ///
    /// # Params
    /// - params: structure containing the id of the menu
    /// - transaction_handle mutable reference to an ongoing transaction
    ///
    /// # Returns
    /// - Ok(menu): on successful connection and retrieval
    /// - Err(_): otherwise
    pub async fn get_menu<'a>(
        params: &MenuGetById,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<Option<Menu>> {
        let menu = sqlx::query_as!(
            Menu,
            r#"
            SELECT
                M.id AS id,
                M.date AS date,
                M.restaurant_id AS restaurant_id,
                M.deleted_at AS deleted_at,
                ARRAY_AGG((I.id, I.name, I.price, I.size, I.is_soup, I.menu_id)) AS "items!: Vec<MenuItem>"
            FROM "Menu" M
            JOIN "MenuItem" I ON M.id = I.menu_id
            WHERE M.id = $1
            GROUP BY M.id, M.date, M.restaurant_id, M.deleted_at;
            "#,
            params.id
        )
            .fetch_optional(transaction_handle.as_mut())
            .await?;

        Ok(menu)
    }

    /// Function which checks if the menu is correct (existing and not deleted)
    ///
    /// # Params
    /// - menu: optional menu retrieved from the database
    ///
    /// # Returns
    /// - Ok(menu): when the menu exists and is not deleted
    /// - Err(DbError): with appropriate error description otherwise
    pub fn menu_is_correct(menu: Option<Menu>) -> DbResultSingle<Menu> {
        match menu {
            Some(
                menu @ Menu {
                    deleted_at: None, ..
                },
            ) => Ok(menu),
            Some(_) => Err(DbError::from(BusinessLogicError::new(
                BusinessLogicErrorKind::MenuDeleted,
            ))),
            None => Err(DbError::from(BusinessLogicError::new(
                BusinessLogicErrorKind::MenuDoesNotExist,
            ))),
        }
    }
}

#[async_trait]
impl DbRepository for MenuRepository {
    #[inline]
    fn new(pool_handler: PoolHandler) -> Self {
        Self { pool_handler }
    }
}

#[async_trait]
impl DbReadOne<MenuGetById, Menu> for MenuRepository {
    /// Gets one menu from the database with its items
    async fn read_one(&self, params: &MenuGetById) -> DbResultSingle<Menu> {
        let mut tx = self.pool_handler.pool.begin().await?;

        let menu = Self::get_menu(params, &mut tx).await?;
        let menu = Self::menu_is_correct(menu)?;
        tx.commit().await?;

        Ok(menu)
    }
}

#[async_trait]
impl DbCreate<MenuCreate, Menu> for MenuRepository {
    /// Create a new menu with its items
    async fn create(&self, data: &MenuCreate) -> DbResultSingle<Menu> {
        let mut tx = self.pool_handler.pool.begin().await?;

        let restaurant = RestaurantRepository::get_restaurant(
            RestaurantGetById::new(&data.restaurant_id),
            &mut tx,
        )
        .await?;
        RestaurantRepository::restaurant_is_correct(restaurant)?;

        let menu_id = sqlx::query_as!(
            MenuId,
            r#"
            INSERT INTO "Menu" (
                date, restaurant_id
            )
            VALUES ($1, $2)
            ON CONFLICT (date, restaurant_id) DO NOTHING
            RETURNING id
            "#,
            data.date,
            data.restaurant_id
        )
        .fetch_optional(tx.as_mut())
        .await?;

        let Some(menu_id) = menu_id else {
            return Ok(Menu {
                id: Default::default(),
                date: Default::default(),
                restaurant_id: Default::default(),
                deleted_at: None,
                items: vec![],
            });
        };

        for item in data.items.iter() {
            sqlx::query!(
                r#"
                INSERT INTO "MenuItem" (
                    name, price, size, is_soup, menu_id
                )
                VALUES ($1, $2, $3, $4, $5)
                "#,
                item.name,
                item.price,
                item.size,
                item.is_soup,
                menu_id.id
            )
            .execute(tx.as_mut())
            .await?;
        }

        let menu = Self::get_menu(&MenuGetById::new(&menu_id.id), &mut tx).await?;
        let menu = Self::menu_is_correct(menu)?;

        tx.commit().await?;

        Ok(menu)
    }
}

#[async_trait]
impl DbReadMany<MenuReadMany, MenuWithRestaurant> for MenuRepository {
    /// Gets menus with basic info about the restaurant as well. Supports filtering by date, pagination and ordering by
    /// distance, average price of the menu and random
    async fn read_many(&self, params: &MenuReadMany) -> DbResultMultiple<MenuWithRestaurant> {
        // Set correct ordering type
        let (order_by, ordering) = match &params.order_by {
            DbRestaurantOrderingMethod::Price(ord) => ("AVG(I.price)".to_string(), ord),
            DbRestaurantOrderingMethod::Range(ord, (long, lat)) => (
                format!(
                    "ST_DistanceSphere(
                                ST_MakePoint(longitude, latitude),
                                ST_MakePoint({long}, {lat}))"
                ),
                ord,
            ),
            DbRestaurantOrderingMethod::Random => ("RANDOM()".to_string(), &DbOrder::Asc),
            DbRestaurantOrderingMethod::Date(ord) => ("date".to_string(), ord),
        };

        // Pagination, only if limit is not None
        let pagination = if let Some(limit) = params.limit {
            format!(" LIMIT {} OFFSET {}", limit, params.offset.unwrap_or(0))
        } else {
            String::new()
        };

        let restaurant = if let Some(restaurant_id) = params.restaurant_id {
            format!("AND R.id = '{}'", restaurant_id)
        } else {
            String::new()
        };

        let query = format!(
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
                ARRAY_AGG(I.*) AS items
            FROM "Restaurant" AS R
            JOIN "Menu" AS M ON R.id = M.restaurant_id
            JOIN "MenuItem" AS I ON M.id = I.menu_id
            WHERE M.date >= $1 AND M.date <= $2 AND M.deleted_at IS NULL AND R.deleted_at IS NULL {restaurant}
            GROUP BY R.id, R.name, R.street, R.house_number, R.zip_code, R.city, R.picture, M.id, M.date
            ORDER BY {order_by} {ordering}
            {pagination}
            "#
        );

        let result = sqlx::query_as::<_, MenuWithRestaurant>(&query)
            .bind(params.date_from)
            .bind(params.date_to)
            .fetch_all(&*self.pool_handler.pool)
            .await?;

        Ok(result)
    }
}

#[async_trait]
impl DbDelete<MenuDelete, Menu> for MenuRepository {
    /// Deletes one menu from the database by its id
    async fn delete(&self, params: &MenuDelete) -> DbResultMultiple<Menu> {
        let mut tx = self.pool_handler.pool.begin().await?;

        let menu = Self::get_menu(&MenuGetById::new(&params.id), &mut tx).await?;
        Self::menu_is_correct(menu)?;

        sqlx::query_as!(
            Uuid,
            r#"
            UPDATE "Menu"
            SET deleted_at = now()
            WHERE id = $1
            "#,
            params.id
        )
        .fetch_one(tx.as_mut())
        .await?;

        let deleted_menu = Self::get_menu(&MenuGetById::new(&params.id), &mut tx).await?;

        let deleted_menu = match deleted_menu {
            Some(value) => value,
            None => {
                // Should not happen
                return Err(DbError::from(BusinessLogicError::new(
                    BusinessLogicErrorKind::MenuDoesNotExist,
                )));
            }
        };

        tx.commit().await?;

        Ok(vec![deleted_menu])
    }
}

#[async_trait]
pub trait GetNumberOfMenus {
    /// Gets number of menus for some range of dates, usable for pagination
    async fn get_number_of_menus(&self, params: &MenuGetCount) -> DbResultSingle<i64>;
}

#[async_trait]
impl GetNumberOfMenus for MenuRepository {
    async fn get_number_of_menus(&self, params: &MenuGetCount) -> DbResultSingle<i64> {
        let count = sqlx::query_as!(
            MenuCount,
            r#"
            SELECT COUNT(*) AS count
            FROM "Restaurant" AS R
            JOIN "Menu" AS M ON R.id = M.restaurant_id
            WHERE M.date >= $1 AND M.date <= $2 AND M.deleted_at IS NULL AND R.deleted_at IS NULL
            "#,
            params.date_from,
            params.date_to
        )
        .fetch_one(&*self.pool_handler.pool)
        .await?;

        Ok(count.count.unwrap_or(0))
    }
}
