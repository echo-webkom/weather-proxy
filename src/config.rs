pub struct Config {
    pub is_production: bool,
}

impl Config {
    pub fn from_env() -> Self {
        let is_production = std::env::var("ENVIRONMENT")
            .unwrap_or_default()
            .to_lowercase()
            == "production";

        Self { is_production }
    }
}
