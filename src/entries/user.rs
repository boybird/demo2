use actix_web::{get, web::Data, Responder};
use diesel::prelude::*;


use crate::models::user::*;
use crate::MysqlPool;



#[get("/api/users")]
async fn index(db: Data<MysqlPool>) -> impl Responder {
    use crate::schema::users::dsl::*;
    // let results = users.limit(10).load::<User>(&*db)
    // .expect("error fetch data");
    "hello"
}
