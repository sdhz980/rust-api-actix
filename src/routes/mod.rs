pub mod auth;
pub mod users;
pub mod todos;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web
            ::scope("/api/v1")
            .configure(auth::config)
            .configure(users::config)
            .configure(todos::config)
    );
}
