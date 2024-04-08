use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use core::config::get_config;
use migration::{
    sea_orm::{ConnectOptions, Database},
    Migrator, MigratorTrait,
};
use secrecy::ExposeSecret;
mod core;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
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

    HttpServer::new(move || {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .app_data(db.clone())
    })
    .bind(("127.0.0.1", config.app_port))?
    .run()
    .await
}
