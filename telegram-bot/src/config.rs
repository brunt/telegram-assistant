use crate::enviroplus::EnviroApi;
use crate::metro::MetroScheduleAPI;
use crate::openweather::OpenWeatherApi;
// use crate::spending::SpendingAPI;
use std::env;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub(crate) struct Config {
    // pub(crate) spending_api: Arc<SpendingAPI>,
    pub(crate) metro_api: Arc<MetroScheduleAPI>,
    pub(crate) enviro_api: Arc<EnviroApi>,
    pub(crate) openweather: Arc<OpenWeatherApi>,
}

impl Config {
    pub(crate) fn from_env() -> Config {
        Self {
            // spending_api: Arc::new(env::var("SPENDING_API_URL").map_or(
            //     SpendingAPI::default(),
            //     |spending_base_url| SpendingAPI {
            //         spending_add_url: format!("{}/spent", spending_base_url),
            //         spending_total_url: format!("{}/spent", spending_base_url),
            //         spending_reset_url: format!("{}/reset", spending_base_url),
            //         budget_set_url: format!("{}/budget", spending_base_url),
            //     },
            // )),
            metro_api: Arc::new(
                env::var("METRO_API_URL")
                    .map_or(MetroScheduleAPI::default(), |url| MetroScheduleAPI { url }),
            ),
            enviro_api: Arc::new(
                env::var("ENVIRO_API_URL").map_or(EnviroApi::default(), |url| EnviroApi { url }),
            ),
            // webserver_port: Into::<Arc<str>>::into(
            //     env::var("BOT_METRICS_PORT").unwrap_or_else(|_| "8010".to_string()),
            // ),
            openweather: Arc::new(OpenWeatherApi::default()),
        }
    }
}
