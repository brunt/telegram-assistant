use chrono::{DateTime, Local, TimeZone, Timelike, Utc};
use forecast::{ApiClient, ApiResponse, ExcludeBlock, ForecastRequestBuilder};
use reqwest::Client;

pub(crate) fn help_weather() -> &'static str {
    r#"Send location to receive weather information.
    Powered by Dark Sky
    https://darksky.net/poweredby/"#
}

pub(crate) async fn weather_request(token: &str, lat: f64, long: f64) -> String {
    let req = Client::new();
    let call = ApiClient::new(&req);

    let mut blocks: Vec<ExcludeBlock> = vec![ExcludeBlock::Minutely, ExcludeBlock::Flags];

    let forecast_builder = ForecastRequestBuilder::new(token, lat, long);
    let forecast_req = forecast_builder.exclude_blocks(&mut blocks).build();

    match call.get_forecast(forecast_req.clone()).await {
        Err(e) => format!("forecast error: {:?}", e),
        Ok(resp) => {
            let resp: ApiResponse = resp.json().await.unwrap();
            let mut s = String::with_capacity(80); //guessing at capacity
            if let Some(alerts) = resp.alerts {
                match alerts.len() {
                    x if x > 1 => s.push_str(&format!("{} Alerts:\n", x)),
                    x => s.push_str(&format!("{} Alert:\n", x)),
                }
                for a in alerts {
                    s.push_str(&format!("{}\n{}\n", a.title, a.description));
                }
            }

            if let Some(data) = resp.currently {
                if let Some(current) = data.summary {
                    s.push_str(&format!("Curently: {}\n", current))
                }
                if let Some(temp) = data.temperature {
                    s.push_str(&format!("Temp: {}\n", temp))
                }
                if let Some(gust) = data.wind_gust {
                    s.push_str(&format!("Wind gust: {}\n", gust))
                }
            }
            if let Some(data) = resp.hourly {
                if let Some(summary) = data.summary {
                    s.push_str(&format!("Today: {}\n", summary))
                }
            }
            if let Some(data) = resp.daily {
                if let Some(high) = data.data[0].temperature_high {
                    s.push_str(&format!("High: {}\n", high));
                }
                if let Some(low) = data.data[0].temperature_low {
                    s.push_str(&format!("Low: {}\n", low));
                }
                if let Some(sunrise) = data.data[0].sunrise_time {
                    let time = unix_to_local(sunrise);
                    if time.minute() < 10 {
                        s.push_str(&format!(
                            "Sunrise: {}:0{} AM\n",
                            time.hour() % 12,
                            time.minute()
                        ));
                    } else {
                        s.push_str(&format!(
                            "Sunrise: {}:{} AM\n",
                            time.hour() % 12,
                            time.minute()
                        ));
                    }
                }
                if let Some(sunset) = data.data[0].sunset_time {
                    let time = unix_to_local(sunset);
                    if time.minute() < 10 {
                        s.push_str(&format!(
                            "Sunset: {}:0{} PM",
                            time.hour() % 12,
                            time.minute()
                        ));
                    } else {
                        s.push_str(&format!(
                            "Sunset: {}:{} PM",
                            time.hour() % 12,
                            time.minute()
                        ));
                    }
                }
            }
            s
        }
    }
}

fn unix_to_local(unix_time: u64) -> DateTime<Local> {
    let utc = Utc.timestamp(unix_time as i64, 0);
    utc.with_timezone(&Local)
}
