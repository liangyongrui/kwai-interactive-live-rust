//! test sdk demo
use std::time::Duration;

use futures::StreamExt;
use kwai_interactive_live::{connect, disconnect, ConnectParams, DisconnectParams, Event};

#[tokio::test]
async fn test() -> anyhow::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp_millis()
        .try_init()
        .unwrap();

    let p = ConnectParams {
        host: "xxxxxxxxx.com".to_string(),
        app_id: "app_id".to_string(),
        code: "code".to_string(),
        ..Default::default()
    };
    let (resp, stream) = connect(p).await?;
    tokio::spawn(async {
        stream
            .for_each(|event| async move {
                match event {
                    Event::Gift(gift) => log::info!("收到个礼物: {gift:?} !"),
                    Event::Comment(comment) => log::info!("收到个弹幕: {comment:?} !"),
                    _ => log::info!("收到其他消息: {event:?}"),
                }
            })
            .await;
    });
    tokio::time::sleep(Duration::from_secs(600)).await;
    let p = DisconnectParams {
        host: "xxxxxxxxx.com".to_string(),
        token: resp.token.clone(),
    };
    disconnect(&p).await?;
    Ok(())
}
