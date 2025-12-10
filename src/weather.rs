use crate::models::QWeatherResponse;

pub async fn get_today_weather(
    location: &str,
    api_key: &str,
) -> Result<QWeatherResponse, Box<dyn std::error::Error>> {
    let url = format!(
        "https://k46apx2392.re.qweatherapi.com/v7/weather/24h?location={}&key={}",
        location, api_key
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
