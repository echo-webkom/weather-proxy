use tracing_subscriber::{Layer, fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub struct Tracing;

impl Tracing {
    pub fn init() {
        let env_filter =
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            });

        let is_prod = std::env::var("ENVIRONMENT")
            .unwrap_or_default()
            .to_lowercase()
            == "production";

        let fmt_layer = if is_prod {
            fmt::layer().json().boxed()
        } else {
            fmt::layer().boxed()
        };

        tracing_subscriber::registry()
            .with(env_filter)
            .with(fmt_layer)
            .init();
    }
}
