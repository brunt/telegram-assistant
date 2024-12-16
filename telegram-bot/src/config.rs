use crate::enviroplus::EnviroApi;
use crate::metro::MetroScheduleAPI;
use crate::news::NewsAPI;
use crate::notifications::NotificationService;
use crate::openweather::OpenWeatherApi;
use crate::spending::SpendingAPI;
use std::env;
use std::sync::Arc;
// use sysinfo::{
//     CpuRefreshKind, NetworkExt, NetworksExt, ProcessExt, RefreshKind, System, SystemExt,
// };

#[derive(Debug, Clone)]
pub struct Config {
    pub(crate) spending_api: Arc<SpendingAPI>,
    pub(crate) metro_api: Arc<MetroScheduleAPI>,
    pub(crate) enviro_api: Arc<EnviroApi>,
    pub(crate) openweather: Arc<OpenWeatherApi>,
    pub(crate) news_api: Arc<NewsAPI>,
    pub(crate) notification_service: Arc<NotificationService>,
    // pub(crate) sysinfo: System,
}

impl Config {
    pub fn from_env() -> Config {
        Self {
            spending_api: Arc::new(env::var("SPENDING_API_URL").map_or(
                SpendingAPI::default(),
                |spending_base_url| SpendingAPI {
                    spending_add_url: format!("{spending_base_url}/spent"),
                    spending_total_url: format!("{spending_base_url}/spent"),
                    spending_reset_url: format!("{spending_base_url}/reset"),
                    budget_set_url: format!("{spending_base_url}/budget"),
                },
            )),
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
            news_api: Arc::new(NewsAPI::default()),
            notification_service: Arc::new(NotificationService::default()),
            // sysinfo: System::new_with_specifics(
            //     RefreshKind::new()
            //         .with_memory()
            //         .with_cpu(CpuRefreshKind::everything())
            //         .with_networks()
            //         .with_components(),
            // ),
        }
    }
}
