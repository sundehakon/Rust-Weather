use std::env;
use std::io::{self, Write};
use reqwest;
use dotenv::dotenv;
use serde::Deserialize;

#[derive(Deserialize)]
struct WeatherResponse {
    main: Main,
}

#[derive(Deserialize)]
struct Main {
    temp: f64,
}

pub fn get_weather(city_name: &str) -> Result<WeatherResponse, Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_key = env::var("APIKEY").map_err(|_| {
        "Environment variable APIKEY not set. Please set it to your OpenWeatherMap API key."
    })?;

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city_name, api_key
    );

    let response = reqwest::blocking::get(&url)?.json::<WeatherResponse>()?;
    Ok(response)
}

pub fn main() {
    print!("Enter the city name: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut city_name = String::new();
    io::stdin().read_line(&mut city_name).expect("Failed to read line");
    let city_name = city_name.trim();

    match get_weather(city_name) {
        Ok(response) => {
            println!("Temperature: {:.2}°C", response.main.temp); 
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
