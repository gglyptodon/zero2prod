use actix_web::{post, web, HttpResponse, Responder};
use chrono::Utc;
use serde_derive::Deserialize;
use sqlx::PgPool;
use tracing_futures::Instrument;
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
request_id = %Uuid::new_v4(),
subscriber_email = %info.email,
subscriber_name = %info.name
)
)]
pub async fn subscribe(
    info: web::Form<Info>,
    connection_pool: web::Data<PgPool>,
) -> impl Responder {
    let query_span = tracing::info_span!("Saving subscriber");
    match sqlx::query!(
        r#" INSERT INTO subscriptions (id, name, email, subscribed_at) VALUES ($1, $2, $3, $4);
      "#,
        Uuid::new_v4(),
        info.name,
        info.email,
        Utc::now()
    )
    .execute(connection_pool.get_ref())
    .instrument(query_span)
    .await
    {
        Err(e) => {
            tracing::error!("Error saving new subscriber record: {:?}", e);
            HttpResponse::InternalServerError()
        }
        Ok(_) => HttpResponse::Ok(),
    }
}
