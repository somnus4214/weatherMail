use crate::models::QWeatherResponse;
use std::f32;

pub mod dsanal;

#[allow(dead_code)]
#[derive(Debug)]
pub struct AnalyseReport {
    pub mean_temp: f32,
    pub max_temp: f32,
    pub min_temp: f32,
    pub range_temp: f32,
}

pub fn weather_report(
    data: &QWeatherResponse,
) -> Result<AnalyseReport, Box<dyn std::error::Error>> {
    let mut today_temp: Vec<f32> = Vec::new();
    for i in &data.hourly {
        let temp = i.temp.parse::<f32>()?;
        today_temp.push(temp);
    }
    let mean = today_temp.iter().sum::<f32>() / today_temp.len() as f32;
    let max = today_temp.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    let min = today_temp.iter().fold(f32::INFINITY, |a, &b| a.min(b));
    let range = max - min;
    Ok(AnalyseReport {
        mean_temp: mean,
        max_temp: max,
        min_temp: min,
        range_temp: range,
    })
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
        .map(|(_, h)| {
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

    #[test]
    fn test_show_analyse() {
        let data = readfromjson("data/example.json").expect("error");
        let result = weather_report(&data).expect("errorfromlogic");
        println!("result:{:?}", result)
    }

    #[test]
    fn test_show_description() {
        let data = readfromjson("data/example.json").expect("error!!!!");
        let desc = weather_to_description(&data);
        println!("天气描述:\n{}", desc);
    }
}
