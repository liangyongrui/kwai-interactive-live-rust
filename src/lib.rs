mod event;
mod kwai;
mod stream;

pub use event::*;
use futures::Stream;
use serde::{Deserialize, Serialize};
use stream::EventStream;

#[derive(Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectParams {
    /// 快手的域名
    pub host: String,
    pub app_id: String,
    pub code: String,
    pub play_id: Option<u32>,
    /// 游戏中的头像url
    pub header: Option<String>,
    /// 游戏中的角色名称
    pub role_name: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectResp {
    /// 主播的快手号
    pub ks_uid: u32,
    pub user: User,
    pub token: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisconnectParams {
    pub host: String,
    /// 建立连接时，获取的token
    pub token: String,
}

/// 用户信息
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    // openid
    pub id: String,
    // 昵称
    pub user_name: Option<String>,
    // 头像url
    pub head_url: Option<String>,
    // 性别
    pub gender: Option<String>,
}

/// 建立连接
///
/// 返回主播信息 和 一个接收消息的异步Stream
pub async fn connect(
    params: &ConnectParams,
) -> anyhow::Result<(ConnectResp, impl Stream<Item = Event>)> {
    let connect_resp = kwai::connect(params).await?;
    let stream = EventStream::new(&params.host, connect_resp.token.clone())?;
    Ok((connect_resp, stream.into_stream()))
}

/// 断开连接
///
/// 退出互动模式时候调用
pub async fn disconnect(params: &DisconnectParams) -> anyhow::Result<()> {
    kwai::disconnect(params).await
}
