#![allow(dead_code)]
#![allow(non_snake_case)]

use serde::Deserialize;
use serde::Serialize;

pub mod endpoints {
    pub const ACCOUNT: &str =   "https://api.binance.com/api/v3/account";
    pub const ORDER: &str =     "https://api.binance.com/api/v3/order";
    pub const TIME: &str =      "https://api.binance.com/api/v3/time";
}

// SHARED TYPES

#[derive(Debug, Deserialize)]
pub struct Fill {
    pub price: String,
    pub qty: String,
    pub commission: String,
    pub commissionAsset: String,
}

// REQUEST TYPES

#[derive(Debug, Serialize)]
pub struct HttpNewRequest {
    pub symbol: String,
    pub side: String,
    // #[serde(rename = "type")]
    pub r#type: String,
    pub quantity: f64,
    pub price: f64,
    pub timeInForce: String,
}

#[derive(Debug, Serialize)]
pub struct HttpCxlRequest {
    pub symbol: String,
    pub orderId: u64,
}

// RESPONSE TYPES

#[derive(Debug, Deserialize)]
pub struct HttpNewResponse {
    pub symbol: String,
    pub orderId: u64,
    pub transactTime: u64,
    pub price: String,
    pub origQty: String,
    pub executedQty: String,
    pub cummulativeQuoteQty: String,
    pub status: String,
    pub timeInForce: String,
    // #[serde(rename = "type")]
    pub r#type: String,
    pub side: String,
    pub fills: Vec<Fill>,
}

#[derive(Debug, Deserialize)]
pub struct HttpCxlResponse {
    pub symbol: String,
    pub orderId: u64,
    pub price: String,
    pub origQty: String,
    pub executedQty: String,
    pub cummulativeQuoteQty: String,
    pub status: String,
    pub timeInForce: String,
    // #[serde(rename = "type")]
    pub r#type: String,
    pub side: String,
}

#[derive(Debug, Deserialize)]
pub struct HttpTimeResponse {
    pub serverTime: u64,
}
