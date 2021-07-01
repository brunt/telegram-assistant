use serde_derive::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone)]
pub(crate) struct EnviroApi {
    pub(crate) url: String,
}

impl EnviroApi {
    pub(crate) async fn request_data(&self) -> Result<EnviroResponse, reqwest::Error> {
        let data = reqwest::get(&self.url).await?.json().await?;
        Ok(data)
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct EnviroResponse {
    pub(crate) gas: GasData,
    pub(crate) humidity: f32,
    pub(crate) light: f32,
    pub(crate) pressure: f32,
    pub(crate) temperature: f32,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct GasData {
    pub(crate) adc: Option<f32>,
    pub(crate) nh3: f32,
    pub(crate) oxidising: f32,
    pub(crate) reducing: f32,
}

impl fmt::Display for EnviroResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Temperature: {:.2} °C\nLight: {:.2} Lux\nPressure: {:.2} hPa\nRelative Humidity: {:.2}%\n\nAir data:\n{:.2}",
            self.temperature, self.light, self.pressure, self.humidity, self.gas
        )
    }
}

impl fmt::Display for GasData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "NH3: {:.2} Ohms\nOxidizing: {:.2} Ohms\nReducing: {:.2} Ohms",
            self.nh3, self.oxidising, self.reducing
        )
    }
}

pub(crate) fn help_thermostat() -> &'static str {
    "Thermostat data from a Pimoroni Enviro+:\nJust say 'Thermostat'"
}
