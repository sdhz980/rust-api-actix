use actix_web::web;

use crate::handlers::auth;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web
            ::scope("/auth")
            .service(web::resource("/register").route(web::post().to(auth::register)))
            .service(web::resource("/login").route(web::post().to(auth::login)))
    );
}
