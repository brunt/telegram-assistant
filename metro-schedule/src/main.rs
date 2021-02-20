#[macro_use]
extern crate rust_embed;

use actix_web::{post, web, App, HttpResponse, HttpServer};
use actix_web_prom::PrometheusMetrics;
use anyhow::{bail, Result};
use chrono::{DateTime, Datelike, Local, Weekday};
use clap::{App as ClApp, Arg};
use csv::Reader;
use metro_schedule::{NextArrivalRequest, NextArrivalResponse, StationTimeSlice};
use std::cmp::Ordering;

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
    let prometheus = PrometheusMetrics::new("metro", Some("/metrics"), None);
    HttpServer::new(move || App::new().wrap(prometheus.clone()).service(next_arrival))
        .bind(format!("0.0.0.0:{}", port))?
        .run()
        .await
}

#[post("/next-arrival")]
async fn next_arrival(req: web::Json<NextArrivalRequest>) -> HttpResponse {
    let input = req.into_inner();
    let t = Local::now();
    match parse_request_pick_file(t, input.direction.as_str()) {
        Some(data) => match Asset::get(&data) {
            Some(file_contents) => {
                match search_csv(&file_contents, input.station.to_lowercase().as_str(), t) {
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
                }
            }
            None => HttpResponse::InternalServerError().into(),
        },
        None => HttpResponse::BadRequest()
            .reason("direction must be 'east' or 'west'")
            .finish(),
    }
}

fn parse_request_pick_file(t: DateTime<Local>, direction: &str) -> Option<String> {
    (direction.eq("east") || direction.eq("west")).then(|| {
        format!(
            "{}bound-{}-schedule.csv",
            direction,
            match t.weekday() {
                Weekday::Sat => "saturday",
                Weekday::Sun => "sunday",
                _ => "weekday",
            }
        )
    })
}

