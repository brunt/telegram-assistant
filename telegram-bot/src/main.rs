extern crate core;

use teloxide::dispatching::UpdateHandler;
use teloxide::utils::command::BotCommands;
// use actix_web::{App, HttpServer};
// use actix_web_prom::{PrometheusMetrics, PrometheusMetricsBuilder};
// use prometheus::{opts, IntCounterVec};
// use std::collections::HashMap;
use teloxide::prelude::*;
use teloxide::requests::Output;
use teloxide::types::Location;

mod config;
mod dispatch;
mod enviroplus;
mod metro;
mod openweather;
mod spending;

// use crate::dispatch::handler;
use crate::openweather::OpenWeatherApi;
use config::Config;

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
        .dependencies(dptree::deps![config])
        .build()
        .dispatch()
        .await;
}

fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    Update::filter_message()
        .branch(Message::filter_location().endpoint(weather_req))
        .branch(
            dptree::entry()
                .filter_command::<Command>()
                .branch(case![Command::Help].endpoint(helpmsg))
                .branch(case![Command::Thermostat].endpoint(thermostat)),
        )
}

async fn helpmsg(bot: AutoSend<Bot>, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await?;
    Ok(())
}

async fn thermostat(bot: AutoSend<Bot>, msg: Message, config: Config) -> HandlerResult {
    // let res = match config.enviro_api.request_data().await {
    //     Ok(resp) => resp.to_string(),
    //     Err(_) => "error getting enviro+ data".to_string(),
    // };
    // bot.send_message(msg.chat.id, res).await?;
    // Ok(())

    bot.send_message(
        msg.chat.id,
        config.enviro_api.request_data().await.map_or(
            "error getting enviro+ data".to_string(),
            |resp| resp.to_string(),
        ),
    )
    .await?;
    Ok(())
}

async fn weather_req(
    bot: AutoSend<Bot>,
    msg: Message,
    location: Location,
    config: Config,
) -> HandlerResult {
    dbg!(location);
    bot.send_message(
        msg.chat.id,
        config
            .openweather
            .request_data(location.latitude, location.longitude)
            .await
            .map_or(
                "error getting openweather data".to_string(),
                |resp| resp.to_string(),
            ),
    )
    .await?;
    Ok(())
}
