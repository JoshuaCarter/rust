use infra::model::venue::MarketVenue;
use tokio::sync::mpsc;
use tonic::{
    Request,
    Response,
    Status,
};
use infra::model::market::*;
use infra::model::market::market_server::Market;
use tokio_stream::wrappers::ReceiverStream;
use crate::venues;
use super::GrpcStream;

#[derive(Debug, Default)]
pub struct MarketService {}

#[tonic::async_trait]
impl Market for MarketService {
    type BookUpdatesStream = GrpcStream<BookUpdatesReply>;

    async fn book_updates(&self, request: Request<BookUpdatesCall>) -> Result<Response<Self::BookUpdatesStream>, Status> {
        let req = BookUpdatesRequest::from(request.into_inner());
        println!("BOOK REQ: {:#?}", req);

        let (grpc_tx, grpc_rx) = mpsc::channel::<Result<BookUpdatesReply, Status>>(1);
        let mut venue = venues::create_venue(req.exchange).map_err(err_to_status)?;

        let (exch_tx, mut exch_rx) = mpsc::channel::<BookUpdatesMessage>(1);

        tokio::spawn(async move {
            // await venue msg
            while let Some(res) = exch_rx.recv().await {
                // send to client
                match grpc_tx.send(Ok(BookUpdatesReply::from(res))).await {
                    Ok(_) => continue,
                    Err(_) => { println!("BOOK DISCON @ {}", infra::utils::time::now_ms()); break; }, // failed to send to client
                }
            }
        });

        tokio::spawn(async move {
            // run venue, sends msgs to other thread
            venue.book_updates(req, exch_tx).await.unwrap();
        });

        let output_stream = Box::pin(ReceiverStream::new(grpc_rx)) as Self::BookUpdatesStream;
        return Ok(Response::new(output_stream));
    }
}

fn err_to_status<T: ToString>(err: T) -> Status {
    return Status::internal(err.to_string());
}
