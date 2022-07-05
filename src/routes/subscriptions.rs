use std::pin::Pin;
use actix_web::{post, web, HttpResponse, Responder};
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
#[tracing::instrument(
name="Adding new subscriber",
skip(info, connection_pool),
fields(
subscriber_email = %info.email,
subscriber_name = %info.name
)
)]
pub async fn subscribe(
    info: web::Form<Info>,
    connection_pool: web::Data<PgPool>,
) -> impl Responder {
    //let query_span = tracing::info_span!("Saving subscriber");
    match insert_subscriber(&connection_pool, &info).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}
#[tracing::instrument(name="Saving new subscriber to db", skip(info, connection_pool))]
pub async fn insert_subscriber(connection_pool: &PgPool, info: &Info) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#" INSERT INTO subscriptions (id, name, email, subscribed_at) VALUES ($1, $2, $3, $4);
      "#,
        Uuid::new_v4(),
        info.name,
        info.email,
        Utc::now()
    )
    .execute(Pin::new(connection_pool).get_ref())
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query:{:?}", e);
        e
    })?;
    Ok(())
}
