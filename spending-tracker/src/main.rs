// #[macro_use]
// extern crate rust_embed;

use chrono::Local;
// use mime_guess::from_path;
use rusty_money::{iso, Money};
use spending_tracker::{Category, SpentRequest, SpentResponse, SpentTotalResponse, Transaction};
use std::borrow::Cow;
use std::sync::{Arc, RwLock};
use axum::{
    extract::MatchedPath,
    http::Request,
    response::IntoResponse,
    routing::get,
    Router,
};
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};
use std::{
    future::ready,
    net::SocketAddr,
};
use std::time::Instant;
use axum_extra::middleware::{self, Next};
use spending_tracker::metrics::{setup_metrics_recorder, track_metrics};

// #[derive(RustEmbed)]
// #[folder = "public/"]
// struct Asset;

struct AppState<'a> {
    state: Arc<RwLock<StateTotal<'a>>>,
}

struct StateTotal<'a> {
    budget: Money<'a, iso::Currency>,
    total: Money<'a, iso::Currency>,
    transactions: Vec<Transaction>,
}

#[tokio::main]
async fn main() {
    let state = Arc::new(RwLock::new(StateTotal {
        budget: Money::from_major(500, iso::USD),
        total: Money::from_minor(0, iso::USD),
        transactions: Vec::new(),
    }));

    let recorder_handle = setup_metrics_recorder();

    let app = Router::new()
        .route("/fast", get(|| async {}))
        .route(
            "/slow",
            get(|| async {}),
        )
        .route("/metrics", get(move || ready(recorder_handle.render())))
        .route_layer(middleware::from_fn(track_metrics));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}



// async fn spent(state: web::Data<AppState<'_>>, req: web::Json<SpentRequest>) -> HttpResponse {
//     let spent = req.into_inner();
//     let add = Money::from_minor((spent.amount * 100.0).round() as i64, iso::USD);
//     match state.state.write() {
//         Ok(mut state) => {
//             state.total += add.clone();
//             state.transactions.push(Transaction {
//                 amount: add.to_string(),
//                 category: spent.category.unwrap_or(Category::Other).to_string(),
//                 time: Local::now().to_string(),
//             });
//             match serde_json::to_string(&SpentResponse {
//                 total: (state.budget.clone() - state.total.clone()).to_string(),
//             }) {
//                 Ok(s) => HttpResponse::Ok().content_type("application/json").body(s),
//                 Err(_) => HttpResponse::InternalServerError().into(),
//             }
//         }
//         Err(_) => HttpResponse::InternalServerError().into(),
//     }
// }
//
// async fn spent_total(req: web::Data<AppState<'_>>) -> HttpResponse {
//     match req.state.read() {
//         Ok(i) => match serde_json::to_string(&SpentTotalResponse {
//             budget: i.budget.clone().to_string(),
//             total: i.total.clone().to_string(),
//             transactions: i.transactions.clone(),
//         }) {
//             Ok(s) => HttpResponse::Ok().content_type("application/json").body(s),
//             Err(_) => HttpResponse::InternalServerError().into(),
//         },
//         Err(_) => HttpResponse::InternalServerError().into(),
//     }
// }
//
// #[get("/reset")]
// async fn reset(req: web::Data<AppState<'_>>) -> HttpResponse {
//     match req.state.write() {
//         Ok(mut i) => {
//             i.budget = Money::from_major(500, iso::USD);
//             i.total = Money::from_minor(0, iso::USD);
//             i.transactions = Vec::new();
//             match serde_json::to_string(&SpentTotalResponse {
//                 budget: i.budget.clone().to_string(),
//                 total: i.total.clone().to_string(),
//                 transactions: Vec::new(),
//             }) {
//                 Ok(s) => HttpResponse::Ok().content_type("application/json").body(s),
//                 Err(_) => HttpResponse::InternalServerError().into(),
//             }
//         }
//         Err(_) => HttpResponse::InternalServerError().into(),
//     }
// }
//
// #[post("/budget")]
// async fn set_budget(state: web::Data<AppState<'_>>, req: web::Json<SpentRequest>) -> HttpResponse {
//     match state.state.write() {
//         Ok(mut i) => {
//             i.budget = Money::from_minor((req.amount * 100.0).round() as i64, iso::USD);
//             match serde_json::to_string(&SpentTotalResponse {
//                 budget: i.budget.clone().to_string(),
//                 total: i.total.clone().to_string(),
//                 transactions: i.transactions.clone(),
//             }) {
//                 Ok(s) => HttpResponse::Ok().content_type("application/json").body(s),
//                 Err(_) => HttpResponse::InternalServerError().into(),
//             }
//         }
//         Err(_) => HttpResponse::InternalServerError().into(),
//     }
// }
//
// fn handle_embedded_file(path: &str) -> HttpResponse {
//     match Asset::get(path) {
//         Some(content) => {
//             let body: Body = match content {
//                 Cow::Borrowed(bytes) => bytes.into(),
//                 Cow::Owned(bytes) => bytes.into(),
//             };
//             HttpResponse::Ok()
//                 .content_type(from_path(path).first_or_octet_stream().to_string())
//                 .body(body)
//         }
//         None => HttpResponse::NotFound().body("404 Not Found"),
//     }
// }
//
// #[get("/")]
// async fn index(_req: web::Data<AppState<'_>>) -> HttpResponse {
//     handle_embedded_file("index.html")
// }
//
// #[get("/dist/{_:.*}")]
// async fn dist(path: web::Path<String>) -> HttpResponse {
//     handle_embedded_file(&path.0)
// }

