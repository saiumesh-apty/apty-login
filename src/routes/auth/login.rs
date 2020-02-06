use actix_web::{ HttpResponse, Responder, web::Json };
use crate::types::user::{UserLogin};

pub async fn login_handler(body: Json<UserLogin>) -> impl Responder {
}