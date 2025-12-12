use crate::staticconfig::get_smtp_config;
use base64::{Engine as _, engine::general_purpose};
use lettre::AsyncTransport;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, Message, message::header};
use std::fs;
use tera::{Context, Tera};

pub async fn mail_send_html(target_mail: &str) -> Result<(), Box<dyn std::error::Error>> {
    let smtp_config = get_smtp_config();

    let image_data = fs::read("assets/icons/150-fill.svg")?;
    let base64_image = general_purpose::STANDARD.encode(&image_data);
    let data_uri = format!("data:image/svg+xml;base64,{}", base64_image);

    let mut ctx = Context::new();
    ctx.insert("date", "20251112");
    ctx.insert("city", "chengdu");
    ctx.insert("icon_url", &data_uri); // 使用 Base64 Data URI
    ctx.insert("weather_desc", "sunny day");
    ctx.insert("temp", "10");
    ctx.insert("temp_max", "17");
    ctx.insert("temp_min", "5");
    ctx.insert("humidity", "80");
    ctx.insert("wind_speed", "12");
    ctx.insert("suggestion", "example_txt");
    ctx.insert("bot_name", "Rust Weather Bot");

    let tera = Tera::new("templates/*.tera")?;
    let html = tera.render("weatheremail.html.tera", &ctx)?;
    let email = Message::builder()
        .from(smtp_config.from.parse()?)
        .to(target_mail.parse()?)
        .subject("Test from Rust")
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

    #[tokio::test]
    async fn test_html_mail_send() {
        let test_mail = "chen1921460502@outlook.com";
        match mail_send_html(test_mail).await {
            Ok(_) => println!("测试通过"),
            Err(e) => eprintln!("发送失败：{}", e),
        }
    }
}
