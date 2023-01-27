use axum::body::{boxed, Full};
use axum::http::{header, HeaderValue, Uri};
use axum::response::Html;
use axum::{
    extract::{Json, State},
    http::{Method, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use chrono::Local;
use clap::{arg, command};
use rust_embed::RustEmbed;
use rusty_money::{iso, Money};
use spending_tracker::{Category, SpentRequest, SpentResponse, SpentTotalResponse, Transaction};
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use tower_http::cors::CorsLayer;

#[derive(RustEmbed)]
#[folder = "public/"]
struct Asset;

#[derive(Clone)]
struct AppState<'a> {
    state: Arc<RwLock<StateTotal<'a>>>,
}

struct StateTotal<'a> {
    budget: Money<'a, iso::Currency>,
    total: Money<'a, iso::Currency>,
    transactions: Vec<Transaction>,
}

impl AppState<'_> {
    fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(StateTotal {
                budget: Money::from_major(500, iso::USD),
                total: Money::from_minor(0, iso::USD),
                transactions: Vec::new(),
            })),
        }
    }
}

#[tokio::main]
async fn main() {
    let cmd = command!()
        .arg(arg!( -p --port [port] "port number for webserver").required(false))
        .get_matches();
    let port = cmd.get_one::<String>("port");
    let state = AppState::new();
    let app = Router::new()
        .route("/budget", post(set_budget))
        .route("/spent", post(spent).get(spent_total))
        .route("/reset", get(reset))
        .route("/dist/*file", get(static_handler))
        .route("/", get(index))
        .layer(
            CorsLayer::new()
                .allow_origin(
                    format!("http://localhost:{}", port.unwrap_or(&"8001".to_string()))
                        .parse::<HeaderValue>()
                        .unwrap(),
                )
                .allow_methods([Method::GET, Method::POST]),
        )
        .fallback_service(get(not_found))
        .with_state(state);

    let addr = SocketAddr::from((
        [0, 0, 0, 0],
        port.map_or(8001, |p| p.parse().unwrap_or(8001)),
    ));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("port already in use?");
}

async fn spent(State(state): State<AppState<'_>>, Json(req): Json<SpentRequest>) -> Response {
    if let Ok(mut state) = state.state.write() {
        let add = Money::from_minor((req.amount * 100.0).round() as i64, iso::USD);
        state.total += add.clone();
        state.transactions.push(Transaction {
            amount: add.to_string(),
            category: req.category.unwrap_or(Category::Other).to_string(),
            time: Local::now().to_string(),
        });
        return Json(SpentResponse {
            total: (state.budget.clone() - state.total.clone()).to_string(),
        })
        .into_response();
    }
    StatusCode::INTERNAL_SERVER_ERROR.into_response()
}

async fn spent_total(State(app_state): State<AppState<'_>>) -> Response {
    if let Ok(state) = app_state.state.read() {
        return Json(SpentTotalResponse {
            budget: state.budget.to_string(),
            total: state.total.to_string(),
            transactions: state.transactions.clone(),
        })
        .into_response();
    }
    StatusCode::INTERNAL_SERVER_ERROR.into_response()
}

async fn reset(State(app_state): State<AppState<'_>>) -> Response {
    if let Ok(mut state) = app_state.state.write() {
        state.budget = Money::from_major(500, iso::USD);
        state.total = Money::from_minor(0, iso::USD);
        state.transactions = Vec::new();
        return Json(SpentTotalResponse {
            budget: state.budget.clone().to_string(),
            total: state.total.clone().to_string(),
            transactions: Vec::new(),
        })
        .into_response();
    }
    StatusCode::INTERNAL_SERVER_ERROR.into_response()
}

async fn set_budget(
    State(app_state): State<AppState<'_>>,
    Json(req): Json<SpentRequest>,
) -> Response {
    if let Ok(mut state) = app_state.state.write() {
        state.budget = Money::from_minor((req.amount * 100.0).round() as i64, iso::USD);
        return Json(SpentTotalResponse {
            budget: state.budget.clone().to_string(),
            total: state.total.clone().to_string(),
            transactions: state.transactions.clone(),
        })
        .into_response();
    }
    StatusCode::INTERNAL_SERVER_ERROR.into_response()
}

async fn index() -> Response {
    static_handler("/index.html".parse::<Uri>().unwrap())
        .await
        .into_response()
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let mut path = uri.path().trim_start_matches('/').to_string();

    if path.starts_with("dist/") {
        path = path.replace("dist/", "");
    }

    StaticFile(path)
}

async fn not_found() -> Html<&'static str> {
    Html("<h1>404</h1><p>Not Found</p>")
}

pub struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> Response {
        let path = self.0.into();

        match Asset::get(path.as_str()) {
            Some(content) => {
                let body = boxed(Full::from(content.data));
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                Response::builder()
                    .header(header::CONTENT_TYPE, mime.as_ref())
                    .body(body)
                    .unwrap()
            }
            None => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(boxed(Full::from("404")))
                .unwrap(),
        }
    }
}
