use actix_web::web;

use crate::middleware::auth_middleware;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/users").wrap(auth_middleware::Auth));
}
