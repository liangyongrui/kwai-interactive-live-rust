# 快手互动直播 Rust SDK

## Introduction

一个异步的，高性能的，更符合人体工学的 快手互动直播 Rust SDK

（目标是更优雅的接口设计，所以在1.0.0之前，不保证向前兼容性）

## Usage

用法特别简单

1. 引入依赖

    ```toml
    [dependencies]
    kwai_interactive_live = "*"
    futures-lite = "1"
    ```

1. 建立连接, 并返回一个异步流

    ```rust
    use kwai_interactive_live::*;
    use futures_lite::stream::StreamExt;

    let p = ConnectParams {
        host: "xxxxxxx.com".to_string(),
        app_id: "app_id".to_string(),
        code: "code".to_string(),
        ..Default::default()
    };
    let (connect_resp, stream) = connect(p).await?;
    stream.into_stream().for_each(|event| match event { 
        Event::Gift(gift) => log::info!("收到个礼物: {gift:?} !"), 
        Event::Comment(comment) => log::info!("收到个弹幕: {comment:?} !"), 
        _ => log::info!("收到其他消息: {event:?}"), 
    }).await;
    ```

1. 关闭游戏 主动断开互动连接

    ```rust
    use kwai_interactive_live::*;

    let p = DisconnectParams {
        host: "xxxxxxx.com".to_string(),
        token: "xxxxxxxxxxx".to_string()
    };
    disconnect(&p).await?;
    ```
