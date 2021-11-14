use crate::config::Config;
use crate::metro::help_schedule;
use metro_schedule::NextArrivalRequest;
use prometheus::IntCounterVec;
use spending_tracker::{Category, SpentRequest};
use std::error::Error;
use teloxide::types::Me;
use teloxide::{
    prelude::*,
    utils::command::{BotCommand, ParseError},
};

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "Supported Commands:")]
enum Command {
    #[command(description = "display this text")]
    Help,
    #[command(
        rename = "arrival",
        description = "next arriving train at station heading in direction",
        parse_with = "split"
    )]
    NextArrival { direction: String, station: String },
    #[command(
        rename = "lowercase",
        description = "keep a running balance against a set budget",
        parse_with = "opt_category"
    )]
    Spent {
        amount: String,
        category: Option<Category>,
    },
    #[command(rename = "total", description = "see running budget total")]
    SpentTotal,
    #[command(rename = "reset", description = "reset spending amount")]
    SpentReset,
    #[command(description = "set a spending budget")]
    Budget(String),
    #[command(description = "read thermostat values")]
    Thermostat,
}

type Bot = AutoSend<teloxide::Bot>;
type Cx = UpdateWithCx<Bot, Message>;

pub(crate) async fn handler(
    cx: Cx,
    config: Config,
    counter: IntCounterVec,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let Me { user, .. } = cx.requester.get_me().await.expect("get_me() failed");
    let name = user.username.expect("Bots must have usernames");

    let text = match cx.update.text() {
        None => return Ok(()),
        Some(text) => text,
    };

    let command = match Command::parse(text, name) {
        Err(_) => return Ok(()),
        Ok(command) => command,
    };

    let ans = match command {
        Command::Help => {
            counter.with_label_values(&["Help"]).inc();
            Command::descriptions()
        }
        Command::NextArrival { direction, station } => {
            counter.with_label_values(&["Next Arrival"]).inc();
            config
                .metro_api
                .next_arrival_request(NextArrivalRequest { station, direction })
                .await
                .map_or_else(|_| help_schedule().to_string(), |resp| resp.to_string())
        }
        Command::Spent { amount, category } => {
            counter.with_label_values(&["Spending"]).inc();
            config
                .spending_api
                .spending_request(SpentRequest {
                    amount: amount.parse::<f64>()?,
                    category,
                })
                .await
                .map_or_else(
                    |_| "error getting data".to_string(),
                    |resp| resp.to_string(),
                )
        }
        Command::SpentTotal => {
            counter.with_label_values(&["Spending"]).inc();
            config
                .spending_api
                .spending_total_request()
                .await
                .map_or_else(
                    |_| "error getting data".to_string(),
                    |resp| resp.to_string(),
                )
        }
        Command::SpentReset => {
            counter.with_label_values(&["Spending"]).inc();
            config
                .spending_api
                .spending_reset_request()
                .await
                .map_or_else(
                    |_| "error getting data".to_string(),
                    |resp| resp.to_string(),
                )
        }
        Command::Budget(amount) => {
            counter.with_label_values(&["Spending"]).inc();
            config
                .spending_api
                .budget_set_request(SpentRequest {
                    amount: amount.parse::<f64>()?,
                    category: None,
                })
                .await
                .map_or_else(
                    |_| "error getting data".to_string(),
                    |resp| resp.to_string(),
                )
        }
        Command::Thermostat => {
            counter.with_label_values(&["Thermostat"]).inc();
            config.enviro_api.request_data().await.map_or_else(
                |_| "error getting data".to_string(),
                |resp| resp.to_string(),
            )
        }
    };

    cx.answer(&ans).await?;
    Ok(())
}

fn opt_category(input: String) -> Result<(String, Option<Category>), ParseError> {
    let split = input.split(' ').collect::<Vec<_>>();
    match split.len() {
        0 => Ok((input, None)),
        1 => Ok((split[0].to_string(), None)),
        2 => Ok((split[0].to_string(), Some(Category::from(split[1])))),
        n => Err(ParseError::TooManyArguments {
            expected: 1,
            found: n,
            message: "Wrong number of arguments".to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_category() {
        if let Command::Spent { amount, category } = Command::parse("/spent 4.00", "").unwrap() {
            assert_eq!(amount, "4.00".to_string());
            assert!(category.is_none());
        }

        if let Command::Spent { amount, category } =
            Command::parse("/spent 12.3 dining", "").unwrap()
        {
            assert_eq!(amount, "12.3".to_string());
            assert!(matches!(category, Some(Category::Dining)));
        }

        assert!(Command::parse("/spent 12.3 dining ascfg", "").is_err())
    }
}
