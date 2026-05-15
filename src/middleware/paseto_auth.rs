use actix_web::{
    Error, HttpMessage, dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready}, error::ErrorUnauthorized
};
use futures_util::future::{LocalBoxFuture, Ready, ready};
use paseto_json::{RegisteredClaims, Time};
use paseto_v4::EncryptedToken;
use std::rc::Rc;
use uuid::Uuid;

use crate::helpers::auth::paseto::local_key;

#[derive(Clone)]
pub struct AuthUser(pub Uuid);

pub struct PasetoAuth;

impl<S, B> Transform<S, ServiceRequest> for PasetoAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = PasetoAuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(PasetoAuthMiddleware { service: Rc::new(service) }))
    }
}

pub struct PasetoAuthMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for PasetoAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();

        Box::pin(async move {
            let token_str = req
                .cookie("session")
                .map(|c| c.value().to_owned())
                .ok_or_else(|| ErrorUnauthorized("Missing session cookie"))?;

            let key = local_key()?;

            let encrypted: EncryptedToken<RegisteredClaims> = token_str
                .parse()
                .map_err(|_| ErrorUnauthorized("Malformed token"))?;

            let decrypted = encrypted
                .decrypt(&key, &Time::valid_now())
                .map_err(|_| ErrorUnauthorized("Invalid or expired token"))?;

            let subject = decrypted
                .claims
                .sub
                .as_deref()
                .ok_or_else(|| ErrorUnauthorized("Token missing subject"))?;

            let user_id = Uuid::parse_str(subject)
                .map_err(|_| ErrorUnauthorized("Invalid subject UUID"))?;

            req.extensions_mut().insert(AuthUser(user_id));

            svc.call(req).await
        })
    }
}
