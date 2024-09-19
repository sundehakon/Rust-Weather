use std::env;
use std::io::{self, Write};
use reqwest;
use dotenv::dotenv;
use serde::Deserialize;

#[derive(Deserialize)]
struct WeatherResponse {
    main: Main,
    weather: Vec<Weather>,
}

#[derive(Deserialize)]
struct Main {
    temp: f64,
}

#[derive(Deserialize)]
struct Weather {
    main: String,
    description: String,
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

pub fn format_weather_message(weather: &Weather, temp: f64) -> String {
    let description = weather.description.to_lowercase();
    let description = description.char_indices().next()
        .map(|(i, c)| description[..i].to_string() + &c.to_uppercase().to_string() + &description[i+1..])
        .unwrap_or(description);

    match weather.main.as_str() {
        "Clear" => format!("It is a clear day with a temperature of {:.2}°C. {}", temp, description),
        "Clouds" => format!("It is cloudy with a temperature of {:.2}°C. {}", temp, description),
        "Rain" => format!("It is rainy with a temperature of {:.2}°C. {}", temp, description),
        "Snow" => format!("It is snowing with a temperature of {:.2}°C. {}", temp, description),
        _ => format!("The weather is {} with a temperature of {:.2}°C. {}", weather.main, temp, description),
    }
}

pub fn main() {
    print!("Enter the city name: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut city_name = String::new();
    io::stdin().read_line(&mut city_name).expect("Failed to read line");
    let city_name = city_name.trim();

    match get_weather(city_name) {
        Ok(response) => {
            if let Some(weather) = response.weather.get(0) {
                let message = format_weather_message(weather, response.main.temp);
                println!("{}", message);
            } else {
                println!("Weather data not found.");
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
