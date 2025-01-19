use notification_service::NotificationsResponse;
use std::error::Error;
#[derive(Debug, Clone)]
pub(crate) struct NotificationService {
    pub(crate) url: String,
    pub(crate) unread_url: String,
}

impl Default for NotificationService {
    fn default() -> Self {
        let base_url = "http://localhost:8002";
        Self {
            url: format!("{base_url}/notifications"),
            unread_url: format!("{base_url}/unread"),
        }
    }
}

impl NotificationService {
    pub(crate) async fn write_notification(&self, payload: String) -> Result<(), reqwest::Error> {
        let client = reqwest::Client::new();
        client.post(&self.url).body(payload).send().await?;

        Ok(())
    }

    pub(crate) async fn read_notifications(&self) -> Result<String, reqwest::Error> {
        let res = reqwest::get(&self.url)
            .await?
            .json::<NotificationsResponse>()
            .await?;

        Ok(format!("{}", res))
    }

    pub(crate) async fn clear_notifications(&self) -> Result<(), reqwest::Error> {
        let client = reqwest::Client::new();
        client.delete(&self.url).send().await?;
        Ok(())
    }

    pub(crate) async fn has_notifications(&self) -> Result<bool, Box<dyn Error>> {
        reqwest::get(&self.unread_url)
            .await?
            .text()
            .await?
            .trim()
            .parse::<bool>()
            .map_err(From::from)
    }
}
