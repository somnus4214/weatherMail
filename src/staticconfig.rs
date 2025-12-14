use std::env;

use dotenv::dotenv;
use once_cell::sync::Lazy;

#[derive(Debug, Clone)]
pub struct SmtpConfig {
    pub server: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub from: String,
}
static SMTP_CONFIG: Lazy<SmtpConfig> = Lazy::new(|| {
    dotenv().ok();
    SmtpConfig {
        server: env::var("SMTP_SERVER").expect("SMTP_SERVER 未设置"),
        port: env::var("SMTP_PORT")
            .expect("SMTP_PORT 未设置")
            .parse()
            .expect("SMTP_PORT 必须是数字"),
        username: env::var("SMTP_USERNAME").expect("SMTP_USERNAME 未设置"),
        password: env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD 未设置"),
        from: env::var("SMTP_FROM").expect("SMTP_FROM 未设置"),
    }
});
pub fn get_smtp_config() -> &'static SmtpConfig {
    &SMTP_CONFIG
}

#[derive(Debug, Clone)]
pub struct ApiConfig {
    pub qweather: String,
    pub deepseek: String,
}
static API_CONFIG: Lazy<ApiConfig> = Lazy::new(|| {
    dotenv().ok();
    ApiConfig {
        qweather: env::var("QWEATHER_KEY").expect("QWEATHER_KEY未设置"),
        deepseek: env::var("DEEPSEEK_API_KEY").expect("DEEPSEEK_API_KEY未设置"),
    }
});
pub fn get_api_config() -> &'static ApiConfig {
    &API_CONFIG
}
