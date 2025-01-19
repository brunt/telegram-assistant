use std::sync::Arc;
// use sysinfo::SystemExt;
use crate::config::Config;
use crate::parser::{
    is_spending_reset_request, is_spending_total_request, parse_budget_request,
    parse_metro_request, parse_spending_request,
};
use metro_schedule::NextArrivalRequest;
use simple_moving_average::{SumTreeSMA, SMA};
use spending_tracker::SpentRequest;
use teloxide::dispatching::{HandlerExt, MessageFilterExt, UpdateFilterExt, UpdateHandler};
use teloxide::prelude::{ChatId, Message, Requester, Update};
use teloxide::types::Location;
use teloxide::utils::command::BotCommands;
use teloxide::{dptree, Bot};

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub trait RequesterWithNotifications: Requester {
    fn send_with_notification(
        &self,
        chat_id: ChatId,
        text: String,
        has_notifications: bool,
    ) -> <Self as Requester>::SendMessage;
}

impl<R: Requester> RequesterWithNotifications for R {
    fn send_with_notification(
        &self,
        chat_id: ChatId,
        text: String,
        has_notifications: bool,
    ) -> <Self as Requester>::SendMessage {
        let response_text = if has_notifications {
            format!("{}\n\n✉️ Notifications available", text)
        } else {
            text
        };

        self.send_message(chat_id, response_text)
    }
}

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
    #[command(description = "View notifications")]
    Notifications,
    #[command(description = "Clear notifications")]
    ClearNotifications,
    // #[command(description = "Get hardware system info for this bot")]
    // System,
}

pub fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
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

const WINDOW_SIZE: usize = 48;
pub async fn monitor_thermostat(config: Arc<Config>) {
    const SLEEP_DURATION: u64 = 3600;

    let mut sensors = vec![
        (
            "🟢Nitrogen Dioxide",
            SumTreeSMA::<_, f32, WINDOW_SIZE>::new(),
        ),
        (
            "🔴Carbon Monoxide",
            SumTreeSMA::<_, f32, WINDOW_SIZE>::new(),
        ),
        ("🟡Ammonia", SumTreeSMA::<_, f32, WINDOW_SIZE>::new()),
        ("🌡️Temperature", SumTreeSMA::<_, f32, WINDOW_SIZE>::new()),
    ];

    loop {
        if let Ok(resp) = config.enviro_api.request_data().await {
            let values = [
                resp.gas.oxidising,
                resp.gas.reducing,
                resp.gas.nh3,
                resp.temperature,
            ];

            for ((name, sensor), &value) in sensors.iter_mut().zip(values.iter()) {
                sensor.add_sample(value);
                check_and_notify(&config, name, sensor, value).await;
            }
        }
        tokio::time::sleep(std::time::Duration::from_secs(SLEEP_DURATION)).await;
    }
}

async fn check_and_notify(
    config: &Arc<Config>,
    name: &str,
    sensor: &SumTreeSMA<f32, f32, WINDOW_SIZE>,
    value: f32,
) {
    let num_samples = sensor.get_sample_window_iter().count();
    if num_samples == sensor.get_sample_window_size() {
        let avg = sensor.get_average();
        let std_dev = (sensor
            .get_sample_window_iter()
            .map(|n| (n - avg).powi(2))
            .sum::<f32>()
            / num_samples as f32)
            .sqrt();

        if (value - avg).abs() > std_dev {
            let message = format!("{} anomaly: {}", name, value - avg);
            let _ = config
                .notification_service
                .write_notification(message)
                .await;
        }
    }
}

async fn thermostat(config: Arc<Config>) -> String {
    config
        .enviro_api
        .request_data()
        .await
        .map_or("error getting enviro+ data".to_string(), |resp| {
            resp.to_string()
        })
}

async fn weather_req(
    bot: Bot,
    msg: Message,
    location: Location,
    config: Arc<Config>,
) -> HandlerResult {
    bot.send_with_notification(
        msg.chat.id,
        config
            .openweather
            .request_data(location.latitude, location.longitude)
            .await
            .map_or("error getting openweather data".to_string(), |resp| {
                resp.to_string()
            }),
        config
            .notification_service
            .has_notifications()
            .await
            .unwrap_or(false),
    )
    .await?;
    Ok(())
}

async fn get_news(config: Arc<Config>) -> String {
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
    config: Arc<Config>,
) -> HandlerResult {
    bot.send_with_notification(
        msg.chat.id,
        config
            .metro_api
            .next_arrival_request(req)
            .await
            .map_or("error getting metro schedule data".to_string(), |resp| {
                resp.to_string()
            }),
        config
            .notification_service
            .has_notifications()
            .await
            .unwrap_or(false),
    )
    .await?;
    Ok(())
}

async fn spending_reset_endpoint(bot: Bot, msg: Message, config: Arc<Config>) -> HandlerResult {
    bot.send_with_notification(
        msg.chat.id,
        config
            .spending_api
            .spending_reset_request()
            .await
            .map_or("error calling spending api".to_string(), |resp| {
                resp.to_string()
            }),
        config
            .notification_service
            .has_notifications()
            .await
            .unwrap_or(false),
    )
    .await?;
    Ok(())
}

async fn spending_total_endpoint(bot: Bot, msg: Message, config: Arc<Config>) -> HandlerResult {
    bot.send_with_notification(
        msg.chat.id,
        config
            .spending_api
            .spending_total_request()
            .await
            .map_or("error calling spending api".to_string(), |resp| {
                resp.to_string()
            }),
        config
            .notification_service
            .has_notifications()
            .await
            .unwrap_or(false),
    )
    .await?;
    Ok(())
}

async fn spending_endpoint(
    bot: Bot,
    msg: Message,
    req: SpentRequest,
    config: Arc<Config>,
) -> HandlerResult {
    bot.send_with_notification(
        msg.chat.id,
        config
            .spending_api
            .spending_request(req)
            .await
            .map_or("error calling spending api".to_string(), |resp| {
                resp.to_string()
            }),
        config
            .notification_service
            .has_notifications()
            .await
            .unwrap_or(false),
    )
    .await?;
    Ok(())
}

async fn budget_endpoint(
    bot: Bot,
    msg: Message,
    req: SpentRequest,
    config: Arc<Config>,
) -> HandlerResult {
    bot.send_with_notification(
        msg.chat.id,
        config
            .spending_api
            .budget_set_request(req)
            .await
            .map_or("error calling spending api".to_string(), |resp| {
                resp.to_string()
            }),
        config
            .notification_service
            .has_notifications()
            .await
            .unwrap_or(false),
    )
    .await?;
    Ok(())
}

async fn get_notifications(config: Arc<Config>) -> String {
    config
        .notification_service
        .read_notifications()
        .await
        .unwrap_or("error retrieving notifications".to_string())
}

async fn clear_notifications(config: Arc<Config>) -> String {
    match config.notification_service.clear_notifications().await {
        Ok(()) => "Notifications have been cleared.".to_string(),
        Err(e) => e.to_string(),
    }
}

async fn commands_handler(
    bot: Bot,
    msg: Message,
    cmd: Command,
    config: Arc<Config>,
) -> HandlerResult {
    bot.send_with_notification(
        msg.chat.id,
        match cmd {
            Command::Help => helpmsg(),
            Command::Thermostat => thermostat(config.clone()).await,
            Command::News => get_news(config.clone()).await,
            Command::Notifications => get_notifications(config.clone()).await,
            Command::ClearNotifications => clear_notifications(config.clone()).await,
        },
        config
            .notification_service
            .has_notifications()
            .await
            .unwrap_or(false),
    )
    .await?;
    Ok(())
}
