use actix_web::dev::Server;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;
use serde_derive::Deserialize;
//#[get("/hello/{name}")]
//pub async fn greet(name: web::Path<String>) -> impl Responder {
//    format!("Hello {name}!")
//}

#[derive(Deserialize)]
struct Info{
    name: String,
    email: String
}

#[get("/health_check")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
#[post("/subscriptions")]
pub async fn subscribe(info: web::Form<Info>) -> impl Responder {
    format!("Hello {}, with email: {} ",info.name, info.email )
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(health_check).service(subscribe))
        .listen(listener)?
        .run();
    Ok(server)
}

#[cfg(test)]
mod tests {
    use super::*;
}
