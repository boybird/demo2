#[derive(Deserialize)]
pub struct UserList {
    page: i64,
    num: i64,
    name: Option<String>,
}

#[post("/api/users")]
async fn index(db: Data<MysqlPool>, req: Json<UserList>) -> impl Responder {
    use crate::schema::users::dsl::*;
    let conn = db.get().unwrap();
    let offset = (req.page - 1) * req.num;

    let limited = users.limit(req.num).offset(req.num);
    let data;
    let q_name;
    if req.name.is_some() && {
        q_name = req.name.as_ref().unwrap();
        q_name.len() > 0
    } {
        let q_name = req.name.as_ref().unwrap();
        data = limited
            .filter(name.like(format!("%{}%", q_name)))
            .offset(offset)
            .load::<User>(&conn)
            .expect("error fetch data");
    } else {
        data = limited
            .offset(offset)
            .load::<User>(&conn)
            .expect("error fetch data");
    }
    Json(data)
}

use actix_web::{
    get, post,
    web::{Data, Json},
    Responder,
};
use diesel::prelude::*;

use crate::models::user::*;
use crate::MysqlPool;

use serde::Deserialize;
