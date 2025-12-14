// 调用deepseek进行穿搭分析
use crate::models::QWeatherResponse;
use crate::staticconfig::get_api_config;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct DeepSeekRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
}

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct DeepSeekResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: MessageContent,
}

#[derive(Debug, Deserialize)]
struct MessageContent {
    content: String,
}

fn weather_to_description(data: &QWeatherResponse) -> String {
    let mut desc = String::new();

    // 当前天气详情
    if let Some(first) = data.hourly.first() {
        desc.push_str("=== 当前天气详情 ===\n");
        desc.push_str(&format!("天气状况: {}\n", first.text));
        desc.push_str(&format!("当前温度: {}°C\n", first.temp));
        desc.push_str(&format!(
            "体感温度(露点): {}°C\n",
            first.dew.as_deref().unwrap_or("未知")
        ));
        desc.push_str(&format!(
            "湿度: {}%\n",
            first.humidity.as_deref().unwrap_or("未知")
        ));
        desc.push_str(&format!(
            "风向: {} ({}°)\n",
            first.wind_dir.as_deref().unwrap_or("未知"),
            first.wind_360.as_deref().unwrap_or("未知")
        ));
        desc.push_str(&format!(
            "风力: {} 级\n",
            first.wind_scale.as_deref().unwrap_or("未知")
        ));
        desc.push_str(&format!(
            "风速: {} km/h\n",
            first.wind_speed.as_deref().unwrap_or("未知")
        ));
        desc.push_str(&format!(
            "降水概率: {}%\n",
            first.pop.as_deref().unwrap_or("未知")
        ));
        desc.push_str(&format!(
            "降水量: {} mm\n",
            first.precip.as_deref().unwrap_or("0.0")
        ));
        desc.push_str(&format!(
            "气压: {} hPa\n",
            first.pressure.as_deref().unwrap_or("未知")
        ));
        desc.push_str(&format!(
            "云量: {}%\n\n",
            first.cloud.as_deref().unwrap_or("未知")
        ));
    }

    // 温度统计
    let temps: Vec<f32> = data
        .hourly
        .iter()
        .filter_map(|h| h.temp.parse().ok())
        .collect();

    if !temps.is_empty() {
        let max_temp = temps.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        let min_temp = temps.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        let avg_temp = temps.iter().sum::<f32>() / temps.len() as f32;

        desc.push_str("=== 今日温度统计 ===\n");
        desc.push_str(&format!("最高温度: {}°C\n", max_temp as i32));
        desc.push_str(&format!("最低温度: {}°C\n", min_temp as i32));
        desc.push_str(&format!("平均温度: {:.1}°C\n", avg_temp));
        desc.push_str(&format!("温差: {}°C\n\n", (max_temp - min_temp) as i32));
    }

    // 湿度统计
    let humidities: Vec<f32> = data
        .hourly
        .iter()
        .filter_map(|h| h.humidity.as_ref()?.parse().ok())
        .collect();

    if !humidities.is_empty() {
        let avg_humidity = humidities.iter().sum::<f32>() / humidities.len() as f32;
        desc.push_str(&format!("平均湿度: {:.0}%\n", avg_humidity));
    }

    // 降水分析
    let total_precip: f32 = data
        .hourly
        .iter()
        .filter_map(|h| h.precip.as_ref()?.parse::<f32>().ok())
        .sum();

    let max_pop: f32 = data
        .hourly
        .iter()
        .filter_map(|h| h.pop.as_ref()?.parse::<f32>().ok())
        .fold(0.0, |a, b| a.max(b));

    if total_precip > 0.0 || max_pop > 0.0 {
        desc.push_str("\n=== 降水信息 ===\n");
        desc.push_str(&format!("预计总降水量: {:.1} mm\n", total_precip));
        desc.push_str(&format!("最高降水概率: {:.0}%\n", max_pop));
    }

    // 天气变化趋势
    let weather_changes: Vec<String> = data
        .hourly
        .iter()
        .take(8)
        .enumerate()
        .map(|(i, h)| {
            let hour = h
                .fx_time
                .split('T')
                .nth(1)
                .and_then(|t| t.split(':').next())
                .unwrap_or("--");
            format!("{}时: {} {}°C", hour, h.text, h.temp)
        })
        .collect();

    desc.push_str("\n=== 未来8小时天气趋势 ===\n");
    desc.push_str(&weather_changes.join("\n"));
    desc.push_str("\n");

    desc
}

pub async fn deepseek_analysis(
    data: &QWeatherResponse,
    api_key: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let weather_desc = weather_to_description(data);

    let prompt = format!(
        "根据以下详细的天气数据，请给出专业的穿搭建议：\n\n{}\n\n请综合考虑温度、湿度、风力、降水等因素，从以下方面给出建议：\n\
        1. 上衣推荐（考虑温度和体感）\n\
        2. 下装推荐\n\
        3. 外套建议（考虑温差和风力）\n\
        4. 配饰提醒（如雨具、帽子、围巾等）\n\
        5. 特别注意事项（如防晒、保暖、防雨等）\n\n\
        请用简洁明了的语言，给出实用的建议。",
        weather_desc
    );

    let request = DeepSeekRequest {
        model: "deepseek-chat".to_string(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: "你是一个专业的时尚穿搭顾问，擅长根据天气情况给出实用的穿搭建议。你会综合考虑温度、湿度、风力、降水等多种因素，给出详细且实用的建议。".to_string(),
            },
            Message {
                role: "user".to_string(),
                content: prompt,
            },
        ],
        temperature: 0.7,
    };

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.deepseek.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(format!("DeepSeek API 错误: {}", error_text).into());
    }

    let deepseek_response: DeepSeekResponse = response.json().await?;

    if let Some(choice) = deepseek_response.choices.first() {
        Ok(choice.message.content.clone())
    } else {
        Err("DeepSeek 返回空响应".into())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::BufReader;

    fn readfromjson(address: &str) -> Result<QWeatherResponse, Box<dyn std::error::Error>> {
        let file = File::open(address)?;
        let reader = BufReader::new(file);
        let qweather_data: QWeatherResponse = serde_json::from_reader(reader)?;
        Ok(qweather_data)
    }

    #[tokio::test]
    async fn test_deepseek_analysis() {
        let data = readfromjson("data/example.json").expect("Failed to read JSON");
        let api_key = get_api_config().deepseek.clone();

        match deepseek_analysis(&data, &api_key).await {
            Ok(suggestion) => {
                println!("穿搭建议:\n{}", suggestion);
                assert!(!suggestion.is_empty());
            }
            Err(e) => {
                eprintln!("调用失败: {}", e);
                panic!("测试失败");
            }
        }
    }

    #[test]
    fn test_weather_description() {
        let data = readfromjson("data/example.json").expect("Failed to read JSON");
        let desc = weather_to_description(&data);
        println!("天气描述:\n{}", desc);
        assert!(!desc.is_empty());
    }
}
