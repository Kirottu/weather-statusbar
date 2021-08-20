use clap::{App, Arg};
use serde_json::Value;

#[tokio::main]
async fn main() {
    let matches = App::new("weather-statusbar")
        .about("Weather info for use in a statusbar")
        .arg(
            Arg::with_name("amount")
                .short("n")
                .long("amount")
                .help("Set the amount of weather data to display")
                .required(false)
                .default_value("3")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("time_offset")
                .short("t")
                .long("time_offset")
                .help("Set the timezone offset from UTC")
                .required(false)
                .default_value("0")
                .takes_value(true),
        )
        .get_matches();

    let client = reqwest::Client::builder()
        .user_agent("github.com/Kirottu/weather-statusbar kirottualt@gmail.com")
        .build()
        .unwrap();

    let weather_string = client
        .get("https://api.met.no/weatherapi/locationforecast/2.0/compact?lat=62.24147&lon=25.72088")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let weather_json: Value = serde_json::from_str(&weather_string).unwrap();

    let time_offset = matches
        .value_of("time_offset")
        .unwrap()
        .parse::<i32>()
        .unwrap();
    let weather_snapshot_amount = matches
        .value_of("amount")
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let temp_symbol = "🌡️";
    let wind_symbol = "💨";

    for i in 0..weather_snapshot_amount {
        let current_timeseries = &weather_json["properties"]["timeseries"][i];
        let time = current_timeseries["time"]
            .as_str()
            .unwrap()
            .split("T")
            .collect::<Vec<&str>>()[1]
            .split(":")
            .collect::<Vec<&str>>()[0]
            .parse::<i32>()
            .unwrap()
            + time_offset;
        let weather_symbol = match current_timeseries["data"]["next_1_hours"]["summary"]
            ["symbol_code"]
            .as_str()
            .unwrap()
        {
            "rain" => "🌧",
            "lightrain" => "🌦",
            "heavyrain" => "🌧",
            "cloudy" => "☁",
            "lightrainshowers_day" => "🌦",
            _ => "?",
        };

        print!(
            "{}:00: [{}{}m/s {} {}°C {} ] ",
            time,
            wind_symbol,
            current_timeseries["data"]["instant"]["details"]["wind_speed"],
            temp_symbol,
            current_timeseries["data"]["instant"]["details"]["air_temperature"],
            weather_symbol,
        );
    }
    println!("");
}
