// use sysinfo::SystemExt;
use crate::parser::{
    is_spending_reset_request, is_spending_total_request, parse_budget_request,
    parse_metro_request, parse_spending_request,
};
use crate::{Config, HandlerResult};
use metro_schedule::NextArrivalRequest;
use spending_tracker::SpentRequest;
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
        .branch(
            Message::filter_text()
                .filter_map(parse_metro_request)
                .endpoint(metro_endpoint),
        )
        .branch(
            Message::filter_text()
                .filter(is_spending_reset_request)
                .endpoint(spending_reset_endpoint),
        )
        .branch(
            Message::filter_text()
                .filter(is_spending_total_request)
                .endpoint(spending_total_endpoint),
        )
        .branch(
            Message::filter_text()
                .filter_map(parse_spending_request)
                .endpoint(spending_endpoint),
        )
        .branch(
            Message::filter_text()
                .filter_map(parse_budget_request)
                .endpoint(budget_endpoint),
        )
}

fn helpmsg() -> String {
    Command::descriptions().to_string()
}

async fn thermostat(config: Config) -> String {
    config
        .enviro_api
        .request_data()
        .await
        .map_or("error getting enviro+ data".to_string(), |resp| {
            resp.to_string()
        })
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

async fn get_news(config: Config) -> String {
    config
        .news_api
        .request_data()
        .await
        .map_or("error getting news data".to_string(), |resp| {
            resp.to_string()
        })
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

async fn metro_endpoint(
    bot: Bot,
    msg: Message,
    req: NextArrivalRequest,
    config: Config,
) -> HandlerResult {
    bot.send_message(
        msg.chat.id,
        config
            .metro_api
            .next_arrival_request(req)
            .await
            .map_or("error getting metro schedule data".to_string(), |resp| {
                resp.to_string()
            }),
    )
    .await?;
    Ok(())
}

async fn spending_reset_endpoint(bot: Bot, msg: Message, config: Config) -> HandlerResult {
    bot.send_message(
        msg.chat.id,
        config
            .spending_api
            .spending_reset_request()
            .await
            .map_or("error calling spending api".to_string(), |resp| {
                resp.to_string()
            }),
    )
    .await?;
    Ok(())
}

async fn spending_total_endpoint(bot: Bot, msg: Message, config: Config) -> HandlerResult {
    bot.send_message(
        msg.chat.id,
        config
            .spending_api
            .spending_total_request()
            .await
            .map_or("error calling spending api".to_string(), |resp| {
                resp.to_string()
            }),
    )
    .await?;
    Ok(())
}

async fn spending_endpoint(
    bot: Bot,
    msg: Message,
    req: SpentRequest,
    config: Config,
) -> HandlerResult {
    bot.send_message(
        msg.chat.id,
        config
            .spending_api
            .spending_request(req)
            .await
            .map_or("error calling spending api".to_string(), |resp| {
                resp.to_string()
            }),
    )
    .await?;
    Ok(())
}

async fn budget_endpoint(
    bot: Bot,
    msg: Message,
    req: SpentRequest,
    config: Config,
) -> HandlerResult {
    bot.send_message(
        msg.chat.id,
        config
            .spending_api
            .budget_set_request(req)
            .await
            .map_or("error calling spending api".to_string(), |resp| {
                resp.to_string()
            }),
    )
    .await?;
    Ok(())
}

async fn commands_handler(bot: Bot, msg: Message, cmd: Command, config: Config) -> HandlerResult {
    bot.send_message(
        msg.chat.id,
        match cmd {
            Command::Help => helpmsg(),
            Command::Thermostat => thermostat(config).await,
            Command::News => get_news(config).await,
        },
    )
    .await?;
    Ok(())
}
