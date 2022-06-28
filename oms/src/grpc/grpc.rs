use anyhow::Result;
use std::{net::SocketAddr, str::FromStr};
use tonic::transport::{Server, Endpoint};
use super::services::*;

pub async fn start_server(uri: &str) -> Result<()> {
    let addy = SocketAddr::from_str(uri)?;

    Server::builder()
        .add_service(OrdersGrpc::get_server())
        .serve(addy)
        .await?;

    Ok(())
}

pub async fn start_client(uri: &str) -> Result<()> {
    let endpoint = Endpoint::from_str(uri)?;
    let channel = endpoint.connect().await?;

    let orders = OrdersGrpc::get_client(channel);

    Ok(())
}
