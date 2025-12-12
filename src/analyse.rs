use crate::models::QWeatherResponse;
use std::f32;

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
}
