use crate::{metro::help_schedule, Config, HandlerResult};
use metro_schedule::NextArrivalRequest;
use teloxide::dispatching::{HandlerExt, MessageFilterExt, UpdateFilterExt, UpdateHandler};
use teloxide::prelude::{Message, Requester, Update};
use teloxide::utils::command::BotCommands;
use teloxide::types::Location;
use teloxide::{dptree, Bot};

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "Weather data from home, indoors")]
    Thermostat,
    //TODO: maybe this should be dialogue as it can be complicated to have all functionality in a single struct
    // #[command(description = "Track Spending", parse_with = "split")]
    // Spending{ amount: f64, category: Option<String>},
    #[command(description = "STL Metro train schedule", parse_with = "split")]
    Metro { station: String, direction: String },
}

pub(crate) fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    Update::filter_message()
        .branch(Message::filter_location().endpoint(weather_req))
        .branch(
            dptree::entry()
                .filter_command::<Command>()
                .endpoint(commands_handler),
        )
}

async fn helpmsg(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await?;
    Ok(())
}

async fn thermostat(bot: Bot, msg: Message, config: Config) -> HandlerResult {
    bot.send_message(
        msg.chat.id,
        config
            .enviro_api
            .request_data()
            .await
            .map_or("error getting enviro+ data".to_string(), |resp| {
                resp.to_string()
            }),
    )
    .await?;
    Ok(())
}

async fn weather_req(bot: Bot, msg: Message, location: Location, config: Config) -> HandlerResult {
    bot.send_message(
        msg.chat.id,
        config
            .openweather
            .request_data(location.latitude, location.longitude)
            .await
            .map_or("error getting openweather data".to_string(), |resp| {
                resp.to_string()
            }),
    )
    .await?;
    Ok(())
}

async fn commands_handler(bot: Bot, msg: Message, cmd: Command, config: Config) -> HandlerResult {
    match cmd {
        Command::Metro { station, direction } => {
            bot.send_message(
                msg.chat.id,
                config
                    .metro_api
                    .next_arrival_request(NextArrivalRequest { station, direction })
                    .await
                    .map_or_else(|_| help_schedule().to_string(), |resp| resp.to_string()),
            )
            .await?;
            Ok(())
        }
        Command::Help => helpmsg(bot, msg).await,
        Command::Thermostat => thermostat(bot, msg, config).await,
    }
}
