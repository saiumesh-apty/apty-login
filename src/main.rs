use actix_web::{App, HttpServer };

// routes
mod routes;
use routes::auth::auth_routes;

// types
mod types;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(auth_routes())
    }).bind("127.0.0.1:8000")?.run().await
}