use crate::analyse::weather_report;
use crate::weather::get_today_weather;
pub async fn main_logic(
    location: &str,
    city_name: &str,
    api_key: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let data = get_today_weather(location, api_key).await?;
    println!("成功获取{}的天气数据,共{}条", city_name, data.hourly.len());
    if let Some(ref update_time) = data.update_time {
        println!("数据更新时间:{}", update_time);
    }
    if data.hourly.is_empty() {
        return Err("未获取到任何天气数据".into());
    }
    let analyse_report = weather_report(&data)?;
    println!(
        "当日全天气温：\n最高温:{}，最低温:{}，平均温度:{}，昼夜温差:{}",
        analyse_report.max_temp,
        analyse_report.min_temp,
        analyse_report.mean_temp,
        analyse_report.range_temp
    );
    for (i, weather) in data.hourly.iter().take(4).enumerate() {
        println!("\n--- 第{}小时预报 ---", i + 1);
        println!("时间: {}", weather.fx_time);
        println!("天气: {}", weather.text);
        println!("温度: {}°C", weather.temp);
        if let Some(ref dir) = weather.wind_dir {
            let scale = weather.wind_scale.as_deref().unwrap_or("未知");
            println!("风向: {} {}级", dir, scale);
        }
        if let Some(ref humidity) = weather.humidity {
            println!("湿度: {}%", humidity);
        }
        if let Some(ref pop) = weather.pop {
            println!("降水概率: {}%", pop);
        }
    }

    Ok(())
}
