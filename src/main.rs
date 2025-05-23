use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct Weather {
    current: Current,
}

#[derive(Debug, Deserialize)]
struct Current {
    temperature_2m: f64,
    relative_humidity_2m: i32,
    apparent_temperature: f64,
    precipitation: f64,
    wind_speed_10m: f64,
}

// fetch weather data from api
async fn fetch_weather(lat: f64, long: f64) -> Result<Weather, Box<dyn Error>> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,relative_humidity_2m,apparent_temperature,precipitation,wind_speed_10m",
        lat, long
    );

    // get the response
    let resp = reqwest::get(&url).await?;
    let data = resp.json::<Weather>().await?;
    Ok(data)
}

// show weather info in a nice format
fn show_weather(weather: &Weather) {
    println!("\nHere's your weather update:");
    println!("------------------------");
    println!("Temperature: {}°C", weather.current.temperature_2m);
    println!("Feels like: {}°C", weather.current.apparent_temperature);
    println!("Humidity: {}%", weather.current.relative_humidity_2m);
    println!("Rain: {}mm", weather.current.precipitation);
    println!("Wind: {} km/h", weather.current.wind_speed_10m);
    println!("------------------------");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // nyc coordinates
    let lat = 40.7128;
    let long = -74.0060;

    println!("Getting weather for New York City...");
    
    match fetch_weather(lat, long).await {
        Ok(weather) => show_weather(&weather),
        Err(e) => println!("Oops! Couldn't get weather data: {}", e),
    }

    Ok(())
}