#[derive(Deserialize)]
pub struct UserList {
    page: i64,
    num: i64,
    name: Option<String>,
}

#[post("/api/users")]
async fn index(db: Data<MysqlPool>, req: Json<UserList>, identity: JwtIdentity) -> impl Responder {
    println!("identity: {}", identity.id);
    use crate::schema::users::dsl::*;
    let conn = db.get().unwrap();
    let offset = (req.page - 1) * req.num;

    let limited = users.limit(req.num).offset(offset);
    let list;
    let q_name;
    let count;
    if req.name.is_some() && {
        q_name = req.name.as_ref().unwrap();
        q_name.len() > 0
    } {
        let q_name = req.name.as_ref().unwrap();
        list = limited
            .filter(name.like(format!("%{}%", q_name)))
            .load::<User>(&conn)
            .expect("error fetch data");
        count = users
            .filter(name.like(format!("%{}%", q_name)))
            .count()
            .get_result(&conn)
            .unwrap();
    } else {
        list = limited.load::<User>(&conn).expect("error fetch data");
        count = users.count().get_result(&conn).unwrap();
    }
    Json(crate::entries::Paged { list, count })
}

use actix_web::{
    post,
    web::{Data, Json},
    Responder,
};
use diesel::prelude::*;

use crate::models::user::*;
use crate::MysqlPool;

use crate::middleware::JwtIdentity;
use serde::Deserialize;
