use infra::model::health::*;
use infra::model::health::health_server::Health;
use tokio::sync::mpsc;
use tonic::{
    Request,
    Response,
    Status,
    Streaming,
};
use tokio_stream::wrappers::ReceiverStream;
use super::GrpcStream;

#[derive(Debug, Default)]
pub struct HealthService {}

#[tonic::async_trait]
impl Health for HealthService {
    type PingStream = GrpcStream<PingPong>;

    async fn ping(&self, reqeust: Request<Streaming<PingPong>>) -> Result<Response<Self::PingStream>, Status> {
        let mut req = reqeust.into_inner();

        let (grpc_tx, grpc_rx) = mpsc::channel::<Result<PingPong, Status>>(1);

        tokio::spawn(async move {
            // ping the pongs
            while let Ok(Some(ping)) = req.message().await {
                // send to client
                let pong = PingPong { sequence: ping.sequence + 1 };
                match grpc_tx.send(Ok(pong)).await {
                    Ok(_) => { infra::utils::time::delay(100).await; },
                    Err(_) => { println!("PING DISCON @ {}", infra::utils::time::now_ms()); break; }, // failed to send to client
                }
            }
        });

        let output_stream = Box::pin(ReceiverStream::new(grpc_rx)) as Self::PingStream;
        return Ok(Response::new(output_stream));
    }
}
