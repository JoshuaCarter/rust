use std::pin::Pin;
use futures::Stream;
use tonic::{
    Request,
    Response,
    Status,
    Streaming,
};
use infra::model::{
    message_stream::*,
    message_stream::message_stream_server::MessageStream,
    health::*,
    trading::*,
    market::*,
};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use super::{
    health::*,
    market::*,
    trading::*,
};

#[derive(Debug, Default)]
pub struct MessageStreamService {}

#[tonic::async_trait]
impl MessageStream for MessageStreamService {
    type MessageStream = Pin<Box<dyn Send + Stream<Item = Result<MessageStreamReply, Status>>>>;

    async fn message(&self, request: Request<Streaming<MessageStreamCall>>) -> Result<Response<Self::MessageStream>, Status> {
        // make mpsc stream
        let (grpc_tx, grpc_rx) = mpsc::channel::<Result<MessageStreamReply, Status>>(1);

        infra::spawn!((grpc_tx) => {
            let health_service = HealthService::new(grpc_tx.clone());
            let trading_service = TradingService::new(grpc_tx.clone());
            let market_service = MarketService::new(grpc_tx.clone());
            let mut client_stream = request.into_inner();

            // listen for messages
            while let msg = client_stream.message().await {
                match msg {
                    Ok(Some(rep)) => {
                        // pass to handler
                        if let Some(new) = rep.new {
                            println!("Got new {:?}", new);
                            infra::spawn!((trading_service) => {
                                trading_service.handle_new(new).await;
                            });
                        }
                        if let Some(cxl) = rep.cxl {
                            println!("Got cxl {:?}", cxl);
                            infra::spawn!((trading_service) => {
                                trading_service.handle_cxl(cxl).await;
                            });
                        }
                        if let Some(book) = rep.book {
                            println!("Got book {:?}", book);
                            infra::spawn!((market_service) => {
                                market_service.handle_book(book).await;
                            });
                        }
                        if let Some(ping) = rep.ping {
                            println!("Got ping");
                            infra::spawn!((health_service) => {
                                health_service.handle_ping(ping).await;
                            });
                        }
                    }
                    // here is where we find out the connection to the client has closed (without needing to attempt a send)
                    Ok(None) => { println!("CONNECTION CLOSED"); break; }
                    // this will be called first if connection wasn't gracefully closed
                    Err(e) => { println!("CONNECTION CLOSED with error {}", e); }
                }
            }
        });

        // send stream reciever to client
        let output_stream = Box::pin(ReceiverStream::new(grpc_rx));
        return Ok(Response::new(output_stream));
    }
}
