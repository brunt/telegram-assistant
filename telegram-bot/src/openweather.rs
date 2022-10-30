use chrono::prelude::*;
use core::fmt;
use serde_derive::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone)]
pub(crate) struct OpenWeatherApi {
    pub(crate) units: String,
    pub(crate) app_id: String,
    pub(crate) url: String,
}

impl Default for OpenWeatherApi {
    fn default() -> Self {
        Self {
            units: "imperial".to_string(),
            app_id: env::var("OPENWEATHER_API_KEY").expect("missing OPENWEATHER_API_KEY"),
            url: "https://api.openweathermap.org/data/2.5/weather".to_string(),
        }
    }
}

impl OpenWeatherApi {
    pub(crate) async fn request_data(
        &self,
        lat: f64,
        lon: f64,
    ) -> Result<OpenWeatherResponse, reqwest::Error> {
        let url = format!(
            "{url}?units={units}&lat={lat}&lon={lon}&appid={app_id}",
            url = self.url,
            units = self.units,
            lat = lat,
            lon = lon,
            app_id = self.app_id
        );
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
    timezone: i64,
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
    pub(crate) gust: Option<f32>,
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
    pub(crate) sunrise: i64,
    pub(crate) sunset: i64,
}

impl fmt::Display for OpenWeatherResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            r#"Weather: {},
Temperature: {:.2} °F
Feels like: {:.2} °F
Min: {:.2} °F
Max: {:.2} °F
Cloudiness: {}%
Sunrise: {},
Sunset: {},
Pressure: {} hPa,
Humidity: {}%,
Wind Speed: {} mph,
Direction: {} ({}°),
Wind Gust: {} mph"#,
            self.weather
                .first()
                .map_or("unknown", |w| w.description.as_str()),
            self.main.temp,
            self.main.feels_like,
            self.main.temp_min,
            self.main.temp_max,
            self.clouds.all,
            Utc.timestamp(self.sys.sunrise + self.timezone, 0)
                .format("%H:%M:%S"),
            Utc.timestamp(self.sys.sunset + self.timezone, 0)
                .format("%H:%M:%S"),
            self.main.pressure,
            self.main.humidity,
            self.wind.speed,
            //http://snowfence.umn.edu/Components/winddirectionanddegrees.htm
            match self.wind.deg {
                x if !(12..349).contains(&x) => "N",
                x if (12..34).contains(&x) => "NNE",
                x if (34..57).contains(&x) => "NE",
                x if (57..79).contains(&x) => "ENE",
                x if (79..102).contains(&x) => "E",
                x if (102..124).contains(&x) => "ESE",
                x if (124..147).contains(&x) => "SE",
                x if (147..169).contains(&x) => "SSE",
                x if (169..192).contains(&x) => "S",
                x if (192..214).contains(&x) => "SSW",
                x if (214..237).contains(&x) => "SW",
                x if (237..259).contains(&x) => "WSW",
                x if (259..282).contains(&x) => "W",
                x if (282..304).contains(&x) => "WNW",
                x if (304..327).contains(&x) => "NW",
                x if (327..349).contains(&x) => "NNW",
                _ => "?!",
            },
            self.wind.deg,
            self.wind.gust.unwrap_or(0.0),
        )
    }
}
