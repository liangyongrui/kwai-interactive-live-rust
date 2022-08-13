//! 快手互动直播sdk

mod event;
mod kwai;
mod stream;

pub use event::*;
use kwai::ConnectReq;
use reqwest::Proxy;
use serde::{Deserialize, Serialize};
pub use stream::EventStream;

/// 建立连接的参数
#[derive(Default, Debug)]
pub struct ConnectParams {
    /// 快手互动直播的域名
    pub host: String,
    /// 游戏的appid
    pub app_id: String,
    /// 连接直播间的匹配码
    pub code: String,
    /// 游戏的玩法id
    ///
    /// 如果只有一个玩法可以不填(默认填0)
    pub play_id: u32,
    /// 游戏中的头像url
    pub header: Option<String>,
    /// 游戏中的角色名称
    pub role_name: Option<String>,
    /// http 代理
    pub http_proxies: Vec<Proxy>,
}

/// 建立连接成果后的返回值
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectResp {
    /// 主播的快手号
    pub ks_uid: u32,
    /// 主播信息
    pub user: User,
    /// 用于表示一次连接
    ///
    /// 做一些独立的动作时，可能需要用到，比如 disconnect
    pub token: String,
}

/// 断开连接的请求参数
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisconnectParams {
    /// 快手互动直播的域名
    pub host: String,
    /// 建立连接时，获取的token
    pub token: String,
}

/// 用户信息
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    /// openid
    pub id: String,
    /// 昵称
    pub user_name: Option<String>,
    /// 头像url
    pub head_url: Option<String>,
    /// 性别
    pub gender: Option<String>,
}

/// 建立连接
///
/// 返回主播信息 和 一个接收消息的异步Stream
///
/// # Errors
///
/// 连接失败，会返回Err
///
/// 一般都是因为code错误导致的
#[inline]
pub async fn connect(params: ConnectParams) -> anyhow::Result<(ConnectResp, EventStream)> {
    let mut http_client_builder = reqwest::ClientBuilder::new();
    let req = ConnectReq {
        host: params.host,
        app_id: params.app_id,
        code: params.code,
        play_id: params.play_id,
        header: params.header,
        role_name: params.role_name,
    };
    for proxy in params.http_proxies {
        http_client_builder = http_client_builder.proxy(proxy);
    }
    let http_client = http_client_builder.build()?;
    let connect_resp = kwai::connect(&http_client, &req).await?;
    let stream = EventStream::new(http_client, &req.host, connect_resp.token.clone())?;
    Ok((connect_resp, stream))
}

/// 断开连接
///
/// 退出互动模式时候调用
///
/// # Errors
///
/// 断开连接失败，会返回Err
///
/// 可能是因为网络问题导致的，一般可以忽略
#[inline]
pub async fn disconnect(params: &DisconnectParams) -> anyhow::Result<()> {
    kwai::disconnect(params).await
}
