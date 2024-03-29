use actix_web::{App, HttpServer};
use std::env;
use dotenv::dotenv;

mod handler;
mod domain;
mod util;
mod load_const;

use handler::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port = env::var("PORT").expect("PORT must be set");

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(ping)
            .service(health_check)
            .service(get_weather)
    })
        .bind(("0.0.0.0", port.parse().unwrap()))?
        .run()
        .await
}