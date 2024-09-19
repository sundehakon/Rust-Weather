pub fn get_weather(city_name: String) {
    let api_key = std::env::var("APIKEY").expect(
        "Environment variable APIKEY not set. Please set it to your OpenWeatherMap API key.",
    );

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}"
    )
}