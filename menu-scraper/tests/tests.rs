#[cfg(test)]
pub mod menu_repo_test {
    use std::sync::Arc;

    use chrono::{NaiveDate};
    use sqlx::PgPool;
    use db::db::common::error::DbResultSingle;
    use db::db::common::{DbReadMany, PoolHandler};
    use db::db::models::{MenuCreate, MenuItemCreate, MenuReadMany, RestaurantCreate, RestaurantOrderingMethod};
    use db::db::repositories::{MenuRepository, RestaurantRepository};
    use db::db::common::DbPoolHandler;
    use db::db::common::DbRepository;
    use db::db::common::DbCreate;
    use db::db::common::query_parameters::DbOrder;


    /// Basic integration test for checking menu repository
    #[sqlx::test()]
    async fn menu_repository_integration_test(pool: PgPool) -> DbResultSingle<()> {
        let arc_pool = Arc::new(pool);

        let mut restaurant_repo = RestaurantRepository::new(PoolHandler::new(arc_pool.clone()));

        let new_restaurant = RestaurantCreate {
            name: "Pivnice Masný Růžek".to_string(),
            street: "Křenová".to_string(),
            house_number: "70".to_string(),
            zip_code: "602 00".to_string(),
            city: "Brno-střed-Trnitá".to_string(),
            picture: None,
            phone_number: None,
            website: None,
            email: None,
            monday_open: None,
            tuesday_open: None,
            wednesday_open: None,
            thursday_open: None,
            friday_open: None,
            saturday_open: None,
            sunday_open: None,
            lunch_served: None,
        };

        let restaurant = restaurant_repo.create(&new_restaurant).await?;

        assert_eq!(restaurant.name, "Pivnice Masný Růžek");

        let mut menu_repo = MenuRepository::new(PoolHandler::new(arc_pool.clone()));

        let new_menu = MenuCreate {
            date: NaiveDate::default(),
            restaurant_id: restaurant.id,
            items: vec![MenuItemCreate {
                name: "Špagety".to_string(),
                price: 80,
                size: "200 g".to_string(),
                is_soup: false,
            }, MenuItemCreate {
                name: "Svíčková".to_string(),
                price: 80,
                size: "200 g".to_string(),
                is_soup: false,
            }],
        };

        let menu = menu_repo.create(&new_menu).await?;

        assert_eq!(menu.restaurant_id, restaurant.id);
        assert_eq!(menu.items.len(), 2);

        let new_restaurant2 = RestaurantCreate {
            name: "Pizzeria Vito".to_string(),
            street: "Křenová".to_string(),
            house_number: "70".to_string(),
            zip_code: "602 00".to_string(),
            city: "Brno-střed-Trnitá".to_string(),
            picture: None,
            phone_number: None,
            website: None,
            email: None,
            monday_open: None,
            tuesday_open: None,
            wednesday_open: None,
            thursday_open: None,
            friday_open: None,
            saturday_open: None,
            sunday_open: None,
            lunch_served: None,
        };

        let restaurant2 = restaurant_repo.create(&new_restaurant2).await?;

        assert_eq!(restaurant2.name, "Pizzeria Vito");

        let new_menu2 = MenuCreate {
            date: NaiveDate::default(),
            restaurant_id: restaurant2.id,
            items: vec![MenuItemCreate {
                name: "Quattro Formaggi".to_string(),
                price: 120,
                size: "200 g".to_string(),
                is_soup: false,
            }, MenuItemCreate {
                name: "Salát Caprese".to_string(),
                price: 120,
                size: "200 g".to_string(),
                is_soup: false,
            }],
        };

        let menu = menu_repo.create(&new_menu2).await?;

        assert_eq!(menu.restaurant_id, restaurant2.id);
        assert_eq!(menu.items.len(), 2);

        let menu_read_many = MenuReadMany {
            date_from: NaiveDate::default(),
            date_to: NaiveDate::default(),
            order_by: RestaurantOrderingMethod::Price(DbOrder::Asc),
            limit: Some(1),
            offset: Some(0),
        };

        let menus_with_restaurant = menu_repo.read_many(&menu_read_many).await?;

        assert_eq!(menus_with_restaurant[0].name, "Pivnice Masný Růžek");

        let menu_read_many = MenuReadMany {
            date_from: NaiveDate::default(),
            date_to: NaiveDate::default(),
            order_by: RestaurantOrderingMethod::Price(DbOrder::Desc),
            limit: Some(1),
            offset: Some(0),
        };

        let menus_with_restaurant = menu_repo.read_many(&menu_read_many).await?;

        assert_eq!(menus_with_restaurant[0].name, "Pizzeria Vito");

        restaurant_repo.disconnect().await;
        menu_repo.disconnect().await;
        Ok(())
    }
}

