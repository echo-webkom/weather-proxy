use std::sync::Arc;

use axum::{Json, Router, extract::State, routing::get};

use crate::{
    repo::{cache::InMemoryWeatherCache, yr::YrWeatherRepo},
    service::WeatherService,
};

mod repo;
mod service;

#[derive(Clone)]
struct AppState {
    weather_service: Arc<WeatherService>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let cache = InMemoryWeatherCache::new();
    let weather_repo = YrWeatherRepo::new();
    let weather_service = WeatherService::new(Arc::new(weather_repo), Arc::new(cache));

    let state = AppState {
        weather_service: Arc::new(weather_service),
    };

    let app = Router::new()
        .route("/", get(root))
        .route("/weather", get(weather))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Welcome to the echo's Weather Proxy (https://github.com/echo-webkom/weather-proxy)! Visit /weather for current weather data."
}

async fn weather(State(state): State<AppState>) -> Json<repo::WeatherData> {
    let weather = state.weather_service.get_weather().await;
    Json(weather)
}
