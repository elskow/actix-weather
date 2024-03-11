use actix_web::{get, HttpResponse, Responder, web::Path};

use crate::domain;
use domain::JsonResponse;

#[get("/hello/{name}")]
pub async fn hello(info: Path<(String,)>) -> impl Responder {
    HttpResponse::Ok().json(JsonResponse {
        status: "success".to_string(),
        message: format!("Hello, {}!", info.0)
    })
}