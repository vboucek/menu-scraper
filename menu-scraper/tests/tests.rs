#[cfg(test)]
pub mod menu_repo_test {
    use std::sync::Arc;

    use chrono::NaiveDate;
    use db::db::common::{
        error::DbResultSingle, query_parameters::DbOrder, DbCreate, DbPoolHandler, DbReadMany,
        DbRepository, DbUpdate, PoolHandler,
    };
    use db::db::models::{
        DbRestaurantOrderingMethod, GroupCreate, GroupGetById, GroupGetGroupsByUser,
        GroupUserCreate, GroupUserDelete, LunchGetMany, MenuCreate, MenuItemCreate, MenuReadMany,
        RestaurantCreate, RestaurantGetByNameAndAddress, UserCreate, UserGetByUsername, UserUpdate,
        VoteCreate, VoteGetMany,
    };
    use db::db::repositories::{
        GroupRepository, GroupRepositoryAddUser, GroupRepositoryListUsers,
        GroupRepositoryRemoveUser, LunchRepository, MenuRepository, RestaurantRepository,
        SearchRestaurant, UserRepository, VoteRepository,
    };
    use sqlx::PgPool;
    use uuid::Uuid;

    /// Basic integration test for checking menu repository
    #[sqlx::test()]
    async fn menu_repository_integration_test(pool: PgPool) -> DbResultSingle<()> {
        let arc_pool = Arc::new(pool);

        let restaurant_repo = RestaurantRepository::new(PoolHandler::new(arc_pool.clone()));

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
            longitude: None,
            latitude: None,
        };

        let restaurant = restaurant_repo.create(&new_restaurant).await?;

        assert_eq!(restaurant.name, "Pivnice Masný Růžek");

        let menu_repo = MenuRepository::new(PoolHandler::new(arc_pool.clone()));

        let new_menu = MenuCreate {
            date: NaiveDate::default(),
            restaurant_id: restaurant.id,
            items: vec![
                MenuItemCreate {
                    name: "Špagety".to_string(),
                    price: 80,
                    size: "200 g".to_string(),
                    is_soup: false,
                },
                MenuItemCreate {
                    name: "Svíčková".to_string(),
                    price: 80,
                    size: "200 g".to_string(),
                    is_soup: false,
                },
            ],
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
            longitude: None,
            latitude: None,
        };

        let restaurant2 = restaurant_repo.create(&new_restaurant2).await?;

        assert_eq!(restaurant2.name, "Pizzeria Vito");

        let new_menu2 = MenuCreate {
            date: NaiveDate::default(),
            restaurant_id: restaurant2.id,
            items: vec![
                MenuItemCreate {
                    name: "Quattro Formaggi".to_string(),
                    price: 120,
                    size: "200 g".to_string(),
                    is_soup: false,
                },
                MenuItemCreate {
                    name: "Salát Caprese".to_string(),
                    price: 120,
                    size: "200 g".to_string(),
                    is_soup: false,
                },
            ],
        };

        let menu = menu_repo.create(&new_menu2).await?;

        assert_eq!(menu.restaurant_id, restaurant2.id);
        assert_eq!(menu.items.len(), 2);

        let menu_read_many = MenuReadMany {
            date_from: NaiveDate::default(),
            date_to: NaiveDate::default(),
            restaurant_id: None,
            order_by: DbRestaurantOrderingMethod::Price(DbOrder::Asc),
            limit: Some(1),
            offset: Some(0),
        };

        let menus_with_restaurant = menu_repo.read_many(&menu_read_many).await?;

        assert_eq!(menus_with_restaurant[0].name, "Pivnice Masný Růžek");

        let menu_read_many = MenuReadMany {
            date_from: NaiveDate::default(),
            date_to: NaiveDate::default(),
            restaurant_id: None,
            order_by: DbRestaurantOrderingMethod::Price(DbOrder::Desc),
            limit: Some(1),
            offset: Some(0),
        };

        let menus_with_restaurant = menu_repo.read_many(&menu_read_many).await?;

        assert_eq!(menus_with_restaurant[0].name, "Pizzeria Vito");

