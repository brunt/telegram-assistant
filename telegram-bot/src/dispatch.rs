use crate::config::Config;
use metro_schedule::NextArrivalRequest;
use prometheus::IntCounterVec;
use spending_tracker::Category;
use std::error::Error;
use teloxide::adaptors::DefaultParseMode;
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
    NextArrival { station: String, direction: String },
    // #[command(
    //     rename = "lowercase",
    //     description = "keep a running balance against a set budget",
    //     parse_with = "split"
    // )]
    // Spent {
    //     amount: String,
    //     // category: Option<Category>,
    // },
    // #[command(description = "see running budget total")]
    // SpentTotal,
    // #[command(rename = "spent-reset",description = "reset spending amount")]
    // SpentReset,
    // #[command(description = "set a spending budget")]
    // Budget(String),
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
        Command::NextArrival { station, direction } => {
            dbg!("{}, {}", &station, &direction);
            counter.with_label_values(&["Next Arrival"]).inc();
            // config.metro_api.next_arrival_request(NextArrivalRequest{
            //     station,
            //     direction,
            // }).await.map_or_else(
            //     |_| "error getting data".to_string(),
            //     |resp| dbg!(resp.to_string()),
            //     )
                        match &config
                .metro_api
                .next_arrival_request(NextArrivalRequest {
                    station: station.to_lowercase(),
                    direction: direction.to_lowercase(),
                })
                .await
            {
                Ok(s) => {
                    s.to_string()
                }
                Err(e) => {
                    dbg!(e);
                    "yeet".to_string()
                }
            }
        }
        // Command::Spent {
        //     amount
        //     // category: Some(category)
        // } => {
        //     counter.with_label_values(&["Spending"]).inc();
        //     // config.spending_api.parse_spent_request(txt, None).await
        //     // &config.spending_api.
        //     "".to_string()
        //
        // }
        // Command::Budget(amount) => {
        //     counter.with_label_values(&["Spending"]).inc();
        //     "".to_string()
        // }
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

fn opt(input: String) -> Result<(Option<String>,), ParseError> {
    match input.split_whitespace().count() {
        0 => Ok((None,)),
        1 => Ok((Some(input),)),
        n => Err(ParseError::TooManyArguments {
            expected: 1,
            found: n,
            message: String::from("Wrong number of arguments"),
        }),
    }
}

// pub(crate) async fn parse_messages(
//     msg: UpdateWithCx<AutoSend<Bot>, Message>,
//     config: Config,
//     counter: IntCounterVec,
// ) {
//     if let Some(txt) = msg.update.text() {


//         } else if is_next_arrival_request(txt) {
//             counter.with_label_values(&["Next Arrival"]).inc();
//             let data_vec: Vec<&str> = txt.splitn(2, ' ').collect();
//             match &config
//                 .metro_api
//                 .next_arrival_request(NextArrivalRequest {
//                     station: data_vec[1].to_string().to_lowercase(),
//                     direction: data_vec[0].to_string().to_lowercase(),
//                 })
//                 .await
//             {
//                 Ok(s) => {
//                     msg.answer(s.to_string()).await.unwrap();
//                 }
//                 Err(_) => {
//                     msg.answer("An error occurred retrieving the schedule")
//                         .await
//                         .unwrap();
//                 }
//             }
//         } else if is_spent_category_request(txt) {
//             counter.with_label_values(&["Spending"]).inc();
//             let category: &str = txt.splitn(3, ' ').last().unwrap();
//             msg.answer(
//                 &config
//                     .spending_api
//                     .parse_spent_request(txt, Some(category.into()))
//                     .await,
//             )
//             .await
//             .unwrap();
//         } else if is_spent_request(txt) {
//             counter.with_label_values(&["Spending"]).inc();
//             msg.answer(&config.spending_api.parse_spent_request(txt, None).await)
//                 .await
//                 .unwrap();
//         }
//     }
// }
