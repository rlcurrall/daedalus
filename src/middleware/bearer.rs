use std::fs::read_to_string;
use std::future::{ready, Future, Ready};
use std::path::PathBuf;
use std::pin::Pin;

use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, FromRequest, HttpMessage};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::result::AppError;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserClaims {
    pub sub: i64,
    pub exp: usize,
    pub scopes: Vec<String>,
}

impl UserClaims {
    pub fn new(sub: i64, exp: usize, scopes: Vec<String>) -> Self {
        UserClaims { sub, exp, scopes }
    }
}

impl FromRequest for UserClaims {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let ext = req.extensions();
        let user_claims = match ext.get::<UserClaims>() {
            None => return ready(Err(AppError::Unauthorized.into())),
            Some(user_claims) => user_claims,
        };

        ready(Ok(user_claims.clone()))
    }
}

pub struct JwtAuth {
    pub_key: String,
}

impl JwtAuth {
    pub fn new(pub_key_path: PathBuf) -> Self {
        let pub_key = read_to_string(pub_key_path).expect("could not open file for public key");

        JwtAuth { pub_key }
    }
}

impl<S, B> Transform<S, ServiceRequest> for JwtAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtAuthMiddleware {
            service,
            pub_key: self.pub_key.clone(),
        }))
    }
}

pub struct JwtAuthMiddleware<S> {
    service: S,
    pub_key: String,
}

impl<S, B> Service<ServiceRequest> for JwtAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_header = match req.headers().get("Authorization") {
            Some(auth_header) => auth_header,
            None => {
                tracing::debug!("No auth header");
                let fut = self.service.call(req);
                return Box::pin(async move { Ok(fut.await?) });
            }
        };

        let bearer = match auth_header.to_str() {
            Ok(bearer) => bearer,
            Err(err) => {
                tracing::error!("Err getting bearer token: {err}");
                let fut = self.service.call(req);
                return Box::pin(async move { Ok(fut.await?) });
            }
        };

        let key = match DecodingKey::from_rsa_pem(self.pub_key.as_bytes()) {
            Ok(key) => key,
            Err(err) => {
                tracing::error!("Error getting decoding key: {err}");
                let fut = self.service.call(req);
                return Box::pin(async move { Ok(fut.await?) });
            }
        };

        let token = bearer.split("Bearer").collect::<Vec<&str>>()[1].trim();
        let token_data = match decode::<UserClaims>(token, &key, &Validation::new(Algorithm::RS256))
        {
            Ok(token_data) => token_data,
            Err(err) => {
                tracing::error!("Error decoding token: {err}");
                let fut = self.service.call(req);
                return Box::pin(async move { Ok(fut.await?) });
            }
        };

        tracing::info!("Token data: {:?}", token_data.claims);

        req.extensions_mut().insert(token_data.claims);

        let fut = self.service.call(req);
        Box::pin(async move { Ok(fut.await?) })
    }
}