        Ok(())
    }

    // Integration test for user repo and group repo
    #[sqlx::test()]
    async fn user_group_integration_test(pool: PgPool) -> DbResultSingle<()> {
        let arc_pool = Arc::new(pool);

        let user_repository = UserRepository::new(PoolHandler::new(arc_pool.clone()));

        let new_user = UserCreate {
            username: "Jacky123".to_string(),
            email: "jacky123@gmail.com".to_string(),
            profile_picture: None,
            password_hash: "123456789".to_string(),
        };

        let new_user2 = UserCreate {
            username: "SpeedDemon".to_string(),
            email: "speederino@gmail.com".to_string(),
            profile_picture: None,
            password_hash: "123456789".to_string(),
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
        };

        // Edit user
        let user = user_repository.update(&edit_user).await?;

        assert_eq!(user.len(), 1);

        let user = user[0].to_owned();

        assert_eq!(user.username, edit_user.username.unwrap());

        // Get users
        let users = user_repository
            .read_many(&UserGetByUsername::new("speed"))
            .await?;

        assert_eq!(users.len(), 1);
        assert_eq!(users[0].username, "SpeedDemon");

        let group_repository = GroupRepository::new(PoolHandler::new(arc_pool.clone()));

        let new_group = GroupCreate {
            name: "Moje Group".to_string(),
            description: Some("...".to_string()),
            author_id: user.id,
            picture: None,
            users: vec![],
        };

        // Group create
        let group = group_repository.create(&new_group).await?;

        assert_eq!(group.name, new_group.name);
        assert_eq!(group.author_id, user.id);

        // Get groups of user
        let groups = group_repository
            .read_many(&GroupGetGroupsByUser::new(&user.id))
            .await?;

        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].name, new_group.name);

        let add_user_to_group = GroupUserCreate {
            user_id: user2.id,
            group_id: group.id,
        };

        // Add user to group
        group_repository
            .add_user_to_group(&add_user_to_group)
            .await?;

        let groups = group_repository
            .read_many(&GroupGetGroupsByUser::new(&user2.id))
            .await?;

        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].name, new_group.name);

        // List users in a group
        let users = group_repository
            .list_group_users(&GroupGetById::new(&group.id))
            .await?;

        assert_eq!(users.len(), 2);
        assert_eq!(users[0].username, user.username);
        assert_eq!(users[1].username, user2.username);

        // Remove user from a group

        group_repository
            .list_group_users(&GroupGetById::new(&group.id))
            .await?;

        group_repository
            .remove_user_from_group(&GroupUserDelete::new(&user2.id, &group.id))
            .await?;

        let users = group_repository
            .list_group_users(&GroupGetById::new(&group.id))
            .await?;

        assert_eq!(users.len(), 1);
        assert_eq!(users[0].username, user.username);

        // Add user again
        group_repository
            .add_user_to_group(&add_user_to_group)
            .await?;

        let users = group_repository
            .list_group_users(&GroupGetById::new(&group.id))
            .await?;

        assert_eq!(users.len(), 2);
        assert_eq!(users[0].username, user.username);
        assert_eq!(users[1].username, user2.username);

        Ok(())
    }

    // Integration test for lunch repo and votes repo
    #[sqlx::test(fixtures("sample_data.sql"))]
    async fn lunch_and_votes_test(pool: PgPool) -> DbResultSingle<()> {
        let arc_pool = Arc::new(pool);

        let vote_repository = VoteRepository::new(PoolHandler::new(arc_pool.clone()));
        let lunch_repository = LunchRepository::new(PoolHandler::new(arc_pool.clone()));
        let user_repository = UserRepository::new(PoolHandler::new(arc_pool.clone()));
        let group_repository = GroupRepository::new(PoolHandler::new(arc_pool.clone()));

        // Get votes for lunch
        let votes = vote_repository
            .read_many(&VoteGetMany {
                lunch_id: Uuid::parse_str("645ae55a-190e-4b5d-b47b-0c00c9f4ce0d").unwrap(),
            })
            .await?;

        assert_eq!(votes.len(), 2);
        assert_eq!(votes[0].votes.len(), 1);
        assert_eq!(votes[1].votes.len(), 1);

        // Get lunches available to user (he is author)
        let lunches = lunch_repository
            .read_many(&LunchGetMany {
                group_id: None,
                user_id: Some(Uuid::parse_str("bfadb3a0-287c-4b5b-9132-cd977217a694").unwrap()),
                from: Some(NaiveDate::parse_from_str("2024-01-15", "%Y-%m-%d").unwrap()),
                to: Some(NaiveDate::parse_from_str("2024-01-15", "%Y-%m-%d").unwrap()),
            })
            .await?;

        assert_eq!(lunches.len(), 1);

        // Get lunches available to user (he is user in group)
        let lunches = lunch_repository
            .read_many(&LunchGetMany {
                group_id: None,
                user_id: Some(Uuid::parse_str("c831db0d-23bf-4a88-8974-332fdea327cd").unwrap()),
                from: Some(NaiveDate::parse_from_str("2024-01-15", "%Y-%m-%d").unwrap()),
                to: Some(NaiveDate::parse_from_str("2024-01-15", "%Y-%m-%d").unwrap()),
            })
            .await?;

        assert_eq!(lunches.len(), 1);

        // Get lunches by group
        let lunches = lunch_repository
            .read_many(&LunchGetMany {
                group_id: Some(Uuid::parse_str("4a51b8d6-c7dc-428b-bee6-97706063a0ae").unwrap()),
                user_id: None,
                from: Some(NaiveDate::parse_from_str("2024-01-15", "%Y-%m-%d").unwrap()),
                to: Some(NaiveDate::parse_from_str("2024-01-15", "%Y-%m-%d").unwrap()),
            })
            .await?;

        assert_eq!(lunches.len(), 1);

        // Add new user to a group
        let new_user = UserCreate {
            username: "Stylo".to_string(),
            email: "stylo@gmail.com".to_string(),
            profile_picture: None,
            password_hash: "123456789".to_string(),
        };

        let user = user_repository.create(&new_user).await?;
        group_repository
            .add_user_to_group(&GroupUserCreate {
                user_id: user.id,
                group_id: Uuid::parse_str("4a51b8d6-c7dc-428b-bee6-97706063a0ae").unwrap(),
            })
            .await?;

        let users = group_repository
            .list_group_users(&GroupGetById::new(
                &Uuid::parse_str("4a51b8d6-c7dc-428b-bee6-97706063a0ae").unwrap(),
            ))
            .await?;

        assert_eq!(users.len(), 3);

        // Add vote
        vote_repository
            .create(&VoteCreate {
                menu_id: Uuid::parse_str("d528ed1d-bb13-4297-a760-f6e7692aa473").unwrap(),
                user_id: user.id,
                lunch_id: Uuid::parse_str("645ae55a-190e-4b5d-b47b-0c00c9f4ce0d").unwrap(),
            })
            .await?;

        let votes = vote_repository
            .read_many(&VoteGetMany {
                lunch_id: Uuid::parse_str("645ae55a-190e-4b5d-b47b-0c00c9f4ce0d").unwrap(),
            })
            .await?;

        assert_eq!(votes.len(), 2);

        let mut vote_count = 0;
        for vote in votes {
            vote_count += vote.votes.len();
        }

        assert_eq!(vote_count, 3);

        Ok(())
    }

    #[sqlx::test(fixtures("sample_data.sql"))]
    async fn test_restaurant_search(pool: PgPool) -> DbResultSingle<()> {
        let arc_pool = Arc::new(pool);

        let restaurant_repo = RestaurantRepository::new(PoolHandler::new(arc_pool.clone()));

        // Get votes for lunch

        let id = restaurant_repo
            .search_restaurant(&RestaurantGetByNameAndAddress {
                name: "Pivnice Masný Růžek".to_string(),
                street: "Křenová".to_string(),
                house_number: "70".to_string(),
                zip_code: "602 00".to_string(),
                city: "Brno".to_string(),
            })
            .await?;

        assert_eq!(
            Uuid::parse_str("7d7ec998-45da-41ee-bb4c-ac5bbe0e4669").unwrap(),
            id.unwrap().id
        );

        Ok(())
    }
}
