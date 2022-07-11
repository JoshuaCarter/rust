use tonic::Status;
use infra::model::{
    venue::*,
    trading::*,
    message_stream::MessageStreamReply,
};
use crate::venues;
use super::GrpcSender;

#[derive(Debug, Clone)]
pub struct TradingService {
    sender: GrpcSender,
}

impl TradingService {
    pub fn new(sender: GrpcSender) -> Self {
        return TradingService { sender };
    }
}

#[tonic::async_trait]
impl TradingServer for TradingService {
    async fn handle_new(&self, call: NewOrderCall) -> Result<(), tonic::Status> {
        let req = NewOrderRequest::from(call);
        println!("New req {:?}", req);

        let mut venue = venues::create_venue(req.exchange).map_err(err_to_status)?;
        let res = venue.new_order(req).await.map_err(err_to_status)?;

        let reply = MessageStreamReply::from(NewOrderReply::from(res));
        println!("send new res");
        self.sender.send(Ok(reply)).await.map_err(err_to_status)?;

        return Ok(());
    }

    async fn handle_cxl(&self, call: CxlOrderCall) -> Result<(), tonic::Status> {
        let req = CxlOrderRequest::from(call);
        println!("Cxl req {:?}", req);

        let mut venue = venues::create_venue(req.exchange).map_err(err_to_status)?;
        let res = venue.cxl_order(req).await.map_err(err_to_status)?;

        let reply = MessageStreamReply::from(CxlOrderReply::from(res));
        println!("send cxl res");
        self.sender.send(Ok(reply)).await.map_err(err_to_status)?;

        return Ok(());
    }
}

fn err_to_status<T: ToString>(err: T) -> Status {
    return Status::internal(err.to_string());
}
