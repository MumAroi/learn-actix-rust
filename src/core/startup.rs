use actix_web::dev::Server;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use sea_orm::DbConn;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

/// Runs the HTTP server.
pub fn run(listener: TcpListener, db: DbConn) -> Result<Server, std::io::Error> {
    let db = web::Data::new(db);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            // Register your controllers below 👇
            .service(hello)
            // Register application-wide shared data below 👇
            .app_data(db.clone()) // 👈 ❗Important: Register the database connection pool
    })
    .listen(listener)?
    .run();
    Ok(server)
}
