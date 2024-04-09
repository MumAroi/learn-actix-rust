use lib::core::{config::get_config, startup::run};
use migration::{
    sea_orm::{ConnectOptions, Database},
    Migrator, MigratorTrait,
};
use secrecy::ExposeSecret;
use std::net::TcpListener;

#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");

    dotenvy::dotenv().ok();

    let config = get_config().expect("Failed to read configuration.");

    let mut opts = ConnectOptions::new(&*config.database.connection_string().expose_secret());
    opts.sqlx_logging(false);
    let db = Database::connect(&*config.database.connection_string().expose_secret())
        .await
        .unwrap();

    match Migrator::up(&db, None).await {
        Err(err) => panic!("{}", err),
        Ok(_) => (),
    };

    let address = format!("127.0.0.1:{}", config.app_port);
    let listener = TcpListener::bind(address)?;
    run(listener, db)?.await
}
