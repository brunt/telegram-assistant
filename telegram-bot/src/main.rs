use std::sync::Arc;
use telegram_chatbot::{config::Config, dispatch::monitor_thermostat, dispatch::schema};
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    //TODO: re-add prometheus metrics
    let config = Arc::new(Config::from_env());
    tokio::spawn(monitor_thermostat(config.clone()));

    // run_webserver(&config, prometheus);
    run_chatbot(config).await;
}

// fn run_webserver(config: &Config, prometheus: PrometheusMetrics) {
//     HttpServer::new(move || App::new().wrap(prometheus.clone()))
//         .bind(format!("0.0.0.0:{}", &config.webserver_port))
//         .expect("address in use")
//         .run();
// }

async fn run_chatbot(config: Arc<Config>) {
    let bot = Bot::from_env();

    Dispatcher::builder(bot, schema())
        .dependencies(dptree::deps![config])
        .build()
        .dispatch()
        .await;
}
