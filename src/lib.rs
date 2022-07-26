mod event;
mod kwai;

use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ConnectParams {
    pub app_id: String,
    pub code: String,
    pub play_id: Option<u32>,
    /// 游戏中的头像url
    pub header: Option<String>,
    /// 游戏中的角色名称
    pub role_name: Option<String>,
}

#[derive(Deserialize)]
pub struct ConnectResponse {
    /// 主播的快手号
    pub ks_uid: u32,
    pub user: User,
    pub token: String,
}

#[derive(Deserialize)]
pub struct User {
    // openid
    id: String,
    // 昵称
    name: Option<String>,
    // 头像url
    header: Option<String>,
}

pub struct Context {
    connect_url: Url,
    disconnect_url: Url,
    poll_url: Url,
}

impl Context {
    pub fn new(host: &str) -> anyhow::Result<Self> {
        let mut connect_url = Url::parse("https://example.com/openapi/sdk/v1/connect")?;
        let mut disconnect_url = Url::parse("https://example.com/openapi/sdk/v1/disconnect")?;
        let mut poll_url = Url::parse("https://example.com/openapi/sdk/v1/poll")?;
        connect_url.set_host(Some(host))?;
        disconnect_url.set_host(Some(host))?;
        poll_url.set_host(Some(host))?;
        Ok(Context {
            connect_url,
            disconnect_url,
            poll_url,
        })
    }
}
