pub mod login;

use actix_web::{ Scope, web::{post, scope} };


pub fn auth_routes() -> Scope {
    scope("/auth")
        .route("/login", post().to(login::login_handler))
}

