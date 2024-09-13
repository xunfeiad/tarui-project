use actix_web::{
    http,
    middleware::{self, Logger},
    App, HttpServer,
    web
};

use anyhow::Context;
use api::{route::config, websocket::server::ChatServer};
// use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use actix_cors::Cors;
use deadpool_redis::{Config, Runtime};
use schema::state::AppState;
use sea_orm::{Database, DatabaseConnection};
use std::{sync::LazyLock, time::Duration};
use tera::Tera;
use tokio:: sync::Mutex;

static REDIS_URL: LazyLock<String> = LazyLock::new(|| {
    dotenv::dotenv().ok();
    std::env::var("redis_url")
        .context("Get env `redis_url` failed.")
        .unwrap()
});

static POSTGRESQL_URL: LazyLock<String> = LazyLock::new(|| {
    std::env::var("postgresql_url")
    .context("Get env `postgresql_url` failed.")
    .unwrap()
});

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().ok();
    env_logger::init();
    
    let cfg = Config::from_url(&*REDIS_URL);
    let pool = cfg.create_pool(Some(Runtime::Tokio1)).unwrap();
    let conn =pool.get().await.unwrap();
    let db: DatabaseConnection =
        Database::connect::<&str>(POSTGRESQL_URL.as_ref()).await.unwrap();
    let tera = Tera::new("templates/**/*").context("can not find template.").unwrap();

    let state = web::Data::new(AppState {
        app_name: "Actx-Web".to_string(),
        db,
        redis: Mutex::new(conn),
        tera
    });
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5174")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .app_data(state.clone())
            .wrap(middleware::DefaultHeaders::new().add(("APP-NAME", "Actx-Web")))
            .wrap(Logger::default())
            .wrap(cors)
            .configure(config)
    })
    .keep_alive(Duration::from_secs(100))
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
