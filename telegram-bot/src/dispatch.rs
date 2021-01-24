use prometheus::IntCounterVec;
use teloxide::prelude::*;
use metro_schedule::{NextArrivalRequest};
use crate::config::Config;
use crate::metro::{help_schedule, is_next_arrival_request};
use crate::spending::{help_spending, is_spent_category_request, is_spent_request};
use crate::weather::{help_weather, weather_request};

fn helpmsg() -> &'static str {
    "Use the following for additional details:\nhelp schedule\nhelp spending\nhelp weather"
}

pub(crate) async fn parse_messages(
    msg: UpdateWithCx<Message>,
    config: Config,
    counter: IntCounterVec,
) {
    match msg.update.text() {
        None => {
            if let Some(loc) = msg.update.location() {
                counter.with_label_values(&["Weather"]).inc();
                msg.answer_str(
                    weather_request(
                        &config.forecast_token,
                        loc.latitude as f64,
                        loc.longitude as f64,
                    )
                    .await,
                )
                .await
                .unwrap();
            }
        }
        Some(txt) => match txt {
            input if input.eq("Help") => {
                counter.with_label_values(&["Help"]).inc();
                msg.answer_str(helpmsg()).await.unwrap();
            }
            input if input.eq("Help schedule") => {
                counter.with_label_values(&["Help schedule"]).inc();
                msg.answer_str(help_schedule()).await.unwrap();
            }
            input if input.eq("Help spending") => {
                counter.with_label_values(&["Help spending"]).inc();
                msg.answer_str(help_spending()).await.unwrap();
            }
            input if input.eq("Help weather") => {
                counter.with_label_values(&["Help weather"]).inc();
                msg.answer_str(help_weather()).await.unwrap();
            }
            input if is_next_arrival_request(input) => {
                counter.with_label_values(&["Next Arrival"]).inc();
                let data_vec: Vec<&str> = input.splitn(2, ' ').collect();
                match &config
                    .metro_api
                    .next_arrival_request(NextArrivalRequest {
                        station: data_vec[1].to_string().to_lowercase(),
                        direction: data_vec[0].to_string().to_lowercase(),
                    })
                    .await
                {
                    Ok(s) => {
                        msg.answer_str(s.to_string()).await.unwrap();
                    }
                    Err(_) => {
                        msg.answer_str("An error occurred retrieving the schedule")
                            .await
                            .unwrap();
                    }
                }
            }
            input if is_spent_category_request(input) => {
                counter.with_label_values(&["Spending"]).inc();
                let category: &str = input.splitn(3, ' ').last().unwrap();
                msg.answer_str(
                    &config
                        .spending_api
                        .parse_spent_request(input, Some(category.into()))
                        .await,
                )
                .await
                .unwrap();
            }
            input if is_spent_request(input) => {
                counter.with_label_values(&["Spending"]).inc();
                msg.answer_str(&config.spending_api.parse_spent_request(input, None).await)
                    .await
                    .unwrap();
            }
            _ => {}
        },
    }
}
