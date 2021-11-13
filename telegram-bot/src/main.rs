use actix_web::{App, HttpServer};
use actix_web_prom::{PrometheusMetrics, PrometheusMetricsBuilder};
use prometheus::{opts, IntCounterVec};
use std::collections::HashMap;
use teloxide::prelude::*;
use tokio_stream::wrappers::UnboundedReceiverStream;

mod config;
mod dispatch;
mod enviroplus;
mod metro;
mod spending;

use crate::dispatch::handler;
use config::Config;
// use dispatch::parse_messages;

#[actix_web::main]
async fn main() {
    let prometheus = PrometheusMetricsBuilder::new("teloxide")
        .endpoint("/metrics")
        .const_labels(HashMap::new())
        .build()
        .unwrap();
    let counter_opts = opts!("counter", "requests").namespace("teloxide");
    let counter = IntCounterVec::new(counter_opts, &["request"]).unwrap();
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
    let bot = Bot::from_env().auto_send();
    teloxide::repl(bot, move |cx| handler(cx, config.clone(), counter.clone())).await;
}
