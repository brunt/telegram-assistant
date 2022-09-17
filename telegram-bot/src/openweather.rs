use core::fmt;
use serde_derive::{Deserialize, Serialize};
use std::env;
use chrono::prelude::*;

#[derive(Debug, Clone)]
pub(crate) struct OpenWeatherApi {
    pub(crate) units: String,
    pub(crate) app_id: String,
}

impl Default for OpenWeatherApi {
    fn default() -> Self {
        Self {
            units: "imperial".to_string(),
            app_id: env::var("OPENWEATHER_API_KEY").expect("missing OPENWEATHER_API_KEY"),
        }
    }
}

impl OpenWeatherApi {
    pub(crate) fn new(app_id: String, units: String) -> Self {
        Self { units, app_id }
    }

    pub(crate) async fn request_data(
        &self,
        lat: f32,
        lon: f32,
    ) -> Result<OpenWeatherResponse, reqwest::Error> {
        let url = format!("https://api.openweathermap.org/data/2.5/weather?units={units}&lat={lat}&lon={lon}&appid={app_id}", units = self.units, lat = lat, lon = lon, app_id = self.app_id);
        let data = reqwest::get(&url).await?.json().await?;
        Ok(data)
    }
}

//TODO: remove debug derive if possible

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct OpenWeatherResponse {
    #[serde(rename = "coord")]
    pub(crate) coordinates: Coord,
    pub(crate) weather: Vec<Weather>,
    pub(crate) base: String,
    pub(crate) main: Main,
    pub(crate) visibility: i32,
    pub(crate) wind: Wind,
    pub(crate) rain: Option<Precipitation>,
    pub(crate) snow: Option<Precipitation>,
    pub(crate) clouds: Clouds,
    dt: i32,
    sys: Sys,
    timezone: i32,
    id: u32,
    name: String,
    cod: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Coord {
    pub(crate) lon: f32,
    pub(crate) lat: f32,
}
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Weather {
    pub(crate) id: i32,
    pub(crate) main: String,
    pub(crate) description: String,
    pub(crate) icon: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Main {
    pub(crate) temp: f32,
    pub(crate) feels_like: f32,
    pub(crate) pressure: f32,
    pub(crate) humidity: f32,
    pub(crate) temp_min: f32,
    pub(crate) temp_max: f32,
}
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Wind {
    pub(crate) speed: f32,
    pub(crate) deg: i32,
    pub(crate) gust: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Precipitation {}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Clouds {
    pub(crate) all: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Sys {
    #[serde(rename = "type")]
    pub(crate) sys_type: i32,
    pub(crate) id: u32,
    pub(crate) message: Option<f32>,
    pub(crate) country: String,
    pub(crate) sunrise: u64,
    pub(crate) sunset: u64,
}

impl fmt::Display for OpenWeatherResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sunrise_str = DateTime<Local>::from_timestamp(self.sys.sunrise);
        write!(
            f,
            r#"Weather: {},
Temperature: {:.2} °F
Feels like: {:.2} °F
Min: {:.2} °F
Max: {:.2} °F
Pressure: {},
Humidity: {},
Sunrise: {},
Sunset: {},
Wind Speed: {},
Direction: {},
Wind Gust: {},

            "#,
            self.weather.first().map_or("unknown", |w| w.description.as_str()),
            self.main.temp,
            self.main.feels_like,
            self.main.temp_min,
            self.main.temp_max,
            self.main.pressure,
            self.main.humidity,
            self.sys.sunrise, //todo chrono convert
            self.sys.sunset,  //todo chrono convert

        )
    }
}
