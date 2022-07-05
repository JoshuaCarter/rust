use std::{net::SocketAddr, str::FromStr, pin::Pin};
use anyhow::Result;
use futures::Stream;
use tonic::{transport::Server, metadata::{MetadataValue, Ascii}, Status, service::Interceptor};
use infra::model::trading::trading_server::*;
use infra::model::market::market_server::*;
use super::trading::TradingService;
use super::market::MarketService;

pub type GrpcStream<T> = Pin<Box<dyn Send + Stream<Item = Result<T, Status>>>>;

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
        .layer(tonic::service::interceptor(interceptor))
        .add_service(TradingServer::new(TradingService::default()))
        .add_service(MarketServer::new(MarketService::default()))
        .serve(addy)
        .await?;

    return Ok(());
}
