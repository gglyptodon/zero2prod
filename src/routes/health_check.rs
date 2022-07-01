use actix_web::{get, HttpResponse, Responder};

#[get("/health_check")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

//
//#[cfg(test)]
//mod tests {
//    use super::*;
//}
