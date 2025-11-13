use std::sync::Arc;

use crate::{
    http::server::Server,
    repo::{cache::InMemoryWeatherCache, yr::YrWeatherRepo},
    service::weather::WeatherService,
    tracing::Tracing,
};

mod http;
mod repo;
mod service;
mod tracing;

#[tokio::main]
async fn main() {
    Tracing::init();

    let cache = InMemoryWeatherCache::new();
    let weather_repo = YrWeatherRepo::new();

    let weather_service = WeatherService::new(Arc::new(weather_repo), Arc::new(cache));

    // Start and run the HTTP server
    let server = Server::new(Arc::new(weather_service));
    server.run().await;
}
