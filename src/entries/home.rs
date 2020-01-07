use actix_web::{get, web, Responder};

#[get("/api/index")]
async fn index() -> impl Responder {

    "hello"
}
