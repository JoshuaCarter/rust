use crate::utils;
use super::health::*;

#[tonic::async_trait]
pub trait HealthServer {
    async fn handle_ping(&self, msg: PingMessage) -> Result<(), tonic::Status>;
}

impl PingMessage {
    pub fn new() -> Self {
        return Self { timestamp: utils::time::now_ms(), };
    }
}
