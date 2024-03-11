use actix_web::{get, HttpResponse, Responder};

use crate::domain;
use domain::JsonResponse;

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().json(JsonResponse {
        status: "success".to_string(),
        message: "Hello, world!".to_string()
    })
}