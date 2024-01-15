use std::{
    env,
    future::{ready, Ready},
    task::{Context, Poll}, sync::{Arc, Mutex},
};

use crate::methods::auth::BEARER;
use crate::models::TokenClaims;
use actix_web::{body::EitherBody, web::Data};
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
};
use futures::{future::LocalBoxFuture, executor::block_on};
use futures::{FutureExt, TryFutureExt};
use jsonwebtoken::{DecodingKey, Validation};
use redis::{aio::Connection, AsyncCommands};

#[doc(hidden)]
pub struct VerifyAuthService<S> {
    service: S,
    required: bool,
}

impl<S, B> Service<ServiceRequest> for VerifyAuthService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let redis = req.app_data::<Data<Arc<Mutex<Connection>>>>().unwrap().clone();
        let token = req.headers().get("Authorization");

        let tkn = match token {
            Some(token) => {
                let token = token.to_str().unwrap().trim_start_matches(BEARER);
                let payload = jsonwebtoken::decode::<TokenClaims>(
                    token,
                    &DecodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_bytes()),
                    &Validation::default(),
                );
                if let Ok(payload) = payload {
                    Some(payload.claims)
                } else {
                    None
                }
            }
            None => None,
        };
    
        if let Some(ref claims) = tkn {
            let mut redis = redis.lock().unwrap();
            let result: Result<bool, redis::RedisError> = block_on(redis.exists(&claims.tkid));
            if let Ok(result) = result {
                if result {
                    return Box::pin(async move {
                        Ok(req.into_response(HttpResponse::Unauthorized().finish().map_into_right_body()))
                    });
                }
            }
        }

        if self.required && tkn.is_none() {
            return Box::pin(async {
                Ok(req.into_response(HttpResponse::Unauthorized().finish().map_into_right_body()))
            });
        }

        if let Some(tkn) = tkn {
            req.extensions_mut().insert(tkn);
        }
        
        return self
            .service
            .call(req)
            .map_ok(ServiceResponse::map_into_left_body)
            .boxed_local();
    }
}

#[derive(Clone, Debug)]
pub struct VerifyAuth {
    required: bool,
}

impl VerifyAuth {
    pub fn required() -> Self {
        Self { required: true }
    }

    pub fn optional() -> Self {
        Self { required: false }
    }
}

impl<S, B> Transform<S, ServiceRequest> for VerifyAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    type Transform = VerifyAuthService<S>;
    type InitError = ();

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(VerifyAuthService {
            service,
            required: self.required,
        }))
    }
}
