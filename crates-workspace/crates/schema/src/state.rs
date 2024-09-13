use tokio::sync::Mutex;
use deadpool_redis::Connection;
use sea_orm::DatabaseConnection;
use tera::Tera;

pub struct AppState {
    pub db: DatabaseConnection,
    pub redis: Mutex<Connection>,
    pub app_name: String,
    pub tera: Tera
}
