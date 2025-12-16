use crate::analyse::dsanal::deepseek_analysis;
use crate::analyse::local::{TemperatureRecord, count_temperature_change};
use crate::analyse::weather_report;
use crate::mailserv::{self, WeatherEmailData, icon_set};
use crate::weather::get_today_weather;

pub async fn main_logic(
    location: &str,
    city_name: &str,
    target_mail: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let api_config = crate::staticconfig::get_api_config();
    let qweather_api_key = &api_config.qweather;
    let deepseek_api_key = &api_config.deepseek;
    let data = get_today_weather(location, qweather_api_key).await?;
    println!("成功获取{}的天气数据,共{}条", city_name, data.hourly.len());
    if let Some(ref update_time) = data.update_time {
        println!("数据更新时间:{}", update_time);
    }
    if data.hourly.is_empty() {
        return Err("未获取到任何天气数据".into());
    }
    let deepseek_desc = deepseek_analysis(&data, deepseek_api_key).await?;
    let analyse_report = weather_report(&data)?;

    // 提取日期部分 "2025-12-16T12:26+08:00" -> "2025-12-16"
    let date_str = data
        .update_time
        .clone()
        .unwrap_or_default()
        .split('T')
        .next()
        .unwrap_or_default()
        .to_string();

    let current_record = TemperatureRecord {
        date: date_str.clone(),
        temp_mean: analyse_report.mean_temp,
        temp_max: analyse_report.max_temp,
        temp_min: analyse_report.min_temp,
    };
    let temp_change = count_temperature_change("data/temperature_records.csv", &current_record)?;

    // 根据 temp_change 是否存在来设置温度变化值
    let (temp_mean_change, temp_max_change, temp_min_change) = if let Some(change) = temp_change {
        (
            format!("{:+.1}", change.mean_change),
            format!("{:+.1}", change.max_change),
            format!("{:+.1}", change.min_change),
        )
    } else {
        ("--".to_string(), "--".to_string(), "--".to_string())
    };

    let email_content = WeatherEmailData {
        date: date_str,
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
        suggestion: deepseek_desc,
        temp_mean_change,
        temp_max_change,
        temp_min_change,
    };
    mailserv::mail_send_html(target_mail, &email_content).await?;

    Ok(())
}
