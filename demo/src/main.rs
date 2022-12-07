//! sdk demo
#![allow(clippy::restriction)]
use core::time::Duration;
use std::io::Write;

use clap::Parser;
use futures_lite::stream::StreamExt;
use kwai_interactive_live::{connect, disconnect, ConnectParams, DisconnectParams, Event};

/// 互动游戏、工具 测试demo, 这些参数不输入也没事
#[derive(Parser, Debug, Clone)]
#[command()]
struct Args {
    #[arg(short = 'x', long)]
    code: Option<String>,
    #[arg(short = 'd', long)]
    host: Option<String>,
    #[arg(short, long)]
    app_type: Option<u8>,
    #[arg(short, long)]
    play_id: Option<u32>,
}

fn prompt(name: &str) -> String {
    let mut line = String::new();
    print!("{name}\n> ");
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Error: Could not read a line");

    return line.trim().to_string();
}

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp_millis()
        .try_init()
        .unwrap();
    let args = Args::parse();

    loop {
        if let Err(e) = run(args.clone()).await {
            log::error!("{e:?}");
        }
        prompt("\n运行结束，按enter重新运行程序");
    }
}

async fn run(args: Args) -> anyhow::Result<()> {
    let host = args.host.unwrap_or_else(|| prompt("输入host"));
    let app_type = if let Some(a) = args.app_type {
        a
    } else {
        prompt("输入 app 类型, 0 游戏, 1 工具").parse()?
    };
    let play_id = if let Some(a) = args.play_id {
        a
    } else {
        prompt("输入 playId").parse()?
    };
    let p = ConnectParams {
        app_type,
        host: host.clone(),
        play_id,
        code: args.code.unwrap_or_else(|| prompt("输入匹配码")),
        ..Default::default()
    };
    let (resp, stream) = connect(p).await?;
    log::info!("链接成功!");
    let sleep = prompt("输入链接持续时长，单位秒").parse()?;
    tokio::spawn(stream.into_stream().for_each(|event| match event {
        Event::Gift(gift) => log::info!("收到个礼物: {gift:?} !"),
        Event::Comment(comment) => log::info!("收到个弹幕: {comment:?} !"),
        _ => log::info!("收到其他消息: {event:?}"),
    }));

    tokio::time::sleep(Duration::from_secs(sleep)).await;
    let p = DisconnectParams {
        host,
        token: resp.token.clone(),
    };
    disconnect(&p).await?;
    Ok(())
}
