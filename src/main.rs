mod analyse;
mod logic;
mod mailserv;
mod models;
mod staticconfig;
mod weather;
use crate::logic::main_logic;
// 城市ID查询: https://github.com/qwd/LocationList
// 成都: 101270101, 北京: 101010100

#[tokio::main]
async fn main() {
    let city_name = "邗江";
    let target_mail = "chen1921460502@outlook.com";
    println!("正在获取{}天气...", city_name);

    if let Err(e) = main_logic(city_name, target_mail).await {
        eprintln!("错误: {}", e);
    }
}
