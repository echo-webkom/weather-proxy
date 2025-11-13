use crate::repo::{CacheValue, WeatherCache, WeatherData};

pub struct InMemoryWeatherCache {
    cache: std::sync::Mutex<Option<CacheValue<WeatherData>>>,
}

impl InMemoryWeatherCache {
    pub fn new() -> Self {
        Self {
            cache: std::sync::Mutex::new(None),
        }
    }
}

impl WeatherCache for InMemoryWeatherCache {
    fn get_cached_weather(&self) -> Option<CacheValue<WeatherData>> {
        tracing::info!(
            message = "fetching cached weather data",
            has_cache = self.cache.lock().unwrap().is_some()
        );

        let cache = self.cache.lock().unwrap();
        cache.clone()
    }

    fn set_cached_weather(&self, data: WeatherData) {
        tracing::info!(message = "updating cached weather data");

        let cache_value = CacheValue::new(data);
        let mut cache = self.cache.lock().unwrap();
        *cache = Some(cache_value);
    }
}
