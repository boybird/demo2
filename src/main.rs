// include!(concat!(env!("OUT_DIR"), "/generated.rs"));
// pub type MysqlPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub type MysqlPool = diesel::r2d2::Pool<ConnectionManager<MysqlConnection>>;

fn esteblish_connection() -> MysqlPool {
    let db_url: String = std::env::var("DATABASE_URL").expect("请设置数据库链接");
    let db_manager = ConnectionManager::<MysqlConnection>::new(db_url);

    diesel::r2d2::Pool::new(db_manager).expect("Failed to create pool")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let _ = dotenv::dotenv();
    let pool = esteblish_connection();

    HttpServer::new(move || {
        // let generated = generate();

        App::new()
            // data
            .data(pool.clone())
            // routes
            .service(entries::home::index)
        // .service(actix_web_static_files::ResourceFiles::new("", generated))
    })
    .bind(std::env::var("HOST_PORT").unwrap_or("127.0.0.1:8080".to_owned()))?
    .run()
    .await
}

// lazy_static! {
    // static ref _ENV: () = {
        // let _ = dotenv::dotenv();
    // };
    // static ref DATABASE_URL: String = std::env::var("DATABASE_URL").expect("请设置数据库链接");
    // static ref HOST_PORT: String =        std::env::var("HOST_PORT").unwrap_or("127.0.0.1:8080".to_owned());
    // static ref DB_POOL: r2d2::Pool<diesel::r2d2::ConnectionManager<MysqlConnection>> =

// }

#[macro_use]
extern crate diesel;
// #[macro_use]
// extern crate lazy_static;
use actix_web::{App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;

// use actix_web_static_files;
// use std::collections::HashMap;

mod entries;
mod error;
mod middleware;
mod schema;
mod models;
