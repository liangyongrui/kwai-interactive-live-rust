use anyhow::{bail, Error};
use serde::Deserialize;
use serde_json::Value;

use crate::User;

#[derive(Debug)]
pub enum Event {
    /// 送礼
    Gift(Gift),
    /// 弹幕
    Comment(Comment),
    /// 点赞
    Like(Like),
    /// 分享
    Share(Share),
    /// 关注主播
    Follow(Follow),
    /// 自定义任务
    Task(Task),
    /// 未知类型 或 异常
    Unknown(Value, Error),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Gift {
    /// 送礼人信息
    pub user: User,
    /// 礼物id
    pub gift_id: String,
    /// 礼物名称
    pub gift_name: String,
    /// 礼物总价值
    pub gift_total_dou: u32,
    /// 礼物个数
    pub count: u32,
}

#[derive(Deserialize, Debug)]
pub struct Comment {
    pub user: User,
    /// 弹幕内容
    pub content: String,
}
#[derive(Deserialize, Debug)]
pub struct Like {
    pub user: User,
    /// 点赞个数
    pub count: u32,
}

#[derive(Deserialize, Debug)]
pub struct Share {
    pub user: User,
}
#[derive(Deserialize, Debug)]
pub struct Follow {
    pub user: User,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    /// 完成任务的用户信息，直播间内的集体任务没有用户
    pub user: Option<User>,
    /// 任务id
    pub task_id: u32,
}

impl From<&Value> for Event {
    fn from(value: &Value) -> Self {
        match parse(value) {
            Ok(e) => e,
            Err(e) => Event::Unknown(value.clone(), e),
        }
    }
}

fn parse(value: &Value) -> anyhow::Result<Event> {
    let cmd = value.get("cmd").and_then(|t| t.as_u64()).unwrap_or(0);
    let payload = value.get("payload").unwrap_or(&Value::Null);
    match cmd {
        2 => Ok(Event::Gift(serde_json::from_value(payload.clone())?)),
        3 => Ok(Event::Comment(serde_json::from_value(payload.clone())?)),
        5 => Ok(Event::Task(serde_json::from_value(payload.clone())?)),
        7 => Ok(Event::Like(serde_json::from_value(payload.clone())?)),
        9 => Ok(Event::Share(serde_json::from_value(payload.clone())?)),
        10 => Ok(Event::Follow(serde_json::from_value(payload.clone())?)),
        _ => bail!("cmd unsupport"),
    }
}
