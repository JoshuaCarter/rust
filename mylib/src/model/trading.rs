use super::common::*;

#[derive(Debug)]
pub enum TradeRequest {
    NewOrderRequest(NewOrderRequest),
    CxlOrderRequest(CxlOrderRequest),
}

#[derive(Debug)]
pub struct NewOrderRequest {
    pub exchange: Exchange,
    pub symbol: Symbol,
    pub side: Side,
    pub type_: Type,
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

pub trait TradeRequestHandler {
    fn new_order_request(&self, order: NewOrderRequest);
    fn cxl_order_request(&self, order: NewOrderRequest);
}

// TODO: response types
