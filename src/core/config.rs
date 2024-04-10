use secrecy::{ExposeSecret, Secret};
use std::env;
// use serde::Deserialize;

// #[derive(Deserialize)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub app_port: u16,
    pub jwt_secret: Secret<String>,
}

// #[derive(Deserialize)]
pub struct DatabaseConfig {
    db_host: String,
    db_port: String,
    db_username: String,
    db_password: Secret<String>,
    db_database: String,
}

impl DatabaseConfig {
    pub fn connection_string(&self) -> Secret<String> {
        // dbg!(format!(
        //     "postgres://{}:{}@{}:{}/{}",
        //     self.db_username,
        //     self.db_password.expose_secret(),
        //     self.db_host,
        //     self.db_port,
        //     self.db_database
        // ));
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.db_username,
            self.db_password.expose_secret(),
            self.db_host,
            self.db_port,
            self.db_database
        ))
    }
}

pub fn get_config() -> Result<AppConfig, dotenvy::Error> {
    let db_host = env::var("DB_HOST").expect("DATABASE_HOST is not set in .env file");
    let db_port = env::var("DB_PORT").expect("DATABASE_PORT is not set in .env file");
    let db_port = db_port.parse().expect("DATABASE_PORT is not a number");
    let db_name = env::var("DB_NAME").expect("DATABASE_NAME is not set in .env file");
    let db_username = env::var("DB_USERNAME").expect("DATABASE_USERNAME is not set in .env file");
    let db_password = env::var("DB_PASSWORD").expect("DATABASE_PASSWORD is not set in .env file");
    let app_port = env::var("APPLICATION_PORT").expect("APPLICATION_PORT is not set in .env file");
    let app_port = app_port.parse().expect("APPLICATION_PORT is not a number");
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET is not set in .env file");
    Ok(AppConfig {
        database: DatabaseConfig {
            db_username,
            db_password: Secret::new(db_password),
            db_host,
            db_port,
            db_database: db_name,
        },
        app_port,
        jwt_secret: Secret::new(jwt_secret)
    })
}
