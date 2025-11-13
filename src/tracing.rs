use tracing_subscriber::{Layer, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::Config;

pub struct Tracing;

impl Tracing {
    pub fn init(config: &Config) {
        let env_filter =
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            });

        let fmt_layer = if config.is_production {
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
