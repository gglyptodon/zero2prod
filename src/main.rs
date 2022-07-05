use sqlx::PgPool;
use std::net::TcpListener;

use zero2prod::configuration::get_config;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("z2p".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let configuration = get_config().expect("Failed to read config");
    let connection_string = configuration.database.connection_string();
    let connection_pool = PgPool::connect(&connection_string)
        .await
        .expect("Could not connect to db");
    let listener = TcpListener::bind("localhost:8000").expect("failing to bind to port 8000");
    run(listener, connection_pool)?.await
}
