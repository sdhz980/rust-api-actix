use actix_web::{ dev::ServiceResponse, middleware::ErrorHandlerResponse, HttpResponse, Result };
use log::warn;
use serde_json::json;

pub fn handle_default_errors<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let status = res.status();

    match status.as_u16() {
        400..=499 => warn!("Client error {} : {}", status, res.request().uri()),
        500..=599 => warn!("Server error {} : {}", status, res.request().uri()),
        _ => {}
    }

    let (req, _res) = res.into_parts();
    let response = match status.as_u16() {
        404 =>
            HttpResponse::NotFound().json(
                json!({
            "error": {
                "code": "NOT_FOUND",
                "message": "The requested resource was not found",
                "status": 404
            }
        })
            ),
        405 =>
            HttpResponse::MethodNotAllowed().json(
                json!({
            "error": {
                "code": "METHOD_NOT_ALLOWED", 
                "message": "Method not allowed for this endpoint",
                "status": 405
            }
        })
            ),
        500 =>
            HttpResponse::InternalServerError().json(
                json!({
            "error": {
                "code": "INTERNAL_SERVER_ERROR",
                "message": "An internal server error occurred",
                "status": 500
            }
        })
            ),
        _ =>
            HttpResponse::build(status).json(
                json!({
            "error": {
                "code": "UNKNOWN_ERROR",
                "message": "An error occurred",
                "status": status.as_u16()
            }
        })
            ),
    };

    let res = ServiceResponse::new(req, response).map_into_boxed_body().map_into_right_body();
    Ok(ErrorHandlerResponse::Response(res))
}
