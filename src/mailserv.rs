use crate::staticconfig::get_smtp_config;
use lettre::AsyncTransport;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, Message, message::header};
use tera::{Context, Tera};

pub struct WeatherEmailData {
    pub date: String,
    pub city: String,
    pub icon_base64: String,
    pub weather_desc: String,
    pub temp: String,
    pub temp_max: String,
    pub temp_min: String,
    pub humidity: String,
    pub wind_speed: String,
    pub suggestion: String,
}

impl WeatherEmailData {
    fn to_context(&self) -> Context {
        let mut ctx = Context::new();
        ctx.insert("date", &self.date);
        ctx.insert("city", &self.city);
        ctx.insert("icon_url", &self.icon_base64);
        ctx.insert("weather_desc", &self.weather_desc);
        ctx.insert("temp", &self.temp);
        ctx.insert("temp_max", &self.temp_max);
        ctx.insert("temp_min", &self.temp_min);
        ctx.insert("humidity", &self.humidity);
        ctx.insert("wind_speed", &self.wind_speed);
        ctx.insert("suggestion", &self.suggestion);
        ctx.insert("bot_name", "Rust Weather Bot");
        ctx
    }
}

pub async fn mail_send_html(
    target_mail: &str,
    weather_data: &WeatherEmailData,
) -> Result<(), Box<dyn std::error::Error>> {
    let smtp_config = get_smtp_config();

    let ctx = weather_data.to_context();

    let tera = Tera::new("templates/*.tera")?;
    let html = tera.render("weatheremail.html.tera", &ctx)?;
    let email = Message::builder()
        .from(smtp_config.from.parse()?)
        .to(target_mail.parse()?)
        .subject("Weather Report")
        .header(header::ContentType::TEXT_HTML)
        .body(html)?;

    let creds = Credentials::new(smtp_config.username.clone(), smtp_config.password.clone());

    let mailer = AsyncSmtpTransport::<lettre::Tokio1Executor>::starttls_relay(&smtp_config.server)?
        .port(smtp_config.port)
        .credentials(creds)
        .build();

    mailer.send(email).await?;
    println!("邮件发送成功!");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use base64::{Engine as _, engine::general_purpose};
    use std::fs;

    #[tokio::test]
    async fn test_html_mail_send() {
        let image_data = fs::read("assets/icons/150-fill.svg").expect("Failed to read icon");
        let base64_image = general_purpose::STANDARD.encode(&image_data);
        let data_uri = format!("data:image/svg+xml;base64,{}", base64_image);

        let weather_data = WeatherEmailData {
            date: "20251112".to_string(),
            city: "chengdu".to_string(),
            icon_base64: data_uri,
            weather_desc: "sunny day".to_string(),
            temp: "10".to_string(),
            temp_max: "17".to_string(),
            temp_min: "5".to_string(),
            humidity: "80".to_string(),
            wind_speed: "12".to_string(),
            suggestion: "example_txt".to_string(),
        };

        let test_mail = "chen1921460502@outlook.com";
        match mail_send_html(test_mail, &weather_data).await {
            Ok(_) => println!("测试通过"),
            Err(e) => eprintln!("发送失败：{}", e),
        }
    }
}
