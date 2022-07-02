use crate::routes::health_check::health_check;
use crate::routes::subscriptions::subscribe;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, pgpool: PgPool) -> Result<Server, std::io::Error> {
    let pgpool = web::Data::new(pgpool);
    let server = HttpServer::new(move || {
        App::new()
            .app_data(pgpool.clone())
            .service(health_check)
            .service(subscribe)
    })
    .listen(listener)?
    .run();
    Ok(server)
}
