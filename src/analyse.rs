use plotters::prelude::*;
use plotters::style::text_anchor::{HPos, Pos, VPos};
use std::f32;

use crate::models::QWeatherResponse;

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

// 解析时间字符串，提取小时
fn parse_hour(fx_time: &str) -> Result<i32, Box<dyn std::error::Error>> {
    // "2025-12-10T15:00+08:00" -> 15
    let time_part = fx_time.split('T').nth(1).ok_or("Invalid time format")?;
    let hour = time_part
        .split(':')
        .next()
        .ok_or("Invalid hour format")?
        .parse::<i32>()?;
    Ok(hour)
}

pub fn temperature_trend_plot(data: &[(String, f32)]) -> Result<(), Box<dyn std::error::Error>> {
    if data.is_empty() {
        return Err("No data to plot".into());
    }

    let root = BitMapBackend::new("temp/temperature_trend.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    // 提取温度值
    let temps: Vec<f32> = data.iter().map(|(_, temp)| *temp).collect();
    let min_temp = temps.iter().fold(f32::INFINITY, |a, &b| a.min(b));
    let max_temp = temps.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    let temp_range = (min_temp.floor() as i32 - 2)..(max_temp.ceil() as i32 + 2);

    // 解析时间
    let hours: Vec<i32> = data
        .iter()
        .map(|(time, _)| parse_hour(time).unwrap_or(0))
        .collect();

    let hour_range = 0..data.len() as i32;

    let font_path = "/usr/share/fonts/truetype/MapleMonoNF/MapleMono-NF-CN-Regular.ttf";

    let mut chart = ChartBuilder::on(&root)
        .caption("Temperature Trend", (font_path, 40))
        .margin(15)
        .x_label_area_size(50)
        .y_label_area_size(60)
        .build_cartesian_2d(hour_range.clone(), temp_range)?;

    chart
        .configure_mesh()
        .x_desc("Time")
        .y_desc("Temp(°C)")
        .label_style((font_path, 18))
        .y_label_formatter(&|y| format!("{:.1}°C", y))
        .x_label_formatter(&|idx| {
            if (*idx as usize) < hours.len() {
                format!("{}:00", hours[*idx as usize])
            } else {
                String::new()
            }
        })
        .draw()?;

    // 绘制折线
    chart
        .draw_series(LineSeries::new(
            temps.iter().enumerate().map(|(i, &t)| (i as i32, t as i32)),
            &RED,
        ))?
        .label("Temperature")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    // 绘制数据点
    chart.draw_series(
        temps
            .iter()
            .enumerate()
            .map(|(i, &t)| Circle::new((i as i32, t as i32), 4, RED.filled())),
    )?;

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .label_font((font_path, 18))
        .draw()?;

    root.present()?;
    println!("图表已保存到 temp/temperature_trend.png");
    Ok(())
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
    fn test_temperature_plot() {
        // 使用新格式测试
        let data = vec![
            ("2025-12-10T15:00+08:00".to_string(), 16.0),
            ("2025-12-10T16:00+08:00".to_string(), 17.5),
            ("2025-12-10T17:00+08:00".to_string(), 18.2),
            ("2025-12-10T18:00+08:00".to_string(), 17.8),
            ("2025-12-10T19:00+08:00".to_string(), 16.5),
            ("2025-12-10T20:00+08:00".to_string(), 15.3),
        ];
        temperature_trend_plot(&data).expect("绘图失败");
        assert!(std::path::Path::new("temp/temperature_trend.png").exists());
    }
}
