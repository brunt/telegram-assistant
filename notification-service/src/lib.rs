use chrono::{DateTime, Duration, Local, Utc};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt::Display;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct Notification {
    id: String,
    message: String,
    created_at: String,
}
impl Notification {
    pub fn new(message: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            message,
            created_at: Utc::now().to_string(),
        }
    }
}

impl Default for Notification {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            message: "Notification Serviced started.".to_string(),
            created_at: Utc::now().to_string(),
        }
    }
}

impl Display for Notification {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let local_time: DateTime<Local> = self
            .created_at
            .parse::<DateTime<Local>>()
            .map_err(|_| std::fmt::Error)?;
        write!(f, "{}\n{}", self.message, local_time)
    }
}

impl PartialEq<DateTime<Utc>> for Notification {
    fn eq(&self, other: &DateTime<Utc>) -> bool {
        self.created_at
            .parse::<DateTime<Utc>>()
            .unwrap_or(Utc::now() - Duration::weeks(2))
            == *other
    }
}

impl PartialOrd<DateTime<Utc>> for Notification {
    fn partial_cmp(&self, other: &DateTime<Utc>) -> Option<Ordering> {
        self.created_at
            .parse::<DateTime<Utc>>()
            .unwrap_or(Utc::now() - Duration::weeks(2))
            .partial_cmp(other)
    }
}

#[derive(Serialize, Deserialize)]
pub struct NotificationsResponse(pub Vec<Notification>);

impl Display for NotificationsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Ok(for noti in self.0.iter().rev() {
            write!(f, "• {}\n", noti)?;
        })
    }
}
