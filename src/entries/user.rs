use actix_web::{
    get,
    web::{Data, Json},
    Responder,
};
use diesel::prelude::*;

use crate::models::user::*;
use crate::MysqlPool;

#[get("/api/users")]
async fn index(db: Data<MysqlPool>) -> impl Responder {
    use crate::schema::users::dsl::*;
    let conn = db.get().unwrap();

    Json(
        users
            .limit(10)
            .load::<User>(&conn)
            .expect("error fetch data"),
    )
}
