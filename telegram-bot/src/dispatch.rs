// use sysinfo::SystemExt;
use crate::parser::parse_metro_request;
use crate::{Config, HandlerResult};
// use spending_tracker::SpentRequest;
use teloxide::dispatching::{HandlerExt, MessageFilterExt, UpdateFilterExt, UpdateHandler};
use teloxide::prelude::{Message, Requester, Update};
use teloxide::types::Location;
use teloxide::utils::command::BotCommands;
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
    #[command(description = "Get some recent news")]
    News,
    // #[command(description = "Get hardware system info for this bot")]
    // System,
}

pub(crate) fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    Update::filter_message()
        .branch(Message::filter_location().endpoint(weather_req))
        .branch(
            dptree::entry()
                .filter_command::<Command>()
                .endpoint(commands_handler),
        )
        .branch(dptree::entry().endpoint(text_handler))
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

async fn get_news(bot: Bot, msg: Message, config: Config) -> HandlerResult {
    bot.send_message(
        msg.chat.id,
        config
            .news_api
            .request_data()
            .await
            .map_or("error getting news data".to_string(), |resp| {
                resp.to_string()
            }),
    )
    .await?;
    Ok(())
}

// async fn get_sysinfo(bot: Bot, msg: Message, mut config: Config) -> HandlerResult {
//     config.sysinfo.refresh_all();
//     bot.send_message(
//         msg.chat.id,
//         config.sysinfo.free_memory().to_string()
//     )
//     .await?;
//     Ok(())
// }

async fn text_handler(bot: Bot, msg: Message, config: Config) -> HandlerResult {
    if let Some(txt) = msg.text() {
        if let Some(metro_req) = parse_metro_request(txt) {
            bot.send_message(
                msg.chat.id,
                serde_json::to_string(
                    &config
                        .metro_api
                        .next_arrival_request(metro_req)
                        .await
                        .map_or("error getting metro schedule data".to_string(), |resp| {
                            resp.to_string()
                        }),
                )?,
            )
            .await?;
        }
    }
    Ok(())
}

async fn commands_handler(bot: Bot, msg: Message, cmd: Command, config: Config) -> HandlerResult {
    match cmd {
        Command::Help => helpmsg(bot, msg).await,
        Command::Thermostat => thermostat(bot, msg, config).await,
        Command::News => get_news(bot, msg, config).await,
        // Command::System => get_sysinfo(bot, msg, config).await,
    }
}
