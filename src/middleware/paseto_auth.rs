use actix_web::{
    Error, HttpMessage, HttpResponse,
    body::EitherBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
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
    type Response = ServiceResponse<EitherBody<B>>;
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

macro_rules! unauthorized {
    ($req:expr, $msg:expr) => {{
        let (req, _) = $req.into_parts();
        let res = HttpResponse::Unauthorized().json(serde_json::json!({ "message": $msg }));
        return Ok(ServiceResponse::new(req, res).map_into_right_body());
    }};
}

impl<S, B> Service<ServiceRequest> for PasetoAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();

        Box::pin(async move {
            let Some(token_str) = req.cookie("session").map(|c| c.value().to_owned()) else {
                unauthorized!(req, "Missing session cookie");
            };

            let key = match local_key() {
                Ok(k) => k,
                Err(_) => unauthorized!(req, "Unauthorized"),
            };

            let Ok(encrypted) = token_str.parse::<EncryptedToken<RegisteredClaims>>() else {
                unauthorized!(req, "Unauthorized");
            };

            let Ok(decrypted) = encrypted.decrypt(&key, &Time::valid_now()) else {
                unauthorized!(req, "Unauthorized");
            };

            let Some(subject) = decrypted.claims.sub.as_deref() else {
                unauthorized!(req, "Unauthorized");
            };

            let Ok(user_id) = Uuid::parse_str(subject) else {
                unauthorized!(req, "Unauthorized");
            };

            req.extensions_mut().insert(AuthUser(user_id));

            svc.call(req).await.map(|res| res.map_into_left_body())
        })
    }
}
