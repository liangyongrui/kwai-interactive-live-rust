use anyhow::bail;
use reqwest::Url;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

use crate::event::Event;
use crate::{ConnectParams, ConnectResp, DisconnectParams};

async fn post<Req: Serialize, Resp: DeserializeOwned>(
    url: Url,
    params: &Req,
) -> anyhow::Result<Resp> {
    let client = reqwest::Client::new();
    let response = client.post(url).json(params).send().await?;
    let value = response.json::<serde_json::Value>().await?;
    let result = value.get("result").and_then(|t| t.as_u64()).unwrap_or(0);
    if result != 1 {
        bail!("response: {value}")
    }
    Ok(serde_json::from_value(value)?)
}

pub(crate) async fn connect(params: &ConnectParams) -> anyhow::Result<ConnectResp> {
    let mut url = Url::parse("https://example.com/openapi/sdk/v1/connect")?;
    url.set_host(Some(&params.host))?;
    post(url, params).await
}

pub(crate) async fn disconnect(params: &DisconnectParams) -> anyhow::Result<()> {
    let mut url = Url::parse("https://example.com/openapi/sdk/v1/disconnect")?;
    url.set_host(Some(&params.host))?;

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .query(&[("token", &params.token)])
        .send()
        .await?;
    let value = response.json::<Value>().await?;
    let result = value.get("result").and_then(|t| t.as_u64()).unwrap_or(0);
    if result != 1 {
        bail!("response: {value}")
    }
    Ok(())
}

#[derive(Debug)]
pub(crate) struct PollResp {
    pub p_cursor: String,
    /// 休息的毫秒数
    pub sleep: u64,
    pub data: Vec<Event>,
}

pub(crate) async fn poll(
    url: Url,
    token: &str,
    p_cursor: &str,
    len: u32,
) -> anyhow::Result<Option<PollResp>> {
    let client = reqwest::Client::new();
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
    let result = value.get("result").and_then(|t| t.as_u64()).unwrap_or(0);
    let value: Option<Value> = match result {
        1 => serde_json::from_value(value)?,
        2 => return Ok(None),
        _ => bail!("response: {value}"),
    };
    if value.is_none() {
        return Ok(None);
    }
    let value = value.unwrap();

    let resp = PollResp {
        p_cursor: value
            .get("pCursor")
            .and_then(|t| t.as_str())
            .unwrap_or("")
            .to_string(),
        sleep: value.get("sleep").and_then(|t| t.as_u64()).unwrap_or(0),
        data: value
            .get("data")
            .and_then(|t| t.as_array())
            .map(|t| t.iter().map(|x| x.into()).collect())
            .unwrap_or_default(),
    };
    Ok(Some(resp))
}
