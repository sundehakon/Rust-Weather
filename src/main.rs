use std::env;
use reqwest;
use dotenv::dotenv;

pub fn get_weather(city_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_key = env::var("APIKEY").map_err(|_| {
        "Environment variable APIKEY not set. Please set it to your OpenWeatherMap API key."
    })?;

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}",
        city_name, api_key
    );

    let response = reqwest::blocking::get(&url)?.text()?;
    Ok(response)
}

pub fn main() {
    let usage = format!("Usage: {} [city_name]", env::args().next().unwrap_or_else(|| "program".to_string()));

    let city_name = env::args()
        .nth(1)
        .expect(&usage);

    match get_weather(&city_name) {
        Ok(body) => println!("{}", body),
        Err(e) => eprintln!("Error: {}", e),
    }
}
