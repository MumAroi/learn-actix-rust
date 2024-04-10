use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use sea_orm::DbConn;
use std::net::TcpListener;
use crate::modules::auth::controller::sign_in;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

/// Runs the HTTP server.
pub fn run(listener: TcpListener, db: DbConn) -> Result<Server, std::io::Error> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let db = web::Data::new(db);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            // %a %t "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
            .wrap(Logger::new("%a %{User-Agent}i"))
            // Register your controllers below 👇
            .service(hello)
            .service(web::scope("/v1")
                .service(
                    web::scope("/auth")
                    .service(sign_in)
                )
            )
            // Register application-wide shared data below 👇
            .app_data(db.clone()) // 👈 ❗Important: Register the database connection pool
    })
    .listen(listener)?
    .run();
    Ok(server)
}
