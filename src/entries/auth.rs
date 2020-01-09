#[derive(Insertable)]
#[table_name = "users"]
#[derive(Deserialize, Serialize)]
pub struct Register {
    name: String,
    email: String,
    #[serde(skip_serializing)]
    password: String,
}

#[post("/api/auth/register")]
async fn register(db: Data<MysqlPool>, mut req: Json<Register>) -> impl Responder {
    let conn = db.get().unwrap();

    req.password = hash(req.password.clone(), 4).unwrap();

    diesel::insert_into(users::table)
        .values(&req.0)
        .execute(&conn)
        .expect("Error saving new post");
    

    Json(req.0)
}

#[derive(Deserialize)]
pub struct Login {
    email: String,
    password: String,
}

#[post("/api/auth/login")]
async fn login(db: Data<MysqlPool>, req: Json<Login>) -> impl Responder {
    use crate::schema::users::dsl::*;

    let conn = db.get().unwrap();

    let user = users
        .limit(1)
        .filter(email.eq(req.0.email))
        .get_result::<UserModel>(&conn)
        .expect("error find user");

    let secret = dotenv::var("secret").unwrap_or("secret123".to_owned());
    let p1 = json!(user.id);
    let header = json!({});
    let jwt1 = encode(header, &secret, &p1, Algorithm::HS256).unwrap();

    if verify(req.0.password, &user.password).unwrap() {
        println!("登录成功")
    }

    Json(
        json!(
            {
                "access_token":jwt1,
                "token_type": "bearer",
                "expires_in": 3600
            }
        )
    )

}

use crate::models::user::User as UserModel;
use crate::schema::users;
use crate::MysqlPool;
use actix_web::{
    post,
    web::{Data, Json},
    Responder,
};
use bcrypt::{hash, verify};
use diesel::prelude::*;
use frank_jwt::{decode, encode, Algorithm};
use serde::{Deserialize, Serialize};