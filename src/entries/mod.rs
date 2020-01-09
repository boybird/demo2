pub mod auth;
pub mod home;
pub mod user;

use serde::Serialize;

#[derive(Serialize)]
pub struct Paged<T>
where
    T: serde::Serialize,
{
    list: Vec<T>,
    count: i64,
}
