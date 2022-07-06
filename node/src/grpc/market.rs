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

        let (tx, rx) = mpsc::channel::<Result<BookUpdatesReply, Status>>(1);
        let mut venue = venues::create_venue(req.exchange).map_err(err_to_status)?;

        tokio::spawn(async move {
            loop {
                infra::utils::time::delay(1000).await; // TEMP
                let res = venue.book_updates(req.clone()).await.map_err(err_to_status);
                match res {
                    Ok(r) => {
                        match tx.send(Ok(BookUpdatesReply::from(r))).await {
                            Ok(_) => continue,
                            Err(_) => break, // failed to send to client
                        }
                    },
                    Err(_) => break, // failed to get exchange data
                }
            }
        });

        let output_stream = Box::pin(ReceiverStream::new(rx)) as Self::BookUpdatesStream;
        return Ok(Response::new(output_stream));
    }
}

fn err_to_status<T: ToString>(err: T) -> Status {
    return Status::internal(err.to_string());
}
