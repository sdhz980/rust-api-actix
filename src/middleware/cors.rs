use actix_cors::Cors;
use actix_web::http;

pub fn cors() -> Cors {
    Cors::default()
        .allowed_origin("http://localhost:3000")
        .allowed_methods(vec!["POST", "GET", "DELETE", "PATCH"])
        .allowed_headers(
            vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::CONTENT_LENGTH,
                http::header::CONTENT_TYPE
            ]
        )
        .max_age(3600)
}
