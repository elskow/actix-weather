use actix_web::{get, HttpResponse, Responder};

use crate::domain;
use domain::JsonResponse;

use std::io;
use std::net::TcpStream;


fn check_internet_connectivity() -> Result<(), io::Error> {
    let result = TcpStream::connect(("www.google.com", 80));
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

fn health_check_handler() -> Result<(), ()> {
    match check_internet_connectivity() {
        Ok(_) => Ok(()),
        Err(_) => Err(())
    }
}

#[get("/health")]
pub async fn health_check() -> impl Responder {
    match health_check_handler() {
        Ok(_) => HttpResponse::Ok().json(JsonResponse {
            status: "success".to_string(),
            message: "Server is healthy".to_string()
        }),
        Err(_) => HttpResponse::InternalServerError().json(JsonResponse {
            status: "error".to_string(),
            message: "Server is unhealthy".to_string()
        })
    }
}