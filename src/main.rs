use std::net::TcpListener;
use zero2prod::run;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    let listener = TcpListener::bind("localhost:8000").expect("failing to bind to port 8000");
    run(listener)?.await
}
