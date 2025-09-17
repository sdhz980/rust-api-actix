use std::future::{ ready, Ready };
use std::pin::Pin;

use actix_web::body::EitherBody;
use actix_web::dev::{ forward_ready, Service, ServiceResponse, Transform };
use actix_web::http::header::{ AUTHORIZATION };
use actix_web::http::Method;
use actix_web::{ HttpMessage, HttpResponse };
use actix_web::{ dev::ServiceRequest, Error };
use serde_json::json;

use crate::utils::jwt::Claims;
use crate::{ utils::jwt::verify_jwt, constants };
pub struct Auth;

impl<S, B> Transform<S, ServiceRequest>
    for Auth
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: actix_web::body::MessageBody + 'static
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(
            Ok(AuthMiddleware {
                service,
            })
        )
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

type LocalBoxFuture<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

impl<S, B> Service<ServiceRequest>
    for AuthMiddleware<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: actix_web::body::MessageBody + 'static
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let mut authenticate_pass: bool = false;

        if Method::OPTIONS == *req.method() {
            authenticate_pass = true;
        } else {
            for ignore_route in constants::INGORE_ROUTES.iter() {
                if req.path().starts_with(ignore_route) {
                    authenticate_pass = true;
                    break;
                }
            }
        }

        if !authenticate_pass {
            let authorization_header = req.headers().get(AUTHORIZATION);

            match authorization_header {
                Some(val) => {
                    let token = match val.to_str() {
                        Ok(val) => val,
                        Err(_) => "",
                    };

                    match verify_jwt(token) {
                        Ok(claims) => {
                            if !claims.sub.is_empty() {
                                req.extensions_mut().insert::<Claims>(claims);
                                authenticate_pass = true;
                            } else {
                                authenticate_pass = false;
                            }
                        }

                        Err(_) => {
                            authenticate_pass = false;
                        }
                    }
                }

                None => {
                    authenticate_pass = false;
                }
            }
        }

        if !authenticate_pass {
            // early return: buat ServiceResponse dan ubah body jadi Right(EitherBody)
            let (request, _pl) = req.into_parts();
            let response = HttpResponse::BadRequest().json(
                json!({
                    "error": {
                        "code": "BAD_REQUEST",
                        "message": "Request cannot be procceded.",
                        "status": 400
                    }
                })
            );
            let srv_resp = ServiceResponse::new(request, response).map_into_right_body();
            return Box::pin(async move { Ok(srv_resp) });
        }

        // normal flow: panggil service, lalu map body ke Left(EitherBody)
        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res.map_into_left_body())
        })
    }
}
