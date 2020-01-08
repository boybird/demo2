use actix_web::{get, Responder};

#[get("/api/index")]
async fn index() -> impl Responder {
    "hello"
}
