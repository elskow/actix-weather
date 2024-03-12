use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JsonResponse {
    pub status: String,
    pub message: String
}

#[derive(Serialize, Deserialize)]
pub struct WeatherResponse {
    pub city: String,
    pub temperature: f64,
    pub humidity: f64,
    pub wind_speed: f64,
    pub wind_direction: String
}

#[derive(Serialize, Deserialize)]
pub struct WeatherRequest {
    pub city: Option<String>
}