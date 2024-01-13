#[cfg(test)]
pub mod menu_repo_test {
    use std::sync::Arc;

    use chrono::NaiveDate;
    use sqlx::PgPool;
    use db::db::common::{error::DbResultSingle, DbReadMany, DbCreate, DbPoolHandler, DbRepository, query_parameters::DbOrder, PoolHandler, DbUpdate};
    use db::db::models::{GroupCreate, GroupGetById, GroupGetGroupsByUser, GroupUserCreate, GroupUserDelete, MenuCreate, MenuItemCreate, MenuReadMany, RestaurantCreate, RestaurantOrderingMethod, UserCreate, UserGetByUsername, UserUpdate};
    use db::db::repositories::{GroupRepository, GroupRepositoryAddUser, GroupRepositoryListUsers, GroupRepositoryRemoveUser, MenuRepository, RestaurantRepository, UserRepository};

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

    #[sqlx::test()]
    async fn user_group_integration_test(pool: PgPool) -> DbResultSingle<()> {
        let arc_pool = Arc::new(pool);

        let mut user_repository = UserRepository::new(PoolHandler::new(arc_pool.clone()));

        let new_user = UserCreate {
            username: "Jacky123".to_string(),
            email: "jacky123@gmail.com".to_string(),
            profile_picture: None,
            password_hash: "123456789".to_string(),
            password_salt: "123456789".to_string(),
        };

        let new_user2 = UserCreate {
            username: "SpeedDemon".to_string(),
            email: "speederino@gmail.com".to_string(),
            profile_picture: None,
            password_hash: "123456789".to_string(),
            password_salt: "123456789".to_string(),
        };

        // Create users
        let user = user_repository.create(&new_user).await?;
        let user2 = user_repository.create(&new_user2).await?;

        assert_eq!(user.username, new_user.username);
        assert_eq!(user.email, new_user.email);
        assert_eq!(user2.username, new_user2.username);
        assert_eq!(user2.email, new_user2.email);

        let edit_user = UserUpdate {
            id: user.id,
            username: Some("Jacky1234".to_string()),
            email: None,
            profile_picture: None,
            password_hash: None,
            password_salt: None,
        };

        // Edit user
        let user = user_repository.update(&edit_user).await?;

        assert_eq!(user.len(), 1);

        let user = user[0].to_owned();

        assert_eq!(user.username, edit_user.username.unwrap());

        // Get users
        let users = user_repository.read_many(&UserGetByUsername::new("speed")).await?;

        assert_eq!(users.len(), 1);
        assert_eq!(users[0].username, "SpeedDemon");

        let mut group_repository = GroupRepository::new(PoolHandler::new(arc_pool.clone()));

        let new_group = GroupCreate {
            name: "Moje Group".to_string(),
            description: Some("...".to_string()),
            author_id: user.id,
        };

        // Group create
        let group = group_repository.create(&new_group).await?;

        assert_eq!(group.name, new_group.name);
        assert_eq!(group.author_id, user.id);

        // Get groups of user
        let groups = group_repository.read_many(&GroupGetGroupsByUser::new(&user.id)).await?;

        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].name, new_group.name);

        let add_user_to_group = GroupUserCreate {
            user_id: user2.id,
            group_id: group.id,
        };

        // Add user to group
        group_repository.add_user_to_group(&add_user_to_group).await?;

        let groups = group_repository.read_many(&GroupGetGroupsByUser::new(&user2.id)).await?;

        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].name, new_group.name);

        // List users in a group
        let users = group_repository.list_group_users(&GroupGetById::new(&group.id)).await?;

        assert_eq!(users.len(), 2);
        assert_eq!(users[0].username, user.username);
        assert_eq!(users[1].username, user2.username);

        // Remove user from a group

        let users = group_repository.list_group_users(&GroupGetById::new(&group.id)).await?;

        group_repository.remove_user_from_group(&GroupUserDelete::new(&user2.id, &group.id)).await?;

        let users = group_repository.list_group_users(&GroupGetById::new(&group.id)).await?;

        assert_eq!(users.len(), 1);
        assert_eq!(users[0].username, user.username);

        // Add user again
        group_repository.add_user_to_group(&add_user_to_group).await?;

        let users = group_repository.list_group_users(&GroupGetById::new(&group.id)).await?;

        assert_eq!(users.len(), 2);
        assert_eq!(users[0].username, user.username);
        assert_eq!(users[1].username, user2.username);

        Ok(())
    }
}

