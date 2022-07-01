use std::net::TcpListener;
use zero2prod::configuration::get_config;
use zero2prod::startup::run;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = get_config().unwrap();
    //println!("settings {:?}", settings);
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    let listener = TcpListener::bind("localhost:8000").expect("failing to bind to port 8000");
    run(listener)?.await
}
