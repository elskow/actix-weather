use actix_web::{get, HttpResponse, Responder};

use crate::{domain, util};
use domain::{WeatherRequest, WeatherResponse, JsonResponse};
use util::i32_random_number_generator;

use crate::load_const::load_cities;

#[get("/weather")]
pub async fn get_weather(weather_request: actix_web::web::Query<WeatherRequest>) -> impl Responder {
    let city_list = load_cities().await.unwrap();
    match &weather_request.city {
        Some(city) => {
            let lower_city = city.to_lowercase();
            if city_list.cities.iter().any(|c| c.to_lowercase() == lower_city) {
                HttpResponse::Ok().json(get_weather_response(city))
            } else {
                HttpResponse::BadRequest().json(JsonResponse {
                    status: "error".to_string(),
                    message: "Invalid or missing city".to_string()
                })
            }
        },
        None => {
            HttpResponse::BadRequest().json(JsonResponse {
                status: "error".to_string(),
                message: "Missing city".to_string()
            })
        }
    }
}

fn get_weather_response(city: &str) -> WeatherResponse {
    let temperature = i32_random_number_generator(20, 40) as f64;
    let humidity = i32_random_number_generator(60, 90) as f64;
    let wind_speed = i32_random_number_generator(5, 10) as f64;
    let wind_direction = match i32_random_number_generator(0, 3) {
        0 => "North",
        1 => "East",
        2 => "South",
        _ => "West"
    };

    WeatherResponse {
        city: city.to_string(),
        temperature,
        humidity,
        wind_speed,
        wind_direction: wind_direction.to_string()
    }
}