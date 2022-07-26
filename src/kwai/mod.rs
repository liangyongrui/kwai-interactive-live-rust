mod model;

use anyhow::bail;
use reqwest::Url;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::{ConnectParams, ConnectResponse, Context};

async fn post<Req: Serialize, Resp: DeserializeOwned>(
    url: Url,
    params: &Req,
) -> anyhow::Result<Resp> {
    let client = reqwest::Client::new();
    let response = client.post(url).json(params).send().await?;
    let value = response.json::<serde_json::Value>().await?;
    if value
        .get("result")
        .filter(|t| t.as_u64() != Some(1))
        .is_some()
    {
        bail!("response error: {value}")
    }

    Ok(serde_json::from_value(value)?)
}

/// query `&[("foo", "a"), ("foo", "b")])` gives `"foo=a&foo=b"`.
async fn get<Resp: DeserializeOwned>(url: Url, query: &[(&str, &str)]) -> anyhow::Result<Resp> {
    let client = reqwest::Client::new();
    let response = client.get(url).query(query).send().await?;
    let value = response.json::<serde_json::Value>().await?;
    if value
        .get("result")
        .filter(|t| t.as_u64() != Some(1))
        .is_some()
    {
        bail!("response error: {value}")
    }

    Ok(serde_json::from_value(value)?)
}

pub async fn connect(context: &Context, params: &ConnectParams) -> anyhow::Result<ConnectResponse> {
    post(context.connect_url.clone(), params).await
}

pub async fn disconnect(context: &Context, token: &str) -> anyhow::Result<()> {
    get(context.disconnect_url.clone(), &[("token", token)]).await
}

pub async fn poll(context: &Context, token: &str, p_cursor: &str, len: u32) -> anyhow::Result<()> {
    get(
        context.poll_url.clone(),
        &[
            ("token", token),
            ("pCursor", p_cursor),
            ("len", &len.to_string()),
        ],
    )
    .await
}
