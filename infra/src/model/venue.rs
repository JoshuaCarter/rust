use super::market_proto::{BookUpdatesRequest, BookUpdatesResponse};
use super::trading_proto::{NewOrderRequest, CxlOrderRequest, CxlOrderResponse, NewOrderResponse};
use anyhow::Result;

#[tonic::async_trait]
pub trait Venue: TradingVenue + MarketVenue {}

#[tonic::async_trait]
pub trait TradingVenue {
    async fn new_order(&mut self, r: NewOrderRequest) -> Result<NewOrderResponse>;
    async fn cxl_order(&mut self, r: CxlOrderRequest) -> Result<CxlOrderResponse>;
}

#[tonic::async_trait]
pub trait MarketVenue {
    async fn book_updates(&mut self, r: BookUpdatesRequest) -> Result<BookUpdatesResponse>;
}
