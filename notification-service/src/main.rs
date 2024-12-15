use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Duration, Utc};
use clap::{arg, command};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use tokio::time::{interval, Duration as TokioDuration};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
struct Notification {
    id: String,
    message: String,
    created_at: String,
}

#[derive(Clone)]
struct AppState {
    notifications: Arc<RwLock<Vec<Notification>>>,
}

async fn create_notification(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<String>,
) -> Json<Notification> {
    let notification = Notification {
        id: Uuid::new_v4().to_string(),
        message: payload,
        created_at: Utc::now().to_string(),
    };

    state
        .notifications
        .write()
        .unwrap()
        .push(notification.clone());

    Json(notification)
}

async fn get_notifications(State(state): State<Arc<AppState>>) -> Json<Vec<Notification>> {
    let notifications = state
        .notifications
        .read()
        .expect("failed to obtain rwlock for notifications")
        .clone();
    Json(notifications)
}

async fn cleanup_old_notifications(state: Arc<AppState>) {
    let mut interval = interval(TokioDuration::from_secs(3600)); // Run every hour
    loop {
        interval.tick().await;
        let week_ago = Utc::now() - Duration::weeks(1);
        state
            .notifications
            .write()
            .expect("failed to write to rwlock")
            .retain(|n| n.created_at.parse::<DateTime<Utc>>().unwrap_or(Utc::now()) > week_ago);
    }
}

#[tokio::main]
async fn main() {
    let cmd = command!()
        .arg(arg!( -p --port [port] "port number for webserver").required(false))
        .get_matches();
    let default_port = "8005".to_string();
    let port = cmd.get_one::<String>("port").unwrap_or(&default_port);

    let state = Arc::new(AppState {
        notifications: Arc::new(RwLock::new(Vec::new())),
    });

    let cleanup_state = state.clone();

    tokio::spawn(async move {
        cleanup_old_notifications(cleanup_state).await;
    });

    let app = Router::new()
        .route("/notifications", post(create_notification))
        .route("/notifications", get(get_notifications))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .expect("Failed to start webserver. Port already in use?");
    axum::serve(listener, app).await.unwrap();
}
