use env_logger::Env;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_config;
use zero2prod::startup::run;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    //let _settings = get_config().unwrap();
    //println!("settings {:?}", settings);
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    let configuration = get_config().expect("Failed to read config");
    let connection_string = configuration.database.connection_string();
    let connection_pool = PgPool::connect(&connection_string)
        .await
        .expect("Could not connect to db");
    let listener = TcpListener::bind("localhost:8000").expect("failing to bind to port 8000");
    run(listener, connection_pool)?.await
}
