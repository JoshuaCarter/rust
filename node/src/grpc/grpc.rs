use std::{net::SocketAddr, str::FromStr};
use anyhow::Result;
use tokio::sync::mpsc::Sender;
use tonic::{transport::Server, metadata::{MetadataValue, Ascii}, Status, service::Interceptor};
use infra::model::message_stream::{message_stream_server::*, MessageStreamReply};
use super::message_stream::MessageStreamService;

pub type GrpcSender = Sender<Result<MessageStreamReply, Status>>;

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
        .concurrency_limit_per_connection(128)
        .layer(tonic::service::interceptor(interceptor))
        .add_service(MessageStreamServer::new(MessageStreamService::default()))
        .serve(addy)
        .await?;

    return Ok(());
}
