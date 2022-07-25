mod model;

use anyhow::bail;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::{ConnectParams, ConnectResponse};

async fn post<Req: Serialize, Resp: DeserializeOwned>(
    url: &str,
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

pub async fn connect(params: &ConnectParams) -> anyhow::Result<ConnectResponse> {
    post(
        "https://open-interaction.game.kuaishou.com/openapi/sdk/v1/connect",
        params,
    )
    .await
}
