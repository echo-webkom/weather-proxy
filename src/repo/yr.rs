#![allow(dead_code)]

use serde::Deserialize;

use crate::repo::{WeatherCondition, WeatherData, WeatherRepo};

#[derive(Debug, Clone, Deserialize)]
pub enum Symbol {
    /// clearsky_day
    #[serde(rename = "clearsky_day")]
    ClearskyDay,
    /// clearsky_night
    #[serde(rename = "clearsky_night")]
    ClearskyNight,
    /// clearsky_polartwilight
    #[serde(rename = "clearsky_polartwilight")]
    ClearskyPolartwilight,
    /// fair_day
    #[serde(rename = "fair_day")]
    FairDay,
    /// fair_night
    #[serde(rename = "fair_night")]
    FairNight,
    /// fair_polartwilight
    #[serde(rename = "fair_polartwilight")]
    FairPolartwilight,
    /// lightssnowshowersandthunder_day
    #[serde(rename = "lightssnowshowersandthunder_day")]
    LightssnowshowersandthunderDay,
    /// lightssnowshowersandthunder_night
    #[serde(rename = "lightssnowshowersandthunder_night")]
    LightssnowshowersandthunderNight,
    /// lightssnowshowersandthunder_polartwilight
    #[serde(rename = "lightssnowshowersandthunder_polartwilight")]
    LightssnowshowersandthunderPolartwilight,
    /// lightsnowshowers_day
    #[serde(rename = "lightsnowshowers_day")]
    LightsnowshowersDay,
    /// lightsnowshowers_night
    #[serde(rename = "lightsnowshowers_night")]
    LightsnowshowersNight,
    /// lightsnowshowers_polartwilight
    #[serde(rename = "lightsnowshowers_polartwilight")]
    LightsnowshowersPolartwilight,
    /// heavyrainandthunder
    #[serde(rename = "heavyrainandthunder")]
    Heavyrainandthunder,
    /// heavysnowandthunder
    #[serde(rename = "heavysnowandthunder")]
    Heavysnowandthunder,
    /// rainandthunder
    #[serde(rename = "rainandthunder")]
    Rainandthunder,
    /// heavysleetshowersandthunder_day
    #[serde(rename = "heavysleetshowersandthunder_day")]
    HeavysleetshowersandthunderDay,
    /// heavysleetshowersandthunder_night
    #[serde(rename = "heavysleetshowersandthunder_night")]
    HeavysleetshowersandthunderNight,
    /// heavysleetshowersandthunder_polartwilight
    #[serde(rename = "heavysleetshowersandthunder_polartwilight")]
    HeavysleetshowersandthunderPolartwilight,
    /// heavysnow
    #[serde(rename = "heavysnow")]
    Heavysnow,
    /// heavyrainshowers_day
    #[serde(rename = "heavyrainshowers_day")]
    HeavyrainshowersDay,
    /// heavyrainshowers_night
    #[serde(rename = "heavyrainshowers_night")]
    HeavyrainshowersNight,
    /// heavyrainshowers_polartwilight
    #[serde(rename = "heavyrainshowers_polartwilight")]
    HeavyrainshowersPolartwilight,
    /// lightsleet
    #[serde(rename = "lightsleet")]
    Lightsleet,
    /// heavyrain
    #[serde(rename = "heavyrain")]
    Heavyrain,
    /// lightrainshowers_day
    #[serde(rename = "lightrainshowers_day")]
    LightrainshowersDay,
    /// lightrainshowers_night
    #[serde(rename = "lightrainshowers_night")]
    LightrainshowersNight,
    /// lightrainshowers_polartwilight
    #[serde(rename = "lightrainshowers_polartwilight")]
    LightrainshowersPolartwilight,
    /// heavysleetshowers_day
    #[serde(rename = "heavysleetshowers_day")]
    HeavysleetshowersDay,
    /// heavysleetshowers_night
    #[serde(rename = "heavysleetshowers_night")]
    HeavysleetshowersNight,
    /// heavysleetshowers_polartwilight
    #[serde(rename = "heavysleetshowers_polartwilight")]
    HeavysleetshowersPolartwilight,
    /// lightsleetshowers_day
    #[serde(rename = "lightsleetshowers_day")]
    LightsleetshowersDay,
    /// lightsleetshowers_night
    #[serde(rename = "lightsleetshowers_night")]
    LightsleetshowersNight,
    /// lightsleetshowers_polartwilight
    #[serde(rename = "lightsleetshowers_polartwilight")]
    LightsleetshowersPolartwilight,
    /// snow
    #[serde(rename = "snow")]
    Snow,
    /// heavyrainshowersandthunder_day
    #[serde(rename = "heavyrainshowersandthunder_day")]
    HeavyrainshowersandthunderDay,
    /// heavyrainshowersandthunder_night
    #[serde(rename = "heavyrainshowersandthunder_night")]
    HeavyrainshowersandthunderNight,
    /// heavyrainshowersandthunder_polartwilight
    #[serde(rename = "heavyrainshowersandthunder_polartwilight")]
    HeavyrainshowersandthunderPolartwilight,
    /// snowshowers_day
    #[serde(rename = "snowshowers_day")]
    SnowshowersDay,
    /// snowshowers_night
    #[serde(rename = "snowshowers_night")]
    SnowshowersNight,
    /// snowshowers_polartwilight
    #[serde(rename = "snowshowers_polartwilight")]
    SnowshowersPolartwilight,
    /// fog
    #[serde(rename = "fog")]
    Fog,
    /// snowshowersandthunder_day
    #[serde(rename = "snowshowersandthunder_day")]
    SnowshowersandthunderDay,
    /// snowshowersandthunder_night
    #[serde(rename = "snowshowersandthunder_night")]
    SnowshowersandthunderNight,
    /// snowshowersandthunder_polartwilight
    #[serde(rename = "snowshowersandthunder_polartwilight")]
    SnowshowersandthunderPolartwilight,
    /// lightsnowandthunder
    #[serde(rename = "lightsnowandthunder")]
    Lightsnowandthunder,
    /// heavysleetandthunder
    #[serde(rename = "heavysleetandthunder")]
    Heavysleetandthunder,
    /// lightrain
    #[serde(rename = "lightrain")]
    Lightrain,
    /// rainshowersandthunder_day
    #[serde(rename = "rainshowersandthunder_day")]
    RainshowersandthunderDay,
    /// rainshowersandthunder_night
    #[serde(rename = "rainshowersandthunder_night")]
    RainshowersandthunderNight,
    /// rainshowersandthunder_polartwilight
    #[serde(rename = "rainshowersandthunder_polartwilight")]
    RainshowersandthunderPolartwilight,
    /// rain
    #[serde(rename = "rain")]
    Rain,
    /// lightsnow
    #[serde(rename = "lightsnow")]
    Lightsnow,
    /// lightrainshowersandthunder_day
    #[serde(rename = "lightrainshowersandthunder_day")]
    LightrainshowersandthunderDay,
    /// lightrainshowersandthunder_night
    #[serde(rename = "lightrainshowersandthunder_night")]
    LightrainshowersandthunderNight,
    /// lightrainshowersandthunder_polartwilight
    #[serde(rename = "lightrainshowersandthunder_polartwilight")]
    LightrainshowersandthunderPolartwilight,
    /// heavysleet
    #[serde(rename = "heavysleet")]
    Heavysleet,
    /// sleetandthunder
    #[serde(rename = "sleetandthunder")]
    Sleetandthunder,
    /// lightrainandthunder
    #[serde(rename = "lightrainandthunder")]
    Lightrainandthunder,
    /// sleet
    #[serde(rename = "sleet")]
    Sleet,
    /// lightssleetshowersandthunder_day
    #[serde(rename = "lightssleetshowersandthunder_day")]
    LightssleetshowersandthunderDay,
    /// lightssleetshowersandthunder_night
    #[serde(rename = "lightssleetshowersandthunder_night")]
    LightssleetshowersandthunderNight,
    /// lightssleetshowersandthunder_polartwilight
    #[serde(rename = "lightssleetshowersandthunder_polartwilight")]
    LightssleetshowersandthunderPolartwilight,
    /// lightsleetandthunder
    #[serde(rename = "lightsleetandthunder")]
    Lightsleetandthunder,
    /// partlycloudy_day
    #[serde(rename = "partlycloudy_day")]
    PartlycloudyDay,
    /// partlycloudy_night
    #[serde(rename = "partlycloudy_night")]
    PartlycloudyNight,
    /// partlycloudy_polartwilight
    #[serde(rename = "partlycloudy_polartwilight")]
    PartlycloudyPolartwilight,
    /// sleetshowersandthunder_day
    #[serde(rename = "sleetshowersandthunder_day")]
    SleetshowersandthunderDay,
    /// sleetshowersandthunder_night
    #[serde(rename = "sleetshowersandthunder_night")]
    SleetshowersandthunderNight,
    /// sleetshowersandthunder_polartwilight
    #[serde(rename = "sleetshowersandthunder_polartwilight")]
    SleetshowersandthunderPolartwilight,
    /// rainshowers_day
    #[serde(rename = "rainshowers_day")]
    RainshowersDay,
    /// rainshowers_night
    #[serde(rename = "rainshowers_night")]
    RainshowersNight,
    /// rainshowers_polartwilight
    #[serde(rename = "rainshowers_polartwilight")]
    RainshowersPolartwilight,
    /// snowandthunder
    #[serde(rename = "snowandthunder")]
    Snowandthunder,
    /// sleetshowers_day
    #[serde(rename = "sleetshowers_day")]
    SleetshowersDay,
    /// sleetshowers_night
    #[serde(rename = "sleetshowers_night")]
    SleetshowersNight,
    /// sleetshowers_polartwilight
    #[serde(rename = "sleetshowers_polartwilight")]
    SleetshowersPolartwilight,
    /// cloudy
    #[serde(rename = "cloudy")]
    Cloudy,
    /// heavysnowshowersandthunder_day
    #[serde(rename = "heavysnowshowersandthunder_day")]
    HeavysnowshowersandthunderDay,
    /// heavysnowshowersandthunder_night
    #[serde(rename = "heavysnowshowersandthunder_night")]
    HeavysnowshowersandthunderNight,
    /// heavysnowshowersandthunder_polartwilight
    #[serde(rename = "heavysnowshowersandthunder_polartwilight")]
    HeavysnowshowersandthunderPolartwilight,
    /// heavysnowshowers_day
    #[serde(rename = "heavysnowshowers_day")]
    HeavysnowshowersDay,
    /// heavysnowshowers_night
    #[serde(rename = "heavysnowshowers_night")]
    HeavysnowshowersNight,
    /// heavysnowshowers_polartwilight
    #[serde(rename = "heavysnowshowers_polartwilight")]
    HeavysnowshowersPolartwilight,
}

