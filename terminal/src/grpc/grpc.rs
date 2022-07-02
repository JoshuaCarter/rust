use std::str::FromStr;
use anyhow::Result;
use infra::model::trading::trading_client::TradingClient;
use tonic::{transport::{Channel, Endpoint}, metadata::{MetadataValue, Ascii}, codegen::InterceptedService, Status, service::Interceptor};

pub struct GrpcClient {
    pub trading: TradingClient<InterceptedService<Channel, GrpcInterceptor>>,
}

pub struct GrpcInterceptor {
    token: MetadataValue<Ascii>,
}

impl Interceptor for GrpcInterceptor {
    fn call(&mut self, mut req: tonic::Request<()>) -> Result<tonic::Request<()>, Status> {
        req.metadata_mut().insert("authorization", self.token.clone());
        return Ok(req);
    }
}

pub async fn start_client(uri: &str) -> Result<GrpcClient> {
    let endpoint = Endpoint::from_str(uri)?;
    let channel = endpoint.connect().await?;
    let interceptor = GrpcInterceptor {
        token: "TOKEN".parse()?,
    };

    return Ok(GrpcClient {
        trading: TradingClient::with_interceptor(channel, interceptor),
    });
}
