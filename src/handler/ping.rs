use actix_web::{get, HttpResponse, Responder};

use crate::domain;
use domain::JsonResponse;

#[get("/ping")]
pub async fn ping() -> impl Responder {
    HttpResponse::Ok().json(JsonResponse {
        status: "success".to_string(),
        message: "pong".to_string()
    })
}