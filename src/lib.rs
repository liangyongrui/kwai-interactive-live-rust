mod kwai;

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
