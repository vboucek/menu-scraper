use async_trait::async_trait;
use sqlx::{Postgres, QueryBuilder, Transaction};
use crate::db::common::error::{BusinessLogicError, BusinessLogicErrorKind, DbError, DbResultMultiple, DbResultSingle};
use crate::db::common::{DbCreate, DbDelete, DbPoolHandler, DbReadOne, DbRepository, DbUpdate, PoolHandler};
use crate::db::models::{Restaurant, RestaurantCreate, RestaurantDelete, RestaurantGetById, RestaurantUpdate};

#[derive(Clone)]
pub struct RestaurantRepository {
    pool_handler: PoolHandler,
}

impl RestaurantRepository {
    /// Function which retrieves a restaurant by their id, usable within a transaction
    ///
    /// # Params
    /// - params: structure containing the id of the restaurant
    /// - transaction_handle mutable reference to an ongoing transaction
    ///
    /// # Returns
    /// - Ok(restaurant): on successful connection and retrieval
    /// - Err(_): otherwise
    pub async fn get_restaurant<'a>(
        params: RestaurantGetById,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<Option<Restaurant>> {
        // Point is not supported, so * cannot be used and columns must be specified explicitly
        let restaurant = sqlx::query_as!(
            Restaurant,
            r#"
            SELECT id, name, street, house_number, zip_code, city, picture, phone_number, website, email,
                monday_open, tuesday_open, wednesday_open, thursday_open, friday_open,
                saturday_open, sunday_open, lunch_served, deleted_at
            FROM "Restaurant"
            WHERE id = $1
            "#,
            params.id
        )
            .fetch_optional(transaction_handle.as_mut())
            .await?;

        Ok(restaurant)
    }

    /// Function which checks if the restaurant is correct (existing and not deleted)
    ///
    /// # Params
    /// - user: optional restaurant retrieved from the database
    ///
    /// # Returns
    /// - Ok(restaurant): when the restaurant exists and is not deleted
    /// - Err(DbError): with appropriate error description otherwise
    pub fn restaurant_is_correct(restaurant: Option<Restaurant>) -> DbResultSingle<Restaurant> {
        match restaurant {
            Some(
                restaurant @ Restaurant {
                    deleted_at: None, ..
                },
            ) => Ok(restaurant),
            Some(_) => Err(DbError::from(BusinessLogicError::new(BusinessLogicErrorKind::RestaurantDeleted))),
            None => Err(DbError::from(BusinessLogicError::new(BusinessLogicErrorKind::RestaurantDoesNotExist))),
        }
    }
}

