use super::market_proto::{BookUpdatesRequest, BookUpdatesMessage};
use super::trading_proto::{NewOrderRequest, CxlOrderRequest, CxlOrderResponse, NewOrderResponse};
use anyhow::Result;
use tokio::sync::mpsc::Sender;

#[tonic::async_trait]
pub trait Venue: TradingVenue + MarketVenue {}

#[tonic::async_trait]
pub trait TradingVenue {
    async fn new_order(&mut self, r: NewOrderRequest) -> Result<NewOrderResponse>;
    async fn cxl_order(&mut self, r: CxlOrderRequest) -> Result<CxlOrderResponse>;
}

#[tonic::async_trait]
pub trait MarketVenue {
    async fn book_updates(&mut self, r: BookUpdatesRequest, tx: Sender<BookUpdatesMessage>) -> Result<()>;
}
