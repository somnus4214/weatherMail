use crate::staticconfig::get_smtp_config;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
fn mail_send(mes: &str, target_mail: &str) -> Result<(), Box<dyn std::error::Error>> {
    let smtp_config = get_smtp_config();
    let email = Message::builder()
        .from(smtp_config.from.parse()?)
        .to(target_mail.parse()?)
        .subject("Test from Rust")
        .body(String::from(mes))?;

    let creds = Credentials::new(smtp_config.username.clone(), smtp_config.password.clone());

    let mailer = SmtpTransport::starttls_relay(&smtp_config.server)?
        .port(smtp_config.port)
        .credentials(creds)
        .build();

    mailer.send(&email)?;
    println!("邮件发送成功!");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mail_send() {
        let mes = "hello from smtp";
        let test_mail = "chen1921460502@outlook.com";
        match mail_send(mes, test_mail) {
            Ok(_) => println!("测试通过"),
            Err(e) => eprintln!("发送失败: {}", e),
        }
    }
}
