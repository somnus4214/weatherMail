use dotenv::dotenv;
use std::env;
mod analyse;
mod logic;
mod models;
mod weather;
use crate::logic::main_logic;
// 城市ID查询: https://github.com/qwd/LocationList
// 成都: 101270101, 北京: 101010100

#[tokio::main]
async fn main() {
    dotenv().ok();
    match env::var("QWEATHER_KEY") {
        Ok(api_key) => {
            let location = "101010100"; // 成都
            let city_name = "北京";
            println!("正在获取{}天气...", city_name);

            if let Err(e) = main_logic(location, city_name, &api_key).await {
                eprintln!("错误: {}", e);
            }
        }
        Err(_) => {
            eprintln!("请在.env文件中设置 QWEATHER_KEY");
        }
    }
}
