use std::sync::Arc;

use axum::{Json, extract::State, http::HeaderValue};
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    repo::{WeatherCondition, WeatherData, cache::InMemoryWeatherCache, yr::YrWeatherRepo},
    service::WeatherService,
};

mod repo;
mod service;

#[derive(Clone)]
struct AppState {
    weather_service: Arc<WeatherService>,
}

#[derive(OpenApi)]
#[openapi(
    paths(root, weather),
    components(schemas(WeatherData, WeatherCondition)),
    tags(
        (name = "weather", description = "Weather API endpoints")
    ),
    info(
        title = "echo Weather Proxy API",
        version = "0.1.0",
        description = "A weather proxy service for echo-webkom"
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cache = InMemoryWeatherCache::new();
    let weather_repo = YrWeatherRepo::new();
    let weather_service = WeatherService::new(Arc::new(weather_repo), Arc::new(cache));

    let state = AppState {
        weather_service: Arc::new(weather_service),
    };

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(root))
        .routes(routes!(weather))
        .with_state(state)
        .split_for_parts();

    let router = router
        .merge(SwaggerUi::new("/swagger").url("/api-docs/openapi.json", api.clone()))
        .layer(
            CorsLayer::permissive()
                .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap()),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    tracing::info!("🚀 Running Axum REST API on http://localhost:{port}");
    tracing::info!("📖 Swagger UI available at http://localhost:{port}/swagger");

    axum::serve(listener, router).await.unwrap();
}

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Welcome message", body = String)
    )
)]
async fn root() -> &'static str {
    "Welcome to the echo's Weather Proxy (https://github.com/echo-webkom/weather-proxy)! Visit /weather for current weather data."
}

#[utoipa::path(
    get,
    path = "/weather",
    responses(
        (status = 200, description = "Current weather data", body = WeatherData)
    )
)]
async fn weather(State(state): State<AppState>) -> Json<repo::WeatherData> {
    let weather = state.weather_service.get_weather().await;
    Json(weather)
}
