use std::{net::SocketAddr, str::FromStr};
use anyhow::Result;
use infra::model::trading::trading_server::*;
use tonic::transport::Server;
use super::trading::TradingService;

pub async fn start_server(uri: &str) -> Result<()> {
    let addy = SocketAddr::from_str(uri)?;

    Server::builder()
        .add_service(TradingServer::new(TradingService::default()))
        .serve(addy)
        .await?;

    return Ok(());
}
