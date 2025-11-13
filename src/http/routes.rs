use axum::{Json, extract::State};

use crate::{http::server::AppState, repo::WeatherData};

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Welcome message", body = String)
    )
)]
pub async fn root() -> &'static str {
    "Welcome to the echo's Weather Proxy (https://github.com/echo-webkom/weather-proxy)! Visit /weather for current weather data."
}

#[utoipa::path(
    get,
    path = "/weather",
    responses(
        (status = 200, description = "Current weather data", body = WeatherData)
    )
)]
pub async fn weather(State(state): State<AppState>) -> Json<WeatherData> {
    let weather = state.weather_service.get_weather().await;
    Json(weather)
}
