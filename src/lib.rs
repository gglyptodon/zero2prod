use actix_web::dev::Server;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}
#[get("/health_check")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(health_check))
        .listen(listener)?
        .run();
    Ok(server)
}

#[cfg(test)]
mod tests {
    use super::*;
}
