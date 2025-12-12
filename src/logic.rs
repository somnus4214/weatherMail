use crate::analyse::weather_report;
use crate::mailserv::{self, WeatherEmailData, icon_set};
use crate::weather::get_today_weather;

pub async fn main_logic(
    location: &str,
    city_name: &str,
    api_key: &str,
    target_mail: &str,
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
    let email_content = WeatherEmailData {
        date: data.update_time.clone().unwrap_or_default(),
        city: city_name.to_string(),
        icon_base64: icon_set(&data.hourly[0].icon.clone().unwrap_or_default()),
        weather_desc: data.hourly[0].text.clone(),
        temp: data.hourly[0].temp.clone(),
        temp_max: analyse_report.max_temp.to_string(),
        temp_min: analyse_report.min_temp.to_string(),
        humidity: data.hourly[0]
            .humidity
            .clone()
            .unwrap_or_else(|| "未知".to_string()),
        wind_speed: data.hourly[0]
            .wind_speed
            .clone()
            .unwrap_or_else(|| "未知".to_string()),
        suggestion: format!(
            "今日气温范围:{}°C ~ {}°C，平均温度:{}°C，昼夜温差:{}°C",
            analyse_report.min_temp,
            analyse_report.max_temp,
            analyse_report.mean_temp,
            analyse_report.range_temp
        ),
    };
    mailserv::mail_send_html(target_mail, &email_content).await?;
    // println!(
    //     "当日全天气温：\n最高温:{}，最低温:{}，平均温度:{}，昼夜温差:{}",
    //     analyse_report.max_temp,
    //     analyse_report.min_temp,
    //     analyse_report.mean_temp,
    //     analyse_report.range_temp
    // );
    // for (i, weather) in data.hourly.iter().take(4).enumerate() {
    //     println!("\n--- 第{}小时预报 ---", i + 1);
    //     println!("时间: {}", weather.fx_time);
    //     println!("天气: {}", weather.text);
    //     println!("温度: {}°C", weather.temp);
    //     if let Some(ref dir) = weather.wind_dir {
    //         let scale = weather.wind_scale.as_deref().unwrap_or("未知");
    //         println!("风向: {} {}级", dir, scale);
    //     }
    //     if let Some(ref humidity) = weather.humidity {
    //         println!("湿度: {}%", humidity);
    //     }
    //     if let Some(ref pop) = weather.pop {
    //         println!("降水概率: {}%", pop);
    //     }
    // }
    Ok(())
}
