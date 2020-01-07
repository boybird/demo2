include!(concat!(env!("OUT_DIR"), "/generated.rs"));

fn esteblish_connection() {}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    esteblish_connection();

    HttpServer::new(move || {
        let generated = generate();
        App::new()
            .service(entries::home::index)
            .service(actix_web_static_files::ResourceFiles::new("", generated))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

lazy_static! {
    static ref _ENV: () = {
        let _ = dotenv::dotenv();
    };
    static ref DATABASE_URL: String = std::env::var("DATABASE_URL").expect("请设置数据库链接");
    static ref HOST_PORT: String =
        std::env::var("HOST_PORT").unwrap_or("127.0.0.1:8080".to_owned());
}

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
use actix_web::{App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2;

use actix_web_static_files;
use std::collections::HashMap;

mod entries;
mod error;
mod middleware;
mod schema;
