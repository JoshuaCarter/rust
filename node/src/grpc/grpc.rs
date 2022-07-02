use std::{net::SocketAddr, str::FromStr};
use anyhow::Result;
use infra::model::trading::trading_server::*;
use tonic::{transport::Server, metadata::{MetadataValue, Ascii}, Status, service::Interceptor};
use super::trading::TradingService;

#[derive(Clone)]
pub struct GrpcInterceptor {
    token: MetadataValue<Ascii>,
}

impl Interceptor for GrpcInterceptor {
    fn call(&mut self, req: tonic::Request<()>) -> Result<tonic::Request<()>, Status> {
        if let Some(t) = req.metadata().get("authorization") {
            if self.token == t {
                return Ok(req);
            }
        }
        return Err(Status::unauthenticated("No valid auth token"));
    }
}

pub async fn start_server(uri: &str) -> Result<()> {
    let addy = SocketAddr::from_str(uri)?;
    let interceptor = GrpcInterceptor {
        token: "TOKEN".parse()?,
    };

    Server::builder()
        .add_service(TradingServer::with_interceptor(TradingService::default(), interceptor))
        .serve(addy)
        .await?;

    return Ok(());
}
