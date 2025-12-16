use chrono::{Duration, NaiveDate};
use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TemperatureRecord {
    pub date: String,
    pub temp_mean: f32,
    pub temp_max: f32,
    pub temp_min: f32,
    pub city: String,
}

// 新增：返回的温度变化结构体
#[derive(Debug)]
pub struct TemperatureChange {
    pub mean_change: f32,
    pub max_change: f32,
    pub min_change: f32,
}

pub fn save_temperature_record(
    file_path: &str,
    record: &TemperatureRecord,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(file_path);
    let mut existing_records = Vec::new();
    let mut existing_dates = HashSet::new();

    if path.exists() {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut csv_reader = Reader::from_reader(reader);
        for result in csv_reader.deserialize() {
            let record: TemperatureRecord = result?;
            existing_dates.insert(record.date.clone());
            existing_records.push(record);
        }
    }
    if existing_dates.contains(record.date.as_str()) {
        println!("日期{}已存在，跳过保存", record.date);
        return Ok(());
    }
    existing_records.push(record.clone());
    let file = File::create(file_path)?;
    let writer = BufWriter::new(file);
    let mut csv_writer = Writer::from_writer(writer);
    for rec in existing_records {
        csv_writer.serialize(rec)?;
    }
    csv_writer.flush()?;
    println!("成功保存日期{}的温度数据", record.date);
    Ok(())
}

pub fn count_temperature_change(
    file_path: &str,
    current_record: &TemperatureRecord,
) -> Result<Option<TemperatureChange>, Box<dyn std::error::Error>> {
    let path = Path::new(file_path);

    // 读取已有记录（如果有）
    let mut existing_records: Vec<TemperatureRecord> = Vec::new();
    if path.exists() {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut csv_reader = Reader::from_reader(reader);
        for result in csv_reader.deserialize() {
            let record: TemperatureRecord = result?;
            existing_records.push(record);
        }
    } else {
        // 若文件不存在，先创建空文件（后续会写入）
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        let mut csv_writer = Writer::from_writer(writer);
        csv_writer.flush()?;
    }

    // 计算前一天日期字符串
    let current_date = NaiveDate::parse_from_str(&current_record.date, "%Y-%m-%d")?;
    let prev_date = current_date
        .checked_sub_signed(Duration::days(1))
        .ok_or("日期计算失败")?;
    let prev_date_str = prev_date.format("%Y-%m-%d").to_string();

    // 查找前一天记录并检查今天记录是否已存在
    let mut prev_record_opt: Option<TemperatureRecord> = None;
    let mut today_present = false;
    let filtered_existing_records: Vec<&TemperatureRecord> = existing_records
        .iter()
        .filter(|c| c.city == current_record.city)
        .collect();
    for r in filtered_existing_records {
        if r.date == prev_date_str {
            prev_record_opt = Some(r.clone());
        }
        if r.date == current_record.date {
            today_present = true;
        }
    }

    // 如果今天记录不存在，追加并写回文件
    if !today_present {
        existing_records.push(current_record.clone());
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        let mut csv_writer = Writer::from_writer(writer);
        for rec in &existing_records {
            csv_writer.serialize(rec)?;
        }
        csv_writer.flush()?;
        println!("已保存今日({})的温度记录", current_record.date);
    } else {
        println!("今日({})记录已存在，未重复保存", current_record.date);
    }

    // 如果找到了前一天记录则计算并返回变化，否则返回 None
    if let Some(prev) = prev_record_opt {
        let mean_change = current_record.temp_mean - prev.temp_mean;
        let max_change = current_record.temp_max - prev.temp_max;
        let min_change = current_record.temp_min - prev.temp_min;
        println!(
            "与前一天({})相比，平均温度变化: {:.2}°C, 最高温度变化: {:.2}°C, 最低温度变化: {:.2}°C",
            prev.date, mean_change, max_change, min_change
        );
        Ok(Some(TemperatureChange {
            mean_change,
            max_change,
            min_change,
        }))
    } else {
        println!("未找到前一天({})的温度记录，无法计算变化", prev_date_str);
        Ok(None)
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_save_temperature_record() {
        let record = TemperatureRecord {
            date: "2024-06-01".to_string(),
            temp_mean: 25.0,
            temp_max: 30.0,
            temp_min: 20.0,
            city: "测试城市".to_string(),
        };
        let result = save_temperature_record("data/temperature_records.csv", &record);
        assert!(result.is_ok());
    }

    #[test]
    fn test_count_temperature_change() {
        let record = TemperatureRecord {
            date: "2024-06-02".to_string(),
            temp_mean: 26.0,
            temp_max: 31.0,
            temp_min: 21.0,
            city: "测试城市".to_string(),
        };
        let result = count_temperature_change("data/temperature_records.csv", &record);
        assert!(result.is_ok());
        if let Ok(Some(change)) = result {
            println!(
                "Mean Change: {:.2}, Max Change: {:.2}, Min Change: {:.2}",
                change.mean_change, change.max_change, change.min_change
            );
        }
    }
}
