// 调用deepseek进行穿搭分析
use crate::analyse::weather_to_description;
use crate::models::QWeatherResponse;
use pulldown_cmark::{Options, Parser, html};
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
        请用简洁明了的语言，给出实用的建议,总字数不要超过100字。",
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
        let html_content = markdown2html(&choice.message.content.clone()).await?;
        Ok(html_content)
    } else {
        Err("DeepSeek 返回空响应".into())
    }
}

pub async fn markdown2html(md_content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(md_content, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    Ok(html_output)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::staticconfig::get_api_config;
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
