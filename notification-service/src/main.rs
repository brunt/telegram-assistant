use axum::{
    extract::State,
    routing::{delete, get, post},
    Json, Router,
};
use chrono::{Duration, Utc};
use clap::{arg, command};
use notification_service::Notification;
use notification_service::NotificationsResponse;
use std::sync::{Arc, RwLock};
use tokio::time::{interval, Duration as TokioDuration};

#[derive(Clone)]
struct AppState {
    notifications: Arc<RwLock<Vec<Notification>>>,
    has_unread: Arc<RwLock<bool>>,
}

async fn create_notification(
    State(state): State<Arc<AppState>>,
    payload: String,
) -> Json<Notification> {
    let notification = Notification::new(payload);

    state
        .notifications
        .write()
        .expect("failed to obtain write lock during create")
        .push(notification.clone());

    *state
        .has_unread
        .write()
        .expect("failed to obtain write lock during create") = true;

    Json(notification)
}

async fn get_notifications(State(state): State<Arc<AppState>>) -> Json<NotificationsResponse> {
    let notifications = state
        .notifications
        .read()
        .expect("failed to obtain read lock when reading notifications")
        .clone();

    *state
        .has_unread
        .write()
        .expect("failed to obtain write lock during read") = false;

    Json(NotificationsResponse(notifications))
}

async fn cleanup_old_notifications(state: Arc<AppState>) {
    let mut interval = interval(TokioDuration::from_secs(3600)); // Run every hour
    loop {
        interval.tick().await;
        let week_ago = Utc::now() - Duration::weeks(1);
        state
            .notifications
            .write()
            .expect("failed to write to get write lock during scheduled cleanup")
            .retain(|n| n > &week_ago);
    }
}

async fn clear_notifications(State(state): State<Arc<AppState>>) -> Json<Vec<Notification>> {
    state
        .notifications
        .write()
        .expect("failed to get write lock when clearing notifications")
        .clear();

    *state
        .has_unread
        .write()
        .expect("failed to obtain write lock during cleanup") = false;

    Json(
        state
            .notifications
            .read()
            .expect("failed to read notifications after clearing them")
            .clone(),
    )
}

async fn any_unread(State(state): State<Arc<AppState>>) -> Json<bool> {
    Json(*state.has_unread.read().expect("failed to read app state"))
}

#[tokio::main]
async fn main() {
    let cmd = command!()
        .arg(arg!( -p --port [port] "port number for webserver").required(false))
        .get_matches();
    let default_port = "8002".to_string();
    let port = cmd.get_one::<String>("port").unwrap_or(&default_port);

    let state = Arc::new(AppState {
        notifications: Arc::new(RwLock::new(vec![Notification::default()])),
        has_unread: Arc::new(RwLock::new(true)),
    });

    let cleanup_state = state.clone();

    tokio::spawn(async move {
        cleanup_old_notifications(cleanup_state).await;
    });

    let app = Router::new()
        .route("/notifications", post(create_notification))
        .route("/notifications", get(get_notifications))
        .route("/notifications", delete(clear_notifications))
        .route("/unread", get(any_unread))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .expect("Failed to start webserver. Port already in use?");
    axum::serve(listener, app)
        .await
        .expect("failed to start webserver");
}
