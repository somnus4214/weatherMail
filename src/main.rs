mod analyse;
mod logic;
mod mailserv;
mod models;
mod staticconfig;
mod weather;
use crate::logic::main_logic;
use chrono::{Local, Timelike};
use tokio::signal;
use tokio::time::{Duration, sleep};

// 城市ID查询: https://github.com/qwd/LocationList
// 成都: 101270101, 北京: 101010100

#[tokio::main]
async fn main() {
    println!("服务启动:{}", Local::now());
    let shutdpown = tokio::spawn(async {
        signal::ctrl_c().await.expect("failed to listen to ctrl+c");
        println!("\n收到关闭信号，正在退出");
    });
    let task = tokio::spawn(async {
        loop {
            let now = Local::now();
            let next_run = if now.hour() < 9 {
                now.date_naive().and_hms_opt(9, 0, 0).unwrap()
            } else {
                (now.date_naive() + chrono::Days::new(1))
                    .and_hms_opt(9, 0, 0)
                    .unwrap()
            };
            let next_run = next_run.and_local_timezone(Local).unwrap();
            let duration = (next_run - now).to_std().unwrap();
            println!("下次运行时间: {}, 等待 {:?} ", next_run, duration);
            sleep(duration).await;
            println!("开始执行任务");
            if let Err(e) = do_task().await {
                eprintln!("任务执行失败: {}", e);
            }
            println!("任务执行成功");
        }
    });
    tokio::select! {
        _ =shutdpown =>{
            println!("服务已关闭");
        }
        _ =task=>{
            println!("任务已结束");
        }
    }
}

async fn do_task() -> Result<(), Box<dyn std::error::Error>> {
    println!("执行时间: {}", Local::now());
    println!("开始业务逻辑处理");
    let city = "成都";
    let target_mail = "chen1921460502@outlook.com";
    println!("查询城市: {}", city);
    main_logic(city, target_mail).await?;
    // 你的业务逻辑
    sleep(Duration::from_secs(2)).await;

    Ok(())
}
