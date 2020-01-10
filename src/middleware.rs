#[derive(Debug)]
pub struct JwtIdentity{pub id: i64}

impl FromRequest for JwtIdentity {
    type Config = ();
    type Error = actix_http::error::Error;
    type Future = Ready<Result<Self, Error>>;

    #[inline]
    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {

        let auth = req.headers().get("Authentication");
        let id = match auth {
            Some(auth_head) => {
                let auth = auth_head.to_str().unwrap();
                decode(
                    auth,
                    &JWT_SECRET,
                    Algorithm::HS256,
                    // TODO remove dangerous validate
                    &ValidationOptions::dangerous(),
                )
                .map(|(_, num)| num.as_i64().unwrap_or(0))
                .unwrap_or(0)
            }
            None => 0,
        };
 
        ok(JwtIdentity{id})
    }
}

use crate::JWT_SECRET;

use actix_web::{ Error, FromRequest, HttpRequest};
use frank_jwt::{decode, Algorithm, ValidationOptions};
use futures::future::{ok, Ready};


use actix_http::Payload;
