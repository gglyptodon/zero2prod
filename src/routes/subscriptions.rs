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
pub async fn subscribe(
    info: web::Form<Info>,
    connection_pool: web::Data<PgPool>,
) -> impl Responder {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!("Adding new subscriber", %request_id, subscriber_email = %info.email, subcriber_name=%info.name);
    let _request_span_guard = request_span.enter();
    tracing::info!(
        "Request {}: Saving new subscriber record: name: '{}', email: '{}'",
        request_id,
        info.name,
        info.email
    );
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
            tracing::error!(
                "Request {}: Error saving new subscriber record: {:?}",
                request_id,
                e
            );
            HttpResponse::InternalServerError()
        }
        Ok(_) => {
            tracing::info!(
                "Request {}: Subscriber record saved successfully",
                request_id
            );
            HttpResponse::Ok()
        }
    }
}
