#![feature(type_name_of_val)]
pub type MysqlPool = diesel::r2d2::Pool<ConnectionManager<MysqlConnection>>;
pub static JWT_SECRET: &'static str = include_str!(".jwt_secret");

fn esteblish_connection() -> MysqlPool {
    let db_url: String = std::env::var("DATABASE_URL").expect("请设置数据库链接");
    let db_manager = ConnectionManager::<MysqlConnection>::new(db_url);

    diesel::r2d2::Pool::new(db_manager).expect("Failed to create pool")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let _ = dotenv::dotenv();
    let pool = esteblish_connection();
    //let ssl_key = std::env::var("SSL_KEY_PATH").unwrap_or("key.pm".to_owned());
    //let ssl_cert = std::env::var("SSL_CERT_PATH").unwrap_or("cert.pem".to_owned());

    //let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    //builder
    //    .set_private_key_file(ssl_key, SslFiletype::PEM)
    //    .unwrap();
    //builder.set_certificate_chain_file(ssl_cert).unwrap();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            // routes
            .service(entries::home::index)
            .service(entries::user::index)
            .service(entries::auth::register)
            .service(entries::auth::login)
    })
    .bind("127.0.0.1:8080")?
    //.bind_openssl("127.0.0.1:8080", builder)?
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
#[macro_use]
extern crate serde_json;
use actix_web::{App, HttpServer};
// use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
// use anyhow::{Context, Result};

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;

mod entries;
mod error;
mod middleware;
mod models;
mod schema;
