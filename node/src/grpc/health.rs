use infra::model::message_stream::MessageStreamReply;
use infra::model::health::*;
use super::GrpcSender;

#[derive(Debug, Clone)]
pub struct HealthService {
    sender: GrpcSender,
}
impl HealthService {
    pub fn new(sender: GrpcSender) -> Self {
        return HealthService { sender };
    }
}

#[tonic::async_trait]
impl HealthServer for HealthService {
    async fn handle_ping(&self, msg: PingMessage) -> Result<(), tonic::Status> {
        infra::utils::time::delay(10000).await; // TEMP?
        match self.sender.send(Ok(MessageStreamReply::from(PingMessage::new()))).await {
            Ok(_) => { return Ok(()); },
            Err(e) => {
                println!("Ping failed due to {}", e);
                return Err(tonic::Status::internal(e.to_string()));
            },
        }
    }
}
