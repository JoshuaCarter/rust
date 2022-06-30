use super::common::*;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait TradeHandler {
    async fn new_order_request(&self, order: NewOrderRequest) -> Result<NewOrderResponse>;
    async fn cxl_order_request(&self, order: CxlOrderRequest) -> Result<CxlOrderResponse>;
}

// REQUEST TYPES

#[derive(Debug)]
pub struct NewOrderRequest {
    pub exchange: Exchange,
    pub symbol: Symbol,
    pub side: Side,
    pub r#type: Type,
    pub quantity: f64,
    pub price: f64,
    pub time_in_force: TimeInForce,
}

#[derive(Debug)]
pub struct CxlOrderRequest {
    pub exchange: Exchange,
    pub symbol: Symbol,
    pub order_id: String,
}

// RESPONSE TYPES

#[derive(Debug)]
pub struct NewOrderResponse {
    pub order_id: String,
    pub exchange: Exchange,
    pub symbol: Symbol,
    pub status: Status,
    pub side: Side,
    pub r#type: Type,
    pub price: f64,
    pub quantity: f64,
    pub executed: f64,
    pub fills: Vec<Fill>,
    pub time_in_force: TimeInForce,
}

#[derive(Debug)]
pub struct CxlOrderResponse {
    pub exchange: Exchange,
    pub symbol: Symbol,
    pub order_id: String,
    pub status: Status,
}
