use infra::model::venue::MarketVenue;
use tokio::sync::mpsc;
use tonic::Status;
use infra::model::{
    market::*,
    message_stream::*,
};
use crate::venues;
use super::GrpcSender;

#[derive(Debug, Clone)]
pub struct MarketService {
    sender: GrpcSender,
}

impl MarketService {
    pub fn new(sender: GrpcSender) -> Self {
        return MarketService { sender };
    }
}

#[tonic::async_trait]
impl MarketServer for MarketService {
    async fn handle_book(&self, call: BookUpdatesCall) -> Result<(), tonic::Status> {
        let req = BookUpdatesRequest::from(call);
        println!("Book req {:?}", req);

        let mut venue = venues::create_venue(req.exchange).map_err(err_to_status)?;
        let (exch_tx, mut exch_rx) = mpsc::channel::<BookUpdatesMessage>(1);

        infra::spawn!((self.sender => sender) => {
            // listen for venue messages
            while let Some(res) = exch_rx.recv().await {
                // send to client
                let reply = MessageStreamReply::from(BookUpdatesReply::from(res));
                match sender.send(Ok(reply)).await {
                    Ok(_) => continue,
                    Err(_) => {
                        println!("Book disconnected @ {}", infra::utils::time::now_ms());
                        break;
                    },
                }
            }
        });

        infra::spawn!(() => {
            // run venue, generates message
            venue.book_updates(req, exch_tx).await.unwrap();
        });

        Ok(())
    }
}

fn err_to_status<T: ToString>(err: T) -> Status {
    return Status::internal(err.to_string());
}