#[async_trait]
impl DbRepository for RestaurantRepository {
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
impl DbReadOne<RestaurantGetById, Restaurant> for RestaurantRepository {
    /// Gets one restaurant from the database
    async fn read_one(&mut self, params: &RestaurantGetById) -> DbResultSingle<Restaurant> {
        let mut tx = self.pool_handler.pool.begin().await?;

        let restaurant = Self::get_restaurant(params.clone(), &mut tx).await?;
        let restaurant = Self::restaurant_is_correct(restaurant)?;
        tx.commit().await?;

        Ok(restaurant)
    }
}

#[async_trait]
impl DbCreate<RestaurantCreate, Restaurant> for RestaurantRepository {
    /// Create a new restaurant with the specified data
    async fn create(&mut self, data: &RestaurantCreate) -> DbResultSingle<Restaurant> {
        let restaurant = sqlx::query_as!(
            Restaurant,
            r#"
            INSERT INTO "Restaurant" (
                name, street, house_number, zip_code, city, picture, phone_number, website, email, monday_open, tuesday_open,
                wednesday_open, thursday_open, friday_open, saturday_open, sunday_open, lunch_served
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
            RETURNING id, name, street, house_number, zip_code, city, picture, phone_number, website, email,
                monday_open, tuesday_open, wednesday_open, thursday_open, friday_open, saturday_open, sunday_open,
                lunch_served, deleted_at
            "#,
            data.name,
            data.street,
            data.house_number,
            data.zip_code,
            data.city,
            data.picture,
            data.phone_number,
            data.website,
            data.email,
            data.monday_open,
            data.tuesday_open,
            data.wednesday_open,
            data.thursday_open,
            data.friday_open,
            data.saturday_open,
            data.sunday_open,
            data.lunch_served,
        )
            .fetch_one(&*self.pool_handler.pool)
            .await?;

        Ok(restaurant)
    }
}

#[async_trait]
impl DbUpdate<RestaurantUpdate, Restaurant> for RestaurantRepository {
    /// Updates one restaurant in the database
    async fn update(&mut self, params: &RestaurantUpdate) -> DbResultMultiple<Restaurant> {
        let columns_and_params = [
            ("name", &params.name),
            ("street", &params.street),
            ("house_number", &params.house_number),
            ("zip_code", &params.zip_code),
            ("city", &params.city),
            ("picture", &params.picture),
            ("phone_number", &params.phone_number),
            ("website", &params.website),
            ("email", &params.email),
            ("monday_open", &params.monday_open),
            ("tuesday_open", &params.tuesday_open),
            ("wednesday_open", &params.wednesday_open),
            ("thursday_open", &params.thursday_open),
            ("friday_open", &params.friday_open),
            ("saturday_open", &params.saturday_open),
            ("sunday_open", &params.sunday_open),
            ("lunch_served", &params.lunch_served),
        ];

        // Check if all parameters are none
        if columns_and_params.map(|x| x.1).iter().all(|x| x.is_none()) {
            return Err(DbError::from(BusinessLogicError::new(BusinessLogicErrorKind::UpdateParametersEmpty)));
        }

        let mut tx = self.pool_handler.pool.begin().await?;

        let restaurant = Self::get_restaurant(RestaurantGetById::new(&params.id), &mut tx).await?;
        Self::restaurant_is_correct(restaurant)?;

        // Start building the query
        let mut query_builder: QueryBuilder<Postgres> =
            QueryBuilder::new(r#"UPDATE "Restaurant" SET "#);

        // Use separated to separate changed columns by comma
        let mut separated = query_builder.separated(", ");

        for (column, value) in columns_and_params {
            if let Some(value) = value {
                separated.push(format!("{column} = "));
                separated.push_bind_unseparated(value);
            }
        }

        // Bind id of the restaurant
        query_builder.push(" WHERE id = ");
        query_builder.push_bind(params.id);
        query_builder.push(" RETURNING id, name, street, house_number, zip_code, picture, city, phone_number, website, email,
            monday_open, tuesday_open, wednesday_open, thursday_open, friday_open, saturday_open, sunday_open,
            lunch_served, deleted_at");

        // Construct the query and run it
        let updated_restaurant = query_builder
            .build_query_as()
            .fetch_all(tx.as_mut())
            .await?;

        tx.commit().await?;

        Ok(updated_restaurant)
    }
}


#[async_trait]
impl DbDelete<RestaurantDelete, Restaurant> for RestaurantRepository {
    async fn delete(&mut self, params: &RestaurantDelete) -> DbResultMultiple<Restaurant> {
        let mut tx = self.pool_handler.pool.begin().await?;

        let restaurant = Self::get_restaurant(RestaurantGetById::new(&params.id), &mut tx).await?;
        Self::restaurant_is_correct(restaurant)?;

        let deleted_restaurant = sqlx::query_as!(
            Restaurant,
            r#"
            UPDATE "Restaurant"
            SET deleted_at = now()
            WHERE id = $1
            RETURNING id, name, street, house_number, zip_code, city, picture, phone_number, website, email,
                monday_open, tuesday_open, wednesday_open, thursday_open, friday_open, saturday_open, sunday_open,
                lunch_served, deleted_at
            "#,
            params.id
        )
            .fetch_all(tx.as_mut())
            .await?;

        tx.commit().await?;

        Ok(deleted_restaurant)
    }
}

