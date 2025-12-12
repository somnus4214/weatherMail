mod analyse;
mod logic;
mod mailserv;
mod models;
mod staticconfig;
mod weather;
use crate::logic::main_logic;
use crate::staticconfig::get_api_config;
// 城市ID查询: https://github.com/qwd/LocationList
// 成都: 101270101, 北京: 101010100

#[tokio::main]
async fn main() {
    let api_config = get_api_config();
    let qweather_api_key = &api_config.qweather;
    let location = "101010100"; // 
    let city_name = "北京";
    let target_mail = "chen1921460502@outlook.com";
    println!("正在获取{}天气...", city_name);

    if let Err(e) = main_logic(location, city_name, &qweather_api_key, target_mail).await {
        eprintln!("错误: {}", e);
    }
}
