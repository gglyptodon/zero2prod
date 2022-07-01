use actix_web::{post, web, Responder};
use chrono::Utc;
use serde_derive::Deserialize;
use sqlx::{PgPool};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Info {
    name: String,
    email: String,
}

#[post("/subscriptions")]
pub async fn subscribe(
    info: web::Form<Info>,
    connection_pool: web::Data<PgPool>,
) -> impl Responder {
    sqlx::query!(
        r#" INSERT INTO subscriptions (id, name, email, subscribed_at) VALUES ($1, $2, $3, $4);
      "#,
        Uuid::new_v4(),
        info.name,
        info.email,
        Utc::now()
    ).execute(connection_pool.get_ref()).await.unwrap();
    format!("Hello {}, with email: {} ", info.name, info.email)
}
