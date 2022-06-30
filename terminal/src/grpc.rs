use std::str::FromStr;
use anyhow::Result;
use proto_types::trading;
use tonic::transport::{Channel, Endpoint};

pub struct GrpcClient {
    pub trading: trading::ProtoClient<Channel>,
}

pub async fn start_client(uri: &str) -> Result<GrpcClient> {
    let endpoint = Endpoint::from_str(uri)?;
    let channel = endpoint.connect().await?;

    return Ok(GrpcClient {
        trading: trading::ProtoClient::new(channel),
    });
}
