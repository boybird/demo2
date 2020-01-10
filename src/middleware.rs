// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct JwtAuth;

impl JwtAuth {
    pub fn new() -> Self {
        JwtAuth
    }
}

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S> for JwtAuth
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtAuthMiddleware { service })
    }
}

pub struct JwtAuthMiddleware<S> {
    service: S,
}

impl<S, B> Service for JwtAuthMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let user_id;
        if req.path().starts_with("/api/")
            && !req.path().starts_with("/api/auth")
            && !req.path().starts_with("/api/public")
        {
            let auth = req.headers().get("Authentication");

            user_id = if auth.is_none() {
                0
            } else {
                let secret = JWT_SECRET.clone();
                let auth = auth.unwrap().to_str().unwrap();
                decode(
                    auth,
                    &secret,
                    Algorithm::HS256,
                    // TODO remove dangerous validate
                    &ValidationOptions::dangerous(),
                )
                .map(|(_, num)| num.as_i64().unwrap_or(0))
                .unwrap_or(0)
            };
        } else {
            user_id = 0;
        }
        // req.set_data_container(extensions: Rc<Extensions>)
        // req.set_payload(user_id);
        // req.app_data().

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}

#[derive(Debug)]
pub struct JwtIdentity{pub id: i64}

impl FromRequest for JwtIdentity {
    type Config = ();
    type Error = actix_http::error::Error;
    type Future = Ready<Result<Self, Error>>;

    #[inline]
    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let secret = JWT_SECRET.clone();
        let auth = req.headers().get("Authentication");
        let id = match auth {
            Some(auth_head) => {
                let auth = auth_head.to_str().unwrap();
                decode(
                    auth,
                    &secret,
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

use std::pin::Pin;
use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, FromRequest, HttpRequest};
use frank_jwt::{decode, Algorithm, ValidationOptions};
use futures::future::{ok, Ready};
use futures::Future;

use actix_http::Payload;
