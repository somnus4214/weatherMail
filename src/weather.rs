use crate::models::QWeatherResponse;

pub fn query_city_location_id(city_name: &str) -> Option<&'static str> {
    let city_map = vec![
        ("邗江", "101190606"),
        ("成都", "101270101"),
        ("北京", "101010100"),
        // 可以继续添加更多城市
    ];

    for (name, id) in city_map {
        if name == city_name {
            return Some(id);
        }
    }
    None
}

pub async fn get_today_weather(
    city_name: &str,
    api_key: &str,
) -> Result<QWeatherResponse, Box<dyn std::error::Error>> {
    let location_id =
        query_city_location_id(city_name).ok_or_else(|| format!("未找到城市: {}", city_name))?;
    let url = format!(
        "https://k46apx2392.re.qweatherapi.com/v7/weather/24h?location={}&key={}",
        location_id, api_key
    );
    println!("请求URL: {}", url);

    let client = reqwest::Client::builder()
        .user_agent("weather_mail/1.0")
        .timeout(std::time::Duration::from_secs(10))
        .gzip(true) // 启用 gzip 解压
        .build()?;

    let response = client.get(&url).send().await?;
    let status = response.status();
    println!("HTTP状态: {}", status);

    if !status.is_success() {
        let error_text = response.text().await?;
        return Err(format!("HTTP错误: {}, {}", status, error_text).into());
    }

    let response_text = response.text().await?;
    println!(
        "API返回前100字符: {}",
        &response_text[..response_text.len().min(100)]
    );

    let data: QWeatherResponse = serde_json::from_str(&response_text)?;

    if data.code != "200" {
        return Err(format!("API错误，代码: {}", data.code).into());
    }

    Ok(data)
}
