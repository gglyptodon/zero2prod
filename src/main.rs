use std::io::stdout;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use tracing_log::LogTracer;

use zero2prod::configuration::get_config;
use zero2prod::startup::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    LogTracer::init().expect("Failed to set logger");
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_|EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("z2p".into(),stdout);
    let subscriber = Registry::default().with(env_filter).with(JsonStorageLayer).with(formatting_layer);
    set_global_default(subscriber).expect("Failed to set subscriber");
    let configuration = get_config().expect("Failed to read config");
    let connection_string = configuration.database.connection_string();
    let connection_pool = PgPool::connect(&connection_string)
        .await
        .expect("Could not connect to db");
    let listener = TcpListener::bind("localhost:8000").expect("failing to bind to port 8000");
    run(listener, connection_pool)?.await
}
