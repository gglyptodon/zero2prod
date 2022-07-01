use actix_web::{post, web, Responder};
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Info {
    name: String,
    email: String,
}

#[post("/subscriptions")]
pub async fn subscribe(info: web::Form<Info>) -> impl Responder {
    format!("Hello {}, with email: {} ", info.name, info.email)
}
