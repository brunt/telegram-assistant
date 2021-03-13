use crate::config::Config;
use crate::metro::{help_schedule, is_next_arrival_request};
use crate::spending::{help_spending, is_spent_category_request, is_spent_request};
use metro_schedule::NextArrivalRequest;
use prometheus::IntCounterVec;
use teloxide::prelude::*;

fn helpmsg() -> &'static str {
    "Use the following for additional details:\nhelp schedule\nhelp spending\nhelp weather"
}

pub(crate) async fn parse_messages(
    msg: UpdateWithCx<Message>,
    config: Config,
    counter: IntCounterVec,
) {
    if let Some(txt) = msg.update.text() {
        if txt.eq("Help") {
            counter.with_label_values(&["Help"]).inc();
            msg.answer_str(helpmsg()).await.unwrap();
        } else if txt.eq("Help schedule") {
            counter.with_label_values(&["Help schedule"]).inc();
            msg.answer_str(help_schedule()).await.unwrap();
        } else if txt.eq("Help spending") {
            counter.with_label_values(&["Help spending"]).inc();
            msg.answer_str(help_spending()).await.unwrap();
        } else if is_next_arrival_request(txt) {
            counter.with_label_values(&["Next Arrival"]).inc();
            let data_vec: Vec<&str> = txt.splitn(2, ' ').collect();
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
        } else if is_spent_category_request(txt) {
            counter.with_label_values(&["Spending"]).inc();
            let category: &str = txt.splitn(3, ' ').last().unwrap();
            msg.answer_str(
                &config
                    .spending_api
                    .parse_spent_request(txt, Some(category.into()))
                    .await,
            )
            .await
            .unwrap();
        } else if is_spent_request(txt) {
            counter.with_label_values(&["Spending"]).inc();
            msg.answer_str(&config.spending_api.parse_spent_request(txt, None).await)
                .await
                .unwrap();
        }
    }
}
