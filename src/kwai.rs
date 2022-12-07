use anyhow::bail;
use reqwest::Url;
use serde::Serialize;
use serde_json::Value;

use crate::event::Event;
use crate::{ConnectResp, DisconnectParams};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectReq {
    /// 快手的域名
    pub host: String,
    pub app_id: String,
    pub code: String,
    pub play_id: u32,
    pub app_type: u8,
    // always true
    pub use_sdk: bool,
    /// 游戏中的头像url
    pub header: Option<String>,
    /// 游戏中的角色名称
    pub role_name: Option<String>,
}

pub async fn connect(client: &reqwest::Client, params: &ConnectReq) -> anyhow::Result<ConnectResp> {
    let mut url = Url::parse("https://example.com/openapi/sdk/v1/connect")?;
    url.set_host(Some(&params.host))?;
    let response = client.post(url).json(params).send().await?;
    let value = response.json::<serde_json::Value>().await?;
    let result = value.get("result").and_then(Value::as_u64).unwrap_or(0);
    if result != 1 {
        bail!("response: {value}")
    }
    Ok(serde_json::from_value(value)?)
}

pub async fn disconnect(params: &DisconnectParams) -> anyhow::Result<()> {
    let mut url = Url::parse("https://example.com/openapi/sdk/v1/disconnect")?;
    url.set_host(Some(&params.host))?;

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .query(&[("token", &params.token)])
        .send()
        .await?;
    let value = response.json::<Value>().await?;
    let result = value.get("result").and_then(Value::as_u64).unwrap_or(0);
    if result != 1 {
        bail!("response: {value}")
    }
    Ok(())
}

#[derive(Debug)]
pub struct PollResp {
    pub p_cursor: String,
    /// 休息的毫秒数
    pub sleep: u64,
    pub data: Vec<Event>,
}

pub async fn poll(
    client: &reqwest::Client,
    url: Url,
    token: &str,
    p_cursor: &str,
    len: u32,
) -> anyhow::Result<Option<PollResp>> {
    let response = client
        .get(url)
        .query(&[
            ("token", token),
            ("pCursor", p_cursor),
            ("len", &len.to_string()),
        ])
        .send()
        .await?;
    let value = response.json::<Value>().await?;
    let result = value.get("result").and_then(Value::as_u64).unwrap_or(0);
    let value: Option<Value> = match result {
        1 => serde_json::from_value(value)?,
        2 => return Ok(None),
        _ => bail!("response: {value}"),
    };
    let Some(value) = value else { return Ok(None); };
    let resp = PollResp {
        p_cursor: value
            .get("pCursor")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_owned(),
        sleep: value.get("sleep").and_then(Value::as_u64).unwrap_or(0),
        data: value
            .get("data")
            .and_then(Value::as_array)
            .map(|t| t.iter().map(Into::into).collect())
            .unwrap_or_default(),
    };
    Ok(Some(resp))
}
