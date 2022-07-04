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

        let (tx, rx) = mpsc::channel::<Result<BookUpdatesReply, Status>>(128);
        let venue = venues::create_venue(req.exchange).map_err(err_to_status)?;
        // let res = venue.book_updates(req, tx).await;

        let output_stream = ReceiverStream::new(rx);
        // let mapped = output_stream.map(|f| -> GrpcResult<BookUpdatesReply> {
        //     match f {
        //         Ok(x) => Ok(Response::new(BookUpdatesReply::from(x))),
        //         Err(e) => Err(Status::internal(e.to_string())),
        //     }
        // });

        let stream = Box::pin(output_stream);
        return Ok(Response::new(stream));
    }
}

fn err_to_status<T: ToString>(err: T) -> Status {
    return Status::internal(err.to_string());
}
