use crate::schema::users;
#[derive(Insertable)]
#[table_name="users"]
pub struct Reg {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct Login {
    username: String,
    email: String,
    // #[serde(skip_serializing)]
    password: String,
}



#[post("/api/auth/register")]
async fn register(db: Data<MysqlPool>, req: Json<Login>) -> impl Responder {
    // let {username, email, passord} = req;

    

    ""
}


use crate::MysqlPool;
use actix_web::{
    post,
    web::{Data, Json},
    Responder,
};
use serde::{Deserialize, Serialize};
