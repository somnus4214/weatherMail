use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct QWeatherResponse {
    pub code: String,
    #[serde(rename = "updateTime")]
    pub update_time: Option<String>,
    pub hourly: Vec<HourlyWeather>,
}

#[derive(Debug, Deserialize)]
pub struct HourlyWeather {
    #[serde(rename = "fxTime")]
    pub fx_time: String,
    pub temp: String,
    pub text: String,
    #[serde(rename = "icon")]
    pub icon: Option<String>,
    #[serde(rename = "windDir")]
    pub wind_dir: Option<String>,
    #[serde(rename = "windScale")]
    pub wind_scale: Option<String>,
    #[serde(rename = "humidity")]
    pub humidity: Option<String>,
    #[serde(rename = "pop")]
    pub pop: Option<String>,
    #[serde(rename = "windSpeed")]
    pub wind_speed: Option<String>,
}
