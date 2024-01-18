mod app;

use crate::app::handlers::auth::auth_config;
use crate::app::handlers::index::index_config;
use crate::app::handlers::registration::registration_config;
use actix_identity::IdentityMiddleware;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::Key;
use actix_web::web::{Data, ServiceConfig};
use actix_web::{web, App, HttpServer};
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
use std::sync::Arc;
use tokio::sync::Mutex;

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

    HttpServer::new(move || {
        App::new()
            // Identity middleware
            .wrap(IdentityMiddleware::default())
            // todo session key
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                Key::from(&[0; 64]),
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
            .configure(auth_config),
    );
}

fn parse_host() -> String {
    let hostname = env::var("HOSTNAME").unwrap_or(DEFAULT_HOSTNAME.to_string());
    let port = env::var("PORT").unwrap_or(DEFAULT_PORT.to_string());
    format!("{hostname}:{port}")
}
