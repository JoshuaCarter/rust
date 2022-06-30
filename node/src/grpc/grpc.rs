use std::{net::SocketAddr, str::FromStr};
use anyhow::Result;
use proto_types::trading::ProtoServer;
use tonic::transport::Server;
use super::trading::TradingService;

pub async fn start_server(uri: &str) -> Result<()> {
    let addy = SocketAddr::from_str(uri)?;

    Server::builder()
        .add_service(ProtoServer::new(TradingService::default()))
        .serve(addy)
        .await?;

    return Ok(());
}
