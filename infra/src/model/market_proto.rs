use super::{market::*, common::*};

#[tonic::async_trait]
pub trait MarketServer {
    async fn handle_book(&self, call: BookUpdatesCall) -> Result<(), tonic::Status>;
}

#[derive(Debug, Clone)]
pub struct BookUpdatesRequest {
    pub exchange: Exchange,
    pub symbol: Symbol,
}
impl From<BookUpdatesCall> for BookUpdatesRequest {
    fn from(x: BookUpdatesCall) -> Self {
        return BookUpdatesRequest {
            exchange: Exchange::from_i32(x.exchange).unwrap(),
            symbol: x.symbol.unwrap(),
        };
    }
}

#[derive(Debug, Clone)]
pub struct BookUpdatesMessage {
    pub exchange: Exchange,
    pub symbol: Symbol,
    pub asks: Vec<Fill>,
    pub bids: Vec<Fill>,
}
impl From<BookUpdatesReply> for BookUpdatesMessage {
    fn from(x: BookUpdatesReply) -> Self {
        return BookUpdatesMessage {
            exchange: Exchange::from_i32(x.exchange).unwrap(),
            symbol: x.symbol.unwrap(),
            asks: x.asks,
            bids: x.bids,
        };
    }
}
impl From<BookUpdatesMessage> for BookUpdatesReply {
    fn from(x: BookUpdatesMessage) -> Self {
        return BookUpdatesReply {
            exchange: x.exchange as i32,
            symbol: Some(x.symbol.to_owned()),
            asks: x.asks,
            bids: x.bids,
        };
    }
}
