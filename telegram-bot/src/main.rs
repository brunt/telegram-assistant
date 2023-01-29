extern crate core;

use teloxide::prelude::*;

mod config;
mod dispatch;
mod enviroplus;
mod metro;
mod news;
mod openweather;
mod parser;
mod spending;
// mod sysinfo;

use crate::dispatch::schema;
use config::Config;

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() {
    //TODO: replace actix with axum, re-add prometheus metrics
    let config = Config::from_env();
    // run_webserver(&config, prometheus);
    run_chatbot(config).await;
}

// fn run_webserver(config: &Config, prometheus: PrometheusMetrics) {
//     HttpServer::new(move || App::new().wrap(prometheus.clone()))
//         .bind(format!("0.0.0.0:{}", &config.webserver_port))
//         .expect("address in use")
//         .run();
// }

async fn run_chatbot(config: Config) {
    let bot = Bot::from_env();

    Dispatcher::builder(bot, schema())
        .dependencies(dptree::deps![config])
        .build()
        .dispatch()
        .await;
}
