use std::str::FromStr;
use anyhow::Result;
use infra::model::trading::trading_client::TradingClient;
use tonic::transport::{Channel, Endpoint};

pub struct GrpcClient {
    pub trading: TradingClient<Channel>,
}

pub async fn start_client(uri: &str) -> Result<GrpcClient> {
    let endpoint = Endpoint::from_str(uri)?;
    let channel = endpoint.connect().await?;

    return Ok(GrpcClient {
        trading: TradingClient::new(channel),
    });
}
