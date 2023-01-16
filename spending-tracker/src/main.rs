#[macro_use]
extern crate rust_embed;

use actix_cors::Cors;
use actix_web::{get, post, web, web::JsonBody::Body, App, HttpResponse, HttpServer};
use actix_web_prom::PrometheusMetricsBuilder;
use chrono::Local;
use mime_guess::from_path;
use rusty_money::{iso, Money};
use spending_tracker::{Category, SpentRequest, SpentResponse, SpentTotalResponse, Transaction};
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};


#[derive(RustEmbed)]
#[folder = "public/"]
struct Asset;

struct AppState<'a> {
    state: Arc<RwLock<StateTotal<'a>>>,
}

struct StateTotal<'a> {
    budget: Money<'a, iso::Currency>,
    total: Money<'a, iso::Currency>,
    transactions: Vec<Transaction>,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let state = Arc::new(RwLock::new(StateTotal {
        budget: Money::from_major(500, iso::USD),
        total: Money::from_minor(0, iso::USD),
        transactions: Vec::new(),
    }));

    HttpServer::new(move || {
        App::new()
            .app_data(AppState {
                state: state.to_owned(),
            })
            .wrap(
                Cors::default()
                    .allowed_origin("localhost")
                    .allowed_methods(vec!["GET", "POST"]),
            )
            .wrap(prometheus.clone())
            .service(
                web::resource("/spent")
                    .route(web::post().to(spent))
                    .route(web::get().to(spent_total)),
            )
            .service(reset)
            .service(set_budget)
            .service(index)
            .service(dist)
    })
    .bind("0.0.0.0:8001")?
    .run()
    .await
}

async fn spent(state: web::Data<AppState<'_>>, req: web::Json<SpentRequest>) -> HttpResponse {
    let spent = req.into_inner();
    let add = Money::from_minor((spent.amount * 100.0).round() as i64, iso::USD);
    match state.state.write() {
        Ok(mut state) => {
            state.total += add.clone();
            state.transactions.push(Transaction {
                amount: add.to_string(),
                category: spent.category.unwrap_or(Category::Other).to_string(),
                time: Local::now().to_string(),
            });
            match serde_json::to_string(&SpentResponse {
                total: (state.budget.clone() - state.total.clone()).to_string(),
            }) {
                Ok(s) => HttpResponse::Ok().content_type("application/json").body(s),
                Err(_) => HttpResponse::InternalServerError().into(),
            }
        }
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

async fn spent_total(req: web::Data<AppState<'_>>) -> HttpResponse {
    match req.state.read() {
        Ok(i) => match serde_json::to_string(&SpentTotalResponse {
            budget: i.budget.clone().to_string(),
            total: i.total.clone().to_string(),
            transactions: i.transactions.clone(),
        }) {
            Ok(s) => HttpResponse::Ok().content_type("application/json").body(s),
            Err(_) => HttpResponse::InternalServerError().into(),
        },
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[get("/reset")]
async fn reset(req: web::Data<AppState<'_>>) -> HttpResponse {
    match req.state.write() {
        Ok(mut i) => {
            i.budget = Money::from_major(500, iso::USD);
            i.total = Money::from_minor(0, iso::USD);
            i.transactions = Vec::new();
            match serde_json::to_string(&SpentTotalResponse {
                budget: i.budget.clone().to_string(),
                total: i.total.clone().to_string(),
                transactions: Vec::new(),
            }) {
                Ok(s) => HttpResponse::Ok().content_type("application/json").body(s),
                Err(_) => HttpResponse::InternalServerError().into(),
            }
        }
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[post("/budget")]
async fn set_budget(state: web::Data<AppState<'_>>, req: web::Json<SpentRequest>) -> HttpResponse {
    match state.state.write() {
        Ok(mut i) => {
            i.budget = Money::from_minor((req.amount * 100.0).round() as i64, iso::USD);
            match serde_json::to_string(&SpentTotalResponse {
                budget: i.budget.clone().to_string(),
                total: i.total.clone().to_string(),
                transactions: i.transactions.clone(),
            }) {
                Ok(s) => HttpResponse::Ok().content_type("application/json").body(s),
                Err(_) => HttpResponse::InternalServerError().into(),
            }
        }
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

fn handle_embedded_file(path: &str) -> HttpResponse {
    match Asset::get(path) {
        Some(content) => {
            let body: Body = match content {
                Cow::Borrowed(bytes) => bytes.into(),
                Cow::Owned(bytes) => bytes.into(),
            };
            HttpResponse::Ok()
                .content_type(from_path(path).first_or_octet_stream().to_string())
                .body(body)
        }
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

#[get("/")]
async fn index(_req: web::Data<AppState<'_>>) -> HttpResponse {
    handle_embedded_file("index.html")
}

#[get("/dist/{_:.*}")]
async fn dist(path: web::Path<String>) -> HttpResponse {
    handle_embedded_file(&path.into_inner())
}

//
// pub struct StaticFile<T>(pub T);
//
// impl<T> IntoResponse for StaticFile<T>
//     where
//         T: Into<String>,
// {
//     fn into_response(self) -> Response {
//         let path = self.0.into();
//
//         match Asset::get(path.as_str()) {
//             Some(content) => {
//                 let body = boxed(Full::from(content.data));
//                 let mime = mime_guess::from_path(path).first_or_octet_stream();
//                 Response::builder().header(header::CONTENT_TYPE, mime.as_ref()).body(body).unwrap()
//             }
//             None => Response::builder().status(StatusCode::NOT_FOUND).body(boxed(Full::from("404"))).unwrap(),
//         }
//     }
// }