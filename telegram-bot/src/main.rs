extern crate core;

use teloxide::dispatching::UpdateHandler;
// use actix_web::{App, HttpServer};
// use actix_web_prom::{PrometheusMetrics, PrometheusMetricsBuilder};
// use prometheus::{opts, IntCounterVec};
// use std::collections::HashMap;
use teloxide::prelude::*;

mod config;
mod dispatch;
mod enviroplus;
mod metro;
mod openweather;
mod spending;

use crate::dispatch::handler;
use config::Config;
use crate::openweather::OpenWeatherApi;

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "Weather data from home, indoors")]
    Thermostat,
    // #[command(description = "Track Spending")]
    // Spending,
    // #[command(description = "STL Metro train schedule")]
    // Metro,
}

/// help -> endpoint
/// thermostat -> endpoint
/// spending -> dialogue? -> endpoint
/// metro -> dialogue? -> endpoint

#[tokio::main]
async fn main() {
    // let prometheus = PrometheusMetricsBuilder::new("teloxide")
    //     .endpoint("/metrics")
    //     .const_labels(HashMap::new())
    //     .build()
    //     .unwrap();
    // let counter_opts = opts!("counter", "requests").namespace("teloxide");
    // let counter = IntCounterVec::new(counter_opts, &["request"]).unwrap();
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
    let bot = Bot::from_env().auto_send();

    Dispatcher::builder(bot, schema())
        // .dependencies()
        .build()
        .dispatch()
        .await;
}


fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    Update::filter_message()
        .branch(Message::filter_location().endpoint(|location| {
            match config.openweather.request_data(location.lat as f32, location.lon as f32).await {
                Ok(resp) => bot.send_message(msg.chat.id, "hi"),
                Err(_e) => bot.send_message(msg.chat.id, "error getting weather data"),
            }
        }))
        .branch(teloxide::filter_command()
            .branch(case![Command::Help].endpoint(helpmsg))
            .branch(case![Command::Thermostat].endpoint(thermostat)))
}

async fn helpmsg(bot: AutoSend<Bot>, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
    Ok(())
}

async fn thermostat(bot: AutoSend<Bot>, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "hi").await?;
    Ok(())
}