#[derive(Debug, Clone, Deserialize)]
pub struct YrCompactResponse {
    /// The type of the weather response
    #[serde(rename = "type")]
    _type: String,

    /// The geometry information
    geometry: YrGeometry,

    /// The properties containing timeseries data
    properties: YrProperties,
}

#[derive(Debug, Clone, Deserialize)]
pub struct YrGeometry {
    /// The type of geometry
    #[serde(rename = "type")]
    _type: String,

    /// The coordinates [longitude, latitude, altitude]
    coordinates: Vec<f64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct YrProperties {
    /// Meta information
    meta: YrMeta,

    /// Timeseries weather data
    timeseries: Vec<YrTimeSeries>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct YrMeta {
    /// The updated time of the data
    updated_at: String,

    /// The units used in the data
    units: YrUnits,
}

#[derive(Debug, Clone, Deserialize)]
pub struct YrUnits {
    /// Air pressure unit
    air_pressure_at_sea_level: String,

    /// Air temperature unit
    air_temperature: String,

    /// Cloud area fraction unit
    cloud_area_fraction: String,

    /// Precipitation amount unit
    precipitation_amount: String,

    /// Relative humidity unit
    relative_humidity: String,

    /// Wind direction unit
    wind_from_direction: String,

    /// Wind speed unit
    wind_speed: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct YrTimeSeries {
    /// The time of the data point
    time: String,
    /// The data details
    data: YrTimeSeriesData,
}

#[derive(Debug, Clone, Deserialize)]
pub struct YrTimeSeriesData {
    /// Instantaneous details
    instant: Option<YrForecastPeriodDetails>,

    /// Next 1 hour forecast
    next_1_hours: Option<YrForecastPeriodDetails>,

    /// Next 6 hours forecast
    next_6_hours: Option<YrForecastPeriodDetails>,

    /// Next 12 hours forecast
    next_12_hours: Option<YrForecastPeriodDetails>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct YrForecastPeriodDetails {
    /// Details at the instant
    details: Option<TimeseriesDetails>,

    /// Summary of the forecast period
    summary: Option<YrForecastSummary>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct YrForecastSummary {
    /// Symbol code representing the weather condition
    symbol_code: Symbol,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TimeseriesDetails {
    /// Air temperature
    air_temperature: Option<f32>,
    /// Cloud area fraction
    cloud_area_fraction: Option<f32>,
    /// Relative humidity
    relative_humidity: Option<f32>,
    /// Wind from direction
    wind_from_direction: Option<f32>,
    /// Wind speed
    wind_speed: Option<f32>,
}

impl From<YrCompactResponse> for WeatherData {
    fn from(yr: YrCompactResponse) -> Self {
        Self {
            temperature: yr
                .properties
                .timeseries
                .first()
                .and_then(|ts| {
                    ts.data.instant.as_ref().and_then(|instant| {
                        instant.details.as_ref().and_then(|d| d.air_temperature)
                    })
                })
                .unwrap_or(0.0),
            condition: yr
                .properties
                .timeseries
                .first()
                .and_then(|ts| {
                    ts.data
                        .next_1_hours
                        .as_ref()
                        .and_then(|next| next.summary.as_ref().map(|s| s.symbol_code.clone()))
                })
                .and_then(|symbol| symbol.into()),
            wind_speed: yr
                .properties
                .timeseries
                .first()
                .and_then(|ts| {
                    ts.data
                        .instant
                        .as_ref()
                        .and_then(|instant| instant.details.as_ref().and_then(|d| d.wind_speed))
                })
                .unwrap_or(0.0),
        }
    }
}

impl From<Symbol> for Option<WeatherCondition> {
    fn from(symbol: Symbol) -> Self {
        use Symbol::*;
        use WeatherCondition::*;

        match symbol {
            // Sunny conditions
            ClearskyDay | ClearskyNight | ClearskyPolartwilight => Some(Sunny),

            // Cloudy conditions
            FairDay
            | FairNight
            | FairPolartwilight
            | PartlycloudyDay
            | PartlycloudyNight
            | PartlycloudyPolartwilight
            | Symbol::Cloudy
            | Fog => Some(WeatherCondition::Cloudy),

            // Rainy conditions
            Lightrain
            | Rain
            | Heavyrain
            | Lightrainandthunder
            | Rainandthunder
            | Heavyrainandthunder
            | LightrainshowersDay
            | LightrainshowersNight
            | LightrainshowersPolartwilight
            | RainshowersDay
            | RainshowersNight
            | RainshowersPolartwilight
            | HeavyrainshowersDay
            | HeavyrainshowersNight
            | HeavyrainshowersPolartwilight
            | LightrainshowersandthunderDay
            | LightrainshowersandthunderNight
            | LightrainshowersandthunderPolartwilight
            | RainshowersandthunderDay
            | RainshowersandthunderNight
            | RainshowersandthunderPolartwilight
            | HeavyrainshowersandthunderDay
            | HeavyrainshowersandthunderNight
            | HeavyrainshowersandthunderPolartwilight => Some(Rainy),

            // Snowy conditions
            Lightsnow
            | Snow
            | Heavysnow
            | Lightsnowandthunder
            | Snowandthunder
            | Heavysnowandthunder
            | LightsnowshowersDay
            | LightsnowshowersNight
            | LightsnowshowersPolartwilight
            | SnowshowersDay
            | SnowshowersNight
            | SnowshowersPolartwilight
            | HeavysnowshowersDay
            | HeavysnowshowersNight
            | HeavysnowshowersPolartwilight
            | LightssnowshowersandthunderDay
            | LightssnowshowersandthunderNight
            | LightssnowshowersandthunderPolartwilight
            | SnowshowersandthunderDay
            | SnowshowersandthunderNight
            | SnowshowersandthunderPolartwilight
            | HeavysnowshowersandthunderDay
            | HeavysnowshowersandthunderNight
            | HeavysnowshowersandthunderPolartwilight => Some(Snowy),

            // Other conditions
            _ => None,
        }
    }
}

pub struct YrWeatherRepo {
    client: reqwest::Client,
}

impl YrWeatherRepo {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .user_agent("echo-weather-proxy/1.0 (+https://weather.echo-webkom.no/)")
            .build()
            .unwrap();

        Self { client }
    }
}

#[async_trait::async_trait]
impl WeatherRepo for YrWeatherRepo {
    async fn get_weather(&self) -> WeatherData {
        tracing::info!("fetching weather data from Yr");

        let resp = self
            .client
            .get(
                "https://api.met.no/weatherapi/locationforecast/2.0/compact?lat=60.3913&lon=5.3221",
            )
            .header("Accept", "application/json")
            .send()
            .await
            .unwrap();

        let data = resp.json::<YrCompactResponse>().await.unwrap();
        WeatherData::from(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_json() {
        let json = include_str!("../../tests/yr.example.json");
        let yr_response: YrCompactResponse = serde_json::from_str(json).unwrap();
        let weather_data: WeatherData = yr_response.into();
        assert_eq!(weather_data.temperature, 13.0);
        assert_eq!(weather_data.wind_speed, 2.5);
    }
}
