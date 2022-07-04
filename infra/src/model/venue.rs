use super::market_proto::{BookUpdatesRequest, BookUpdatesResponse};
use super::trading_proto::{NewOrderRequest, CxlOrderRequest, CxlOrderResponse, NewOrderResponse};
use tokio::sync::mpsc::Sender;
use anyhow::Result;

#[tonic::async_trait]
pub trait Venue: TradingVenue  {}

#[tonic::async_trait]
pub trait TradingVenue {
    async fn new_order(&self, r: NewOrderRequest) -> Result<NewOrderResponse>;
    async fn cxl_order(&self, r: CxlOrderRequest) -> Result<CxlOrderResponse>;
}

#[tonic::async_trait]
pub trait MarketVenue {
    async fn book_updates(&self, r: BookUpdatesRequest, sender: Sender<BookUpdatesResponse>) -> Result<BookUpdatesResponse>;
}
