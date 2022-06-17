use simple_error::*;

#[derive(Debug)]
pub enum Exchange {
    FTX,
    Binance
}

impl Exchange {
    pub fn to_string(&self) -> &'static str {
        match self {
            Exchange::FTX => { return "FTX"; }
            Exchange::Binance => { return "Binance"; }
        }
    }
    pub fn from_string(val: &String) -> Result<Exchange, SimpleError> {
        match val.as_str() {
            "FTX" => { return Ok(Exchange::FTX); }
            "Binance" => { return Ok(Exchange::Binance); }
            _ => { return Err(SimpleError::new(format!("Failed to parse {} into an Exchange", val))); }
        }
    }
}

#[derive(Debug)]
pub enum Side {
    Ask,
    Bid
}

#[derive(Debug)]
pub struct Order {
    exchange: Exchange,
    market: String,
    quantity: String,
    price: String,
    side: Side,
}

impl Order {
    pub fn new(
        exchange: Exchange,
        market: &String,
        quantity: &String,
        price: &String,
        side: Side,
) -> Order {
        return Order {
            exchange,
            market: market.to_string(),
            quantity: quantity.to_string(),
            price: price.to_string(),
            side,
        };
    }
}
