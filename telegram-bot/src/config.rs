use crate::metro::MetroScheduleAPI;
use crate::spending::SpendingAPI;
use std::env;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub(crate) struct Config {
    pub(crate) spending_api: Arc<SpendingAPI>,
    pub(crate) metro_api: Arc<MetroScheduleAPI>,
    pub(crate) webserver_port: Arc<str>,
}

impl Config {
    pub(crate) fn from_env() -> Config {
        let spending_base_url = env::var("SPENDING_API_URL").unwrap_or("http://localhost:8001".into());
        Config {
            spending_api: Arc::new(SpendingAPI {
                spending_add_url: format!("{}/spent", spending_base_url),
                spending_total_url: format!("{}/spent", spending_base_url),
                spending_reset_url: format!("{}/reset", spending_base_url),
                budget_set_url: format!("{}/budget", spending_base_url),
            }),
            metro_api: Arc::new(MetroScheduleAPI {
                url: env::var("METRO_API_URL").unwrap_or("http://localhost:8000/next-arrival".into()),
            }),
            webserver_port: Into::<Arc<str>>::into(
                env::var("BOT_METRICS_PORT").unwrap_or("8010".into()),
            ),
        }
    }
}
