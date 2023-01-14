#[macro_use]
extern crate rust_embed;

use actix_web::{post, web, App, HttpResponse, HttpServer};
use actix_web_prom::PrometheusMetricsBuilder;
use anyhow::{bail, Result};
use chrono::{DateTime, Datelike, Local, Timelike, Weekday};
use clap::{App as ClApp, Arg};
use csv::Reader;
use metro_schedule::{
    Direction, NextArrivalRequest, NextArrivalResponse, Station, StationTimeSlice,
};
use std::collections::HashMap;

#[derive(RustEmbed)]
#[folder = "data/"]
struct Asset;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = ClApp::new("metro-schedule-api")
        .arg(Arg::with_name("port").help("port number for webserver"))
        .get_matches();
    let port = args.value_of("port").unwrap_or("8000");
    println!("app starting on port {}", &port);
    let prometheus = PrometheusMetricsBuilder::new("metro")
        .endpoint("/metrics")
        .const_labels(HashMap::new())
        .build()
        .unwrap();
    HttpServer::new(move || App::new().wrap(prometheus.clone()).service(next_arrival))
        .bind(format!("0.0.0.0:{}", port))?
        .run()
        .await
}

#[post("/next-arrival")]
async fn next_arrival(req: web::Json<NextArrivalRequest>) -> HttpResponse {
    let input = req.into_inner();
    let t = Local::now();
    let data = parse_request_pick_file(t, &input.direction);
    match Asset::get(&data) {
        Some(file_contents) => match search_csv(&file_contents, &input.station, t) {
            Ok(s) => match serde_json::to_string(&NextArrivalResponse {
                station: input.station,
                direction: input.direction,
                line: s.1,
                time: s.0,
            }) {
                Ok(s) => HttpResponse::Ok().content_type("application/json").body(s),
                Err(_) => HttpResponse::InternalServerError().into(),
            },
            Err(_) => HttpResponse::InternalServerError().into(),
        },
        None => HttpResponse::InternalServerError().into(),
    }
}

fn parse_request_pick_file(t: DateTime<Local>, direction: &Direction) -> String {
    format!(
        "{}bound-{}-schedule.csv",
        direction.to_string().to_lowercase(),
        match t.weekday() {
            Weekday::Sat => "saturday",
            Weekday::Sun => "sunday",
            _ => "weekday",
        }
    )
}

macro_rules! search_station {
    ($s:ident, $reader:expr, $t:expr) => {
        for result in $reader.deserialize() {
            let record: StationTimeSlice = result?;
            if let Some(s) = record.$s {
                if schedule_time_is_later_than_now($t, s.clone()) {
                    return Ok(line_info(s));
                }
            }
        }
        bail!("failed to find a time from schedule data")
    };
}

fn search_csv(
    file_contents: &[u8],
    station: &Station,
    t: DateTime<Local>,
) -> Result<(String, String)> {
    let mut reader = Reader::from_reader(file_contents);
    match station {
        Station::LambertT1 => {
            search_station!(lambert_t1, reader, t);
        }
        Station::LambertT2 => {
            search_station!(lambert_t2, reader, t);
        }
        Station::NorthHanley => {
            search_station!(north_hanley, reader, t);
        }
        Station::UMSLNorth => {
            search_station!(umsl_north, reader, t);
        }
        Station::UMSLSouth => {
            search_station!(umsl_south, reader, t);
        }
        Station::RockRoad => {
            search_station!(rock_road, reader, t);
        }
        Station::Wellston => {
            search_station!(wellston, reader, t);
        }
        Station::DelmarLoop => {
            search_station!(delmar_loop, reader, t);
        }
        Station::Shrewsbury => {
            search_station!(shrewsbury, reader, t);
        }
        Station::Sunnen => {
            search_station!(sunnen, reader, t);
        }
        Station::MaplewoodManchester => {
            search_station!(maplewood_manchester, reader, t);
        }
        Station::Brentwood => {
            search_station!(brentwood, reader, t);
        }
        Station::RichmondHeights => {
            search_station!(richmond_heights, reader, t);
        }
        Station::Clayton => {
            search_station!(clayton, reader, t);
        }
        Station::Forsyth => {
            search_station!(forsyth, reader, t);
        }
        Station::UCity => {
            search_station!(u_city, reader, t);
        }
        Station::Skinker => {
            search_station!(skinker, reader, t);
        }
        Station::ForestPark => {
            search_station!(forest_park, reader, t);
        }
        Station::CWE => {
            search_station!(cwe, reader, t);
        }
        Station::Cortex => {
            search_station!(cortex, reader, t);
        }
        Station::Grand => {
            search_station!(grand, reader, t);
        }
        Station::Union => {
            search_station!(union, reader, t);
        }
        Station::CivicCenter => {
            search_station!(civic_center, reader, t);
        }
        Station::Stadium => {
            search_station!(stadium, reader, t);
        }
        Station::EighthPine => {
            search_station!(eight_pine, reader, t);
        }
        Station::ConventionCenter => {
            search_station!(convention_center, reader, t);
        }
        Station::LacledesLanding => {
            search_station!(lacledes_landing, reader, t);
        }
        Station::EastRiverfront => {
            search_station!(east_riverfront, reader, t);
        }
        Station::FifthMissouri => {
            search_station!(fifth_missouri, reader, t);
        }
        Station::EmersonPark => {
            search_station!(emerson_park, reader, t);
        }
        Station::JJK => {
            search_station!(jjk, reader, t);
        }
        Station::Washington => {
            search_station!(washington, reader, t);
        }
        Station::FairviewHeights => {
            search_station!(fairview_heights, reader, t);
        }
        Station::MemorialHospital => {
            search_station!(memorial_hospital, reader, t);
        }
        Station::Swansea => {
            search_station!(swansea, reader, t);
        }
        Station::Belleville => {
            search_station!(belleville, reader, t);
        }
        Station::College => {
            search_station!(college, reader, t);
        }
        Station::ShilohScott => {
            search_station!(shiloh_scott, reader, t);
        }
    }
}

fn schedule_time_is_later_than_now(t: DateTime<Local>, mut s: String) -> bool {
    let _ = s.pop(); //remove line type
    let is_pm = s.pop().map_or(false, |c| c.to_string().eq("P"));
    let x: Vec<&str> = s.split(':').collect();
    let hh: u32 = x[0].parse::<u32>().unwrap_or_default();
    t.le(&Local::now()
        .with_hour(if is_pm { ((hh % 12) + 12) % 24 } else { hh })
        .expect("invalid time")
        .with_minute(x[1].parse::<u32>().unwrap_or_default())
        .expect("invalid time"))
}

fn line_info(mut s: String) -> (String, String) {
    let line = match s.pop() {
        Some(c) => match c {
            'R' => "red",
            'B' => "blue",
            _ => "",
        },
        None => "",
    };
    (s, line.to_string())
}
