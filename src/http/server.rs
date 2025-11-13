use std::sync::Arc;

use axum::{Router, http::HeaderValue};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

use crate::{http::routes, service::weather::WeatherService};

#[derive(Clone)]
pub struct AppState {
    pub weather_service: Arc<WeatherService>,
}

#[derive(OpenApi)]
#[openapi(
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

pub struct Server {
    router: Router,
}

impl Server {
    pub fn new(weather_service: Arc<WeatherService>) -> Self {
        let state = AppState { weather_service };

        let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
            .routes(routes!(routes::root))
            .routes(routes!(routes::weather))
            .with_state(state)
            .split_for_parts();

        let router = router
            .merge(SwaggerUi::new("/swagger").url("/api-docs/openapi.json", api.clone()))
            .layer(
                CorsLayer::permissive()
                    .allow_origin([
                        "http://localhost:5173".parse::<HeaderValue>().unwrap(),
                        "https://screen.echo-webkom.no".parse::<HeaderValue>().unwrap()
                        ])
            )
            .layer(TraceLayer::new_for_http());

        Self { router }
    }

    pub async fn run(self) {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        let port = listener.local_addr().unwrap().port();

        tracing::info!("🚀 Running Axum REST API on http://localhost:{port}");
        tracing::info!("📖 Swagger UI available at http://localhost:{port}/swagger");

        axum::serve(listener, self.router).await.unwrap();
    }
}
