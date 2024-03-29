mod app;

use crate::app::handlers::auth::auth_config;
use crate::app::handlers::group::group_config;
use crate::app::handlers::index::index_config;
use crate::app::handlers::lunch::lunch_config;
use crate::app::handlers::menu::menu_config;
use crate::app::handlers::registration::registration_config;
use crate::app::handlers::restaurant::restaurant_config;
use crate::app::handlers::user::user_config;
use crate::app::handlers::vote::vote_config;
use actix_identity::IdentityMiddleware;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::Key;
use actix_web::web::{Data, ServiceConfig};
use actix_web::{web, App, HttpServer};
use chrono::{FixedOffset, Local};
use cron::Schedule;
use db::db::common::run_migration::run_migration;
use db::db::common::{DbPoolHandler, DbRepository, PoolHandler};
use db::db::repositories::{
    GroupRepository, LunchRepository, MenuRepository, RestaurantRepository, UserRepository,
    VoteRepository,
};
use env_logger::Env;
use log::{info, warn};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

mod scrapping;

const DEFAULT_HOSTNAME: &str = "localhost";
const DEFAULT_PORT: &str = "8000";

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    if let Err(e) = dotenvy::dotenv() {
        warn!("failed loading .env file: {e}")
    };

    let host = parse_host();
    info!("starting server on {host}");

    if let Err(e) = dotenvy::dotenv() {
        warn!("failed loading .env file: {e}");
    };

    let pool = set_up_database_pool().await;

    run_migration(pool.clone())
        .await
        .expect("could not run migration on database");

    let user_repository = UserRepository::new(PoolHandler::new(pool.clone()));
    let group_repository = GroupRepository::new(PoolHandler::new(pool.clone()));
    let lunch_repository = LunchRepository::new(PoolHandler::new(pool.clone()));
    let menu_repository = MenuRepository::new(PoolHandler::new(pool.clone()));
    let restaurant_repository = RestaurantRepository::new(PoolHandler::new(pool.clone()));
    let vote_repository = VoteRepository::new(PoolHandler::new(pool.clone()));

    let initial_scrap = scrapping::service::scraping_service::scrap(
        RestaurantRepository::new(PoolHandler::new(pool.clone())),
        MenuRepository::new(PoolHandler::new(pool.clone())),
    );

    actix_rt::spawn(async move {
        let _ = initial_scrap.await;
    });

    actix_rt::spawn(async move {
        let expression = "0   8   *     *       *  *  *";
        let schedule = Schedule::from_str(expression).unwrap();
        let offset = FixedOffset::east_opt(1 * 3600).unwrap();

        loop {
            let mut upcoming = schedule.upcoming(offset).take(1);
            actix_rt::time::sleep(Duration::from_secs(3600)).await;
            let local = &Local::now();

            if let Some(datetime) = upcoming.next() {
                if datetime.timestamp() <= local.timestamp() {
                    let _ = scrapping::service::scraping_service::scrap(
                        RestaurantRepository::new(PoolHandler::new(pool.clone())),
                        MenuRepository::new(PoolHandler::new(pool.clone())),
                    )
                    .await;
                }
            }
        }
    });

    HttpServer::new(move || {
        App::new()
            // Identity middleware
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                Key::from(
                    env::var("SESSION_KEY")
                        .expect("Could not load session key.")
                        .as_bytes(),
                ),
            ))
            // Add repositories
            .app_data(Data::new(user_repository.clone()))
            .app_data(Data::new(group_repository.clone()))
            .app_data(Data::new(lunch_repository.clone()))
            .app_data(Data::new(menu_repository.clone()))
            .app_data(Data::new(restaurant_repository.clone()))
            .app_data(Data::new(vote_repository.clone()))
            // Configure endpoints
            .configure(configure_webapp)
    })
    .bind(host)?
    .run()
    .await?;

    Ok(())
}

/// Sets-up sqlx's postgres connection pool
/// DATABASE_URL environment variable needs to be set with proper connection string.
async fn set_up_database_pool() -> Arc<Pool<Postgres>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("could not create database pool");

    Arc::new(pool)
}

pub fn configure_webapp(config: &mut ServiceConfig) {
    config.service(
        web::scope("")
            // Static images ans CSS files
            .service(actix_files::Files::new("/static", "./static").prefer_utf8(true))
            // User uploaded files
            .service(actix_files::Files::new("/uploads", "./uploads").prefer_utf8(true))
            .configure(index_config)
            .configure(registration_config)
            .configure(auth_config)
            .configure(user_config)
            .configure(lunch_config)
            .configure(vote_config)
            .configure(menu_config)
            .configure(restaurant_config)
            .configure(group_config),
    );
}

fn parse_host() -> String {
    let hostname = env::var("HOSTNAME").unwrap_or(DEFAULT_HOSTNAME.to_string());
    let port = env::var("PORT").unwrap_or(DEFAULT_PORT.to_string());
    format!("{hostname}:{port}")
}
