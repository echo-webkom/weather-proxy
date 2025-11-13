use std::sync::Arc;

use crate::{
    config::Config,
    http::server::Server,
    repo::{cache::InMemoryWeatherCache, yr::YrWeatherRepo},
    service::weather::WeatherService,
    tracing::Tracing,
};

mod config;
mod http;
mod repo;
mod service;
mod tracing;

#[tokio::main]
async fn main() {
    let config = Config::from_env();

    Tracing::init(&config);

    let cache = InMemoryWeatherCache::new();
    let weather_repo = YrWeatherRepo::new();

    let weather_service = WeatherService::new(Arc::new(weather_repo), Arc::new(cache));

    // Start and run the HTTP server
    let server = Server::new(Arc::new(weather_service));
    server.run().await;
}
