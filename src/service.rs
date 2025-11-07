use std::sync::Arc;

use crate::repo::{WeatherCache, WeatherData, WeatherRepo};

pub struct WeatherService {
    repo: Arc<dyn WeatherRepo>,
    cache: Arc<dyn WeatherCache>,
}

impl WeatherService {
    pub fn new(repo: Arc<dyn WeatherRepo>, cache: Arc<dyn WeatherCache>) -> Self {
        Self { repo, cache }
    }

    pub async fn get_weather(&self) -> WeatherData {
        let cached = self.cache.get_cached_weather();
        if let Some(cache_value) = cached {
            let age = chrono::Utc::now() - cache_value.timestamp;
            if age.num_minutes() < 30 {
                return cache_value.data;
            }
        }

        let weather = self.repo.get_weather().await;
        self.cache.set_cached_weather(weather.clone());
        weather
    }
}
