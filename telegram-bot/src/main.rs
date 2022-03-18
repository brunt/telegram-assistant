use actix_web::{App, HttpServer};
use actix_web_prom::PrometheusMetrics;
use prometheus::{IntCounterVec, opts};
use teloxide::prelude::*;

mod config;
mod dispatch;
mod enviroplus;
mod metro;
mod spending;

use config::Config;
use dispatch::parse_messages;

#[tokio::main]
async fn main() {
    let prometheus = PrometheusMetrics::new("teloxide", Some("/metrics"), None);
    let counter_opts = opts!("counter", "requests").namespace("teloxide");
    let counter = IntCounterVec::new(counter_opts, &["request"]).unwrap();
    prometheus
        .registry
        .register(Box::new(counter.clone()))
        .unwrap();
    let config = Config::from_env();
    run_webserver(&config, prometheus);
    run_chatbot(config, counter).await;
}

fn run_webserver(config: &Config, prometheus: PrometheusMetrics) {
    HttpServer::new(move || App::new().wrap(prometheus.clone()))
        .bind(format!("0.0.0.0:{}", &config.webserver_port))
        .expect("address in use")
        .run();
}

async fn run_chatbot(config: Config, counter: IntCounterVec) {
    let bot = Bot::from_env();
    Dispatcher::new(bot)
        .messages_handler(move |rx: DispatcherHandlerRx<Message>| {
            rx.for_each_concurrent(None, move |msg| {
                parse_messages(msg, config.clone(), counter.clone())
            })
        })
        .dispatch()
        .await;
}