fn search_csv(file_contents: &[u8], station: &str, t: DateTime<Local>) -> Result<(String, String)> {
    let mut reader = Reader::from_reader(&file_contents[..]);
    match station {
        "lambert" => {
            for result in reader.deserialize() {
                let record: StationTimeSlice = result?;
                if let Some(s) = record.lambert_t1 {
                    if schedule_time_is_later_than_now(t, s.clone()) {
                        return Ok(line_info(s));
                    }
                }
            }
            bail!("failed to find a time from schedule data")
        }
        "lambert2" => {
            for result in reader.deserialize() {
                let record: StationTimeSlice = result.unwrap();
                if let Some(s) = record.lambert_t2 {
                    if schedule_time_is_later_than_now(t, s.clone()) {
                        return Ok(line_info(s));
                    }
                }
            }
            bail!("failed to find a time from schedule data")
        }
        "hanley" => {
            for result in reader.deserialize() {
                let record: StationTimeSlice = result.unwrap();
                if let Some(s) = record.north_hanley {
                    if schedule_time_is_later_than_now(t, s.clone()) {
                        return Ok(line_info(s));
                    }
                }
            }
            bail!("failed to find a time from schedule data")
        }
        "umsl north" | "umsl" => {
            for result in reader.deserialize() {
                let record: StationTimeSlice = result.unwrap();
                if let Some(s) = record.umsl_north {
                    if schedule_time_is_later_than_now(t, s.clone()) {
                        return Ok(line_info(s));
                    }
                }
            }
            bail!("failed to find a time from schedule data")
        }
        "umsl south" => {
            for result in reader.deserialize() {
                let record: StationTimeSlice = result.unwrap();
                if let Some(s) = record.umsl_south {
                    if schedule_time_is_later_than_now(t, s.clone()) {
                        return Ok(line_info(s));
                    }
                }
            }
            bail!("failed to find a time from schedule data")
        }
        "rock road" => {
            for result in reader.deserialize() {
                let record: StationTimeSlice = result.unwrap();
                if let Some(s) = record.rock_road {
                    if schedule_time_is_later_than_now(t, s.clone()) {
                        return Ok(line_info(s));
                    }
                }
            }
            bail!("failed to find a time from schedule data")
        }
        "wellston" => {
            for result in reader.deserialize() {
                let record: StationTimeSlice = result.unwrap();
                if let Some(s) = record.wellston {
                    if schedule_time_is_later_than_now(t, s.clone()) {
                        return Ok(line_info(s));
                    }
                }
            }
            bail!("failed to find a time from schedule data")
        }
        "delmar" => {
            for result in reader.deserialize() {
                let record: StationTimeSlice = result.unwrap();
                if let Some(s) = record.delmar_loop {
                    if schedule_time_is_later_than_now(t, s.clone()) {
                        return Ok(line_info(s));
                    }
                }
            }
            bail!("failed to find a time from schedule data")
        }
        "shrewsbury" => {
            for result in reader.deserialize() {
                let record: StationTimeSlice = result.unwrap();
                if let Some(s) = record.shrewsbury {
                    if schedule_time_is_later_than_now(t, s.clone()) {
                        return Ok(line_info(s));
                    }
                }
            }
            bail!("failed to find a time from schedule data")
        }
        "sunnen" => {
            for result in reader.deserialize() {
                let record: StationTimeSlice = result.unwrap();
                if let Some(s) = record.sunnen {
                    if schedule_time_is_later_than_now(t, s.clone()) {
                        return Ok(line_info(s));
                    }
                }
            }
            bail!("failed to find a time from schedule data")
        }
        "maplewood" => {
            for result in reader.deserialize() {
                let record: StationTimeSlice = result.unwrap();
                if let Some(s) = record.maplewood_manchester {
                    if schedule_time_is_later_than_now(t, s.clone()) {
                        return Ok(line_info(s));
                    }
                }
            }
            bail!("failed to find a time from schedule data")
        }
        "brentwood" => {
            for result in reader.deserialize() {
                let record: StationTimeSlice = result.unwrap();
                if let Some(s) = record.brentwood {
                    if schedule_time_is_later_than_now(t, s.clone()) {
                        return Ok(line_info(s));
                    }
                }
            }
            bail!("failed to find a time from schedule data")
        }
        "richmond" | "richmond heights" => {
            for result in reader.deserialize() {
                let record: StationTimeSlice = result.unwrap();
                if let Some(s) = record.richmond_heights {
                    if schedule_time_is_later_than_now(t, s.clone()) {
                        return Ok(line_info(s));
                    }
                }
            }
            bail!("failed to find a time from schedule data")
        }
        "clayton" => {
            for result in reader.deserialize() {
                let record: StationTimeSlice = result.unwrap();
                if let Some(s) = record.clayton {
                    if schedule_time_is_later_than_now(t, s.clone()) {
                        return Ok(line_info(s));
                    }
                }
            }
            bail!("failed to find a time from schedule data")
        }
        "forsyth" => {
            for result in reader.deserialize() {
                let record: StationTimeSlice = result.unwrap();
                if let Some(s) = record.forsyth {
                    if schedule_time_is_later_than_now(t, s.clone()) {
                        return Ok(line_info(s));
                    }
                }
            }
            bail!("failed to find a time from schedule data")
        }
        "u city" | "university city" => {
            for result in reader.deserialize() {
                let record: StationTimeSlice = result.unwrap();
                if let Some(s) = record.u_city {
                    if schedule_time_is_later_than_now(t, s.clone()) {
                        return Ok(line_info(s));
                    }
                }
            }
            bail!("failed to find a time from schedule data")
        }
        "skinker" => {
            for result in reader.deserialize() {
                let record: StationTimeSlice = result.unwrap();
                if let Some(s) = record.skinker {
                    if schedule_time_is_later_than_now(t, s.clone()) {
                        return Ok(line_info(s));
                    }
                }
            }
            bail!("failed to find a time from schedule data")
        }
        "forest park" => {
            for result in reader.deserialize() {
                let record: StationTimeSlice = result.unwrap();
                if let Some(s) = record.forest_park {
                    if schedule_time_is_later_than_now(t, s.clone()) {
                        return Ok(line_info(s));
                    }
                }
            }
            bail!("failed to find a time from schedule data")
        }
        "cwe" | "central west end" => {
            for result in reader.deserialize() {
                let record: StationTimeSlice = result.unwrap();
                if let Some(s) = record.cwe {
                    if schedule_time_is_later_than_now(t, s.clone()) {
                        return Ok(line_info(s));
                    }
                }
            }
            bail!("failed to find a time from schedule data")
        }
        "cortex" => {
            for result in reader.deserialize() {
                let record: StationTimeSlice = result.unwrap();
                if let Some(s) = record.cortex {
                    if schedule_time_is_later_than_now(t, s.clone()) {
                        return Ok(line_info(s));
                    }
                }
            }
            bail!("failed to find a time from schedule data")
        }
        "grand" => {
            for result in reader.deserialize() {
                let record: StationTimeSlice = result.unwrap();
                if let Some(s) = record.grand {
                    if schedule_time_is_later_than_now(t, s.clone()) {
                        return Ok(line_info(s));
                    }
                }
            }
            bail!("failed to find a time from schedule data")
        }
        "union" => {
            for result in reader.deserialize() {
                let record: StationTimeSlice = result.unwrap();
                if let Some(s) = record.union {
                    if schedule_time_is_later_than_now(t, s.clone()) {
                        return Ok(line_info(s));
                    }
                }
            }
            bail!("failed to find a time from schedule data")
        }
        "civic center" | "civic" => {
            for result in reader.deserialize() {
                let record: StationTimeSlice = result.unwrap();
                if let Some(s) = record.civic_center {
                    if schedule_time_is_later_than_now(t, s.clone()) {
                        return Ok(line_info(s));
                    }
                }
            }
            bail!("failed to find a time from schedule data")
        }
        "stadium" => {
            for result in reader.deserialize() {
                let record: StationTimeSlice = result.unwrap();
                if let Some(s) = record.stadium {
                    if schedule_time_is_later_than_now(t, s.clone()) {
                        return Ok(line_info(s));
                    }
                }
            }
            bail!("failed to find a time from schedule data")
        }
        "8th and pine" | "8th pine" => {
            for result in reader.deserialize() {
                let record: StationTimeSlice = result.unwrap();
                if let Some(s) = record.eight_pine {
                    if schedule_time_is_later_than_now(t, s.clone()) {
                        return Ok(line_info(s));
                    }
                }
            }
            bail!("failed to find a time from schedule data")
        }
        "convention center" | "convention" => {
            for result in reader.deserialize() {
                let record: StationTimeSlice = result.unwrap();
                if let Some(s) = record.convention_center {
                    if schedule_time_is_later_than_now(t, s.clone()) {
                        return Ok(line_info(s));
                    }
                }
            }
            bail!("failed to find a time from schedule data")
        }
        "lacledes" | "lacledes landing" => {
            for result in reader.deserialize() {
                let record: StationTimeSlice = result.unwrap();
                if let Some(s) = record.lacledes_landing {
                    if schedule_time_is_later_than_now(t, s.clone()) {
                        return Ok(line_info(s));
                    }
                }
            }
            bail!("failed to find a time from schedule data")
        }
        "riverfront" | "east riverfront" => {
            for result in reader.deserialize() {
                let record: StationTimeSlice = result.unwrap();
                if let Some(s) = record.east_riverfront {
                    if schedule_time_is_later_than_now(t, s.clone()) {
                        return Ok(line_info(s));
                    }
                }
            }
            bail!("failed to find a time from schedule data")
        }
        "fifth missouri" | "5th missouri" => {
            for result in reader.deserialize() {
                let record: StationTimeSlice = result.unwrap();
                if let Some(s) = record.fifth_missouri {
                    if schedule_time_is_later_than_now(t, s.clone()) {
                        return Ok(line_info(s));
                    }
                }
            }
            bail!("failed to find a time from schedule data")
        }
        "emerson" | "emerson park" => {
            for result in reader.deserialize() {
                let record: StationTimeSlice = result.unwrap();
                if let Some(s) = record.emerson_park {
                    if schedule_time_is_later_than_now(t, s.clone()) {
                        return Ok(line_info(s));
                    }
                }
            }
            bail!("failed to find a time from schedule data")
        }
        "jjk" | "jackie joiner" => {
            for result in reader.deserialize() {
                let record: StationTimeSlice = result.unwrap();
                if let Some(s) = record.jjk {
                    if schedule_time_is_later_than_now(t, s.clone()) {
                        return Ok(line_info(s));
                    }
                }
            }
            bail!("failed to find a time from schedule data")
        }
        "washington" => {
            for result in reader.deserialize() {
                let record: StationTimeSlice = result.unwrap();
                if let Some(s) = record.washington {
                    if schedule_time_is_later_than_now(t, s.clone()) {
                        return Ok(line_info(s));
                    }
                }
            }
            bail!("failed to find a time from schedule data")
        }
        "fvh" | "fairview heights" => {
            for result in reader.deserialize() {
                let record: StationTimeSlice = result.unwrap();
                if let Some(s) = record.fairview_heights {
                    if schedule_time_is_later_than_now(t, s.clone()) {
                        return Ok(line_info(s));
                    }
                }
            }
            bail!("failed to find a time from schedule data")
        }
        "memorial hospital" => {
            for result in reader.deserialize() {
                let record: StationTimeSlice = result.unwrap();
                if let Some(s) = record.memorial_hospital {
                    if schedule_time_is_later_than_now(t, s.clone()) {
                        return Ok(line_info(s));
                    }
                }
            }
            bail!("failed to find a time from schedule data")
        }
        "swansea" => {
            for result in reader.deserialize() {
                let record: StationTimeSlice = result.unwrap();
                if let Some(s) = record.swansea {
                    if schedule_time_is_later_than_now(t, s.clone()) {
                        return Ok(line_info(s));
                    }
                }
            }
            bail!("failed to find a time from schedule data")
        }
        "belleville" => {
            for result in reader.deserialize() {
                let record: StationTimeSlice = result.unwrap();
                if let Some(s) = record.belleville {
                    if schedule_time_is_later_than_now(t, s.clone()) {
                        return Ok(line_info(s));
                    }
                }
            }
            bail!("failed to find a time from schedule data")
        }
        "college" => {
            for result in reader.deserialize() {
                let record: StationTimeSlice = result.unwrap();
                if let Some(s) = record.college {
                    if schedule_time_is_later_than_now(t, s.clone()) {
                        return Ok(line_info(s));
                    }
                }
            }
            bail!("failed to find a time from schedule data")
        }
        "shiloh" | "shiloh scott" => {
            for result in reader.deserialize() {
                let record: StationTimeSlice = result.unwrap();
                if let Some(s) = record.shiloh_scott {
                    if schedule_time_is_later_than_now(t, s.clone()) {
                        return Ok(line_info(s));
                    }
                }
            }
            bail!("failed to find a time from schedule data")
        }
        _ => bail!("that station is not in the schedule"),
    }
}

fn schedule_time_is_later_than_now(t: DateTime<Local>, mut s: String) -> bool {
    let mut plus_twelve = false;
    let _ = s.pop(); //remove line type
    if s.pop().unwrap().to_string().eq("P") {
        plus_twelve = true;
    }
    let x: Vec<&str> = s.split(':').collect();
    let mut hh: u32 = x[0].parse::<u32>().unwrap_or_default();
    let mm: u32 = x[1].parse::<u32>().unwrap_or_default();
    if plus_twelve {
        hh = ((hh % 12) + 12) % 24;
    }
    match t.cmp(&Local::today().and_hms(hh, mm, 00)) {
        Ordering::Less => true,
        Ordering::Equal => true,
        Ordering::Greater => false,
    }
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
