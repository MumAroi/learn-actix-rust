use migration::{
    sea_orm::{ConnectOptions, Database, DatabaseConnection},
    DbErr,
};

pub struct AppConfig {
    db_host: String,
    db_port: String,
    db_username: String,
    db_password: String,
    db_database: String,
    jwt_secret: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            db_host: std::env::var("DB_HOST").unwrap_or("127.0.0.1".to_string()),
            db_port: std::env::var("DB_PORT").unwrap_or("3306".to_string()),
            db_username: std::env::var("DB_USERNAME").unwrap_or("root".to_string()),
            db_password: std::env::var("DB_PASSWORD").unwrap_or("332211".to_string()),
            db_database: std::env::var("DB_DATABASE").unwrap_or("bookstore".to_string()),
            jwt_secret: std::env::var("JWT_SECRET")
                .expect("Please set the secret key in the .env file"),
        }
    }
}

pub async fn connect(config: &AppConfig) -> Result<DatabaseConnection, DbErr> {
    dbg!(format!(
        "postgres://{}:{}@{}:{}/{}",
        config.db_username, config.db_password, config.db_host, config.db_port, config.db_database
    ));
    let mut opts = ConnectOptions::new(format!(
        "postgres://{}:{}@{}:{}/{}",
        config.db_username, config.db_password, config.db_host, config.db_port, config.db_database
    ));
    opts.sqlx_logging(false);
    Database::connect(opts).await
}
