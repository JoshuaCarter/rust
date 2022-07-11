use super::{trading::*, common::*};

#[tonic::async_trait]
pub trait TradingServer {
    async fn handle_new(&self, call: NewOrderCall) -> Result<(), tonic::Status>;
    async fn handle_cxl(&self, call: CxlOrderCall) -> Result<(), tonic::Status>;
}

#[derive(Debug, Clone)]
pub struct NewOrderRequest {
    pub exchange: Exchange,
    pub symbol: Symbol,
    pub side: Side,
    pub r#type: Type,
    pub quantity: f64,
    pub price: f64,
    pub time_in_force: TimeInForce,
}
impl From<NewOrderCall> for NewOrderRequest {
    fn from(x: NewOrderCall) -> Self {
        return NewOrderRequest {
            exchange: Exchange::from_i32(x.exchange).unwrap(),
            symbol: x.symbol.unwrap(),
            side: Side::from_i32(x.side).unwrap(),
            r#type: Type::from_i32(x.r#type).unwrap(),
            quantity: x.quantity,
            price: x.price,
            time_in_force: TimeInForce::from_i32(x.time_in_force).unwrap(),
        };
    }
}

#[derive(Debug, Clone)]
pub struct NewOrderResponse {
    pub order_id: String,
    pub exchange: Exchange,
    pub symbol: Symbol,
    pub side: Side,
    pub status: Status,
    pub r#type: Type,
    pub quantity: f64,
    pub price: f64,
    pub executed: f64,
    pub time_in_force: TimeInForce,
    pub fills: Vec<Fill>,
}
impl From<NewOrderReply> for NewOrderResponse {
    fn from(x: NewOrderReply) -> Self {
        return NewOrderResponse {
            order_id: x.order_id,
            exchange: Exchange::from_i32(x.exchange).unwrap(),
            symbol: x.symbol.unwrap(),
            side: Side::from_i32(x.side).unwrap(),
            status: Status::from_i32(x.status).unwrap(),
            r#type: Type::from_i32(x.r#type).unwrap(),
            quantity: x.quantity,
            price: x.price,
            executed: x.executed,
            time_in_force: TimeInForce::from_i32(x.time_in_force).unwrap(),
            fills: x.fills,
        };
    }
}
impl From<NewOrderResponse> for NewOrderReply {
    fn from(x: NewOrderResponse) -> Self {
        return NewOrderReply {
            order_id: x.order_id,
            exchange: x.exchange as i32,
            symbol: Some(x.symbol.to_owned()),
            side: x.side as i32,
            r#type: x.r#type as i32,
            time_in_force: x.time_in_force as i32,
            status: x.status as i32,
            price: x.price,
            quantity: x.quantity,
            executed: x.executed,
            fills: x.fills,
        };
    }
}

#[derive(Debug, Clone)]
pub struct CxlOrderRequest {
    pub order_id: String,
    pub exchange: Exchange,
    pub symbol: Symbol,
}
impl From<CxlOrderCall> for CxlOrderRequest {
    fn from(x: CxlOrderCall) -> Self {
        return CxlOrderRequest {
            order_id: x.order_id,
            exchange: Exchange::from_i32(x.exchange).unwrap(),
            symbol: x.symbol.unwrap(),
        };
    }
}

#[derive(Debug, Clone)]
pub struct CxlOrderResponse {
    pub order_id: String,
    pub exchange: Exchange,
    pub symbol: Symbol,
    pub status: Status,
}
impl From<CxlOrderReply> for CxlOrderResponse {
    fn from(x: CxlOrderReply) -> Self {
        return CxlOrderResponse {
            order_id: x.order_id,
            exchange: Exchange::from_i32(x.exchange).unwrap(),
            symbol: x.symbol.unwrap(),
            status: Status::from_i32(x.status).unwrap(),
        };
    }
}
impl From<CxlOrderResponse> for CxlOrderReply {
    fn from(x: CxlOrderResponse) -> Self {
        return CxlOrderReply {
            order_id: x.order_id,
            exchange: x.exchange as i32,
            symbol: Some(x.symbol.to_owned()),
            status: x.status as i32,
        };
    }
}
