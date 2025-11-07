pub mod cache;
pub mod yr;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum WeatherCondition {
    Sunny,
    Cloudy,
    Rainy,
    Snowy,
}

#[derive(Debug, Clone, Serialize)]
pub struct WeatherData {
    pub temperature: f32,
    pub condition: Option<WeatherCondition>,
    pub wind_speed: f32,
}

#[async_trait::async_trait]
pub trait WeatherRepo: Send + Sync {
    async fn get_weather(&self) -> WeatherData;
}

#[derive(Debug, Clone)]
pub struct CacheValue<T> {
    pub data: T,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl<T> CacheValue<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            timestamp: chrono::Utc::now(),
        }
    }
}

pub trait WeatherCache: Send + Sync {
    fn get_cached_weather(&self) -> Option<CacheValue<WeatherData>>;
    fn set_cached_weather(&self, data: WeatherData);
}
