#[allow(dead_code)]
pub mod endpoints {
    pub const ACCOUNT: &str =       "https://api.binance.com/api/v3/account";
    pub const ORDER: &str =         "https://api.binance.com/api/v3/order";
    pub const TIME: &str =          "https://api.binance.com/api/v3/time";
}

#[allow(dead_code)]
#[allow(non_snake_case)]
pub mod types {
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Debug, Serialize)]
    pub struct NewOrderRequest {
        pub symbol: String,
        pub side: String,
        #[serde(rename = "type")]
        pub type_: String,
        pub quantity: f64,
        pub price: f64,
        pub timeInForce: String,
    }

    #[derive(Debug, Serialize)]
    pub struct CxlOrderRequest {
        pub symbol: String,
        pub orderId: u64,
    }

    #[derive(Debug, Deserialize)]
    pub struct TimeResponse {
        pub serverTime: u64,
    }

    #[derive(Debug, Deserialize)]
    pub struct Fill {
        pub price: String,
        pub qty: String,
        pub commission: String,
        pub commissionAsset: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct NewOrderResponse {
        pub symbol: String,
        pub orderId: u64,
        pub transactTime: u64,
        pub price: String,
        pub origQty: String,
        pub executedQty: String,
        pub cummulativeQuoteQty: String,
        pub status: String,
        pub timeInForce: String,
        #[serde(rename = "type")]
        pub type_: String,
        pub side: String,
        pub fills: Vec<Fill>,
    }

    #[derive(Debug, Deserialize)]
    pub struct CxlOrderResponse {
        pub symbol: String,
        pub orderId: u64,
        pub price: String,
        pub origQty: String,
        pub executedQty: String,
        pub cummulativeQuoteQty: String,
        pub status: String,
        pub timeInForce: String,
        #[serde(rename = "type")]
        pub type_: String,
        pub side: String,
    }
}
