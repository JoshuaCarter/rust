
use super::{market::*, common::*};

#[derive(Debug)]
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

#[derive(Debug)]
pub struct BookUpdatesResponse {
    pub exchange: Exchange,
    pub symbol: Symbol,
    pub asks: Vec<Fill>,
    pub bids: Vec<Fill>,
}
impl From<BookUpdatesReply> for BookUpdatesResponse {
    fn from(x: BookUpdatesReply) -> Self {
        return BookUpdatesResponse {
            exchange: Exchange::from_i32(x.exchange).unwrap(),
            symbol: x.symbol.unwrap(),
            asks: x.asks,
            bids: x.bids,
        };
    }
}
impl From<BookUpdatesResponse> for BookUpdatesReply {
    fn from(x: BookUpdatesResponse) -> Self {
        return BookUpdatesReply {
            exchange: x.exchange as i32,
            symbol: Some(x.symbol.to_owned()),
            asks: x.asks,
            bids: x.bids,
        };
    }
}
