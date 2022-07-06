pub use std::str::FromStr;
pub use crate::error::FromStrError;
pub use std::fmt::Display;

use lazy_regex::regex_captures;

use super::common::*;

// Exchange
impl FromStr for Exchange {
    type Err = FromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "BINANCE" => Ok(Exchange::Binance),
            "FTX" => Ok(Exchange::Ftx),
            _ => Err(FromStrError::new::<Self>(s)),
        }
    }
}
impl Display for Exchange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Exchange::Binance => f.write_str("Binance"),
            Exchange::Ftx => f.write_str("FTX"),
        }
    }
}

// Side
impl FromStr for Side {
    type Err = FromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "ASK" => Ok(Side::Ask),
            "BID" => Ok(Side::Bid),
            _ => Err(FromStrError::new::<Self>(s)),
        }
    }
}
impl Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Side::Ask => f.write_str("Ask"),
            Side::Bid => f.write_str("Bid"),
        }
    }
}
impl Side {
    pub fn to_buy_sell(&self) -> String {
        match self {
            Side::Ask => String::from("Sell"),
            Side::Bid => String::from("Buy"),
        }
    }
}

// Type
impl FromStr for Type {
    type Err = FromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "LIMIT" => Ok(Type::Limit),
            "MARKET" => Ok(Type::Market),
            _ => Err(FromStrError::new::<Self>(s)),
        }
    }
}
impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Limit => f.write_str("Limit"),
            Type::Market => f.write_str("Market"),
        }
    }
}

// TimeInForce
impl FromStr for TimeInForce {
    type Err = FromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "GTC" => Ok(TimeInForce::Gtc),
            "IOC" => Ok(TimeInForce::Ioc),
            "FOK" => Ok(TimeInForce::Fok),
            _ => Err(FromStrError::new::<Self>(s)),
        }
    }
}
impl Display for TimeInForce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeInForce::Gtc => f.write_str("GTC"),
            TimeInForce::Ioc => f.write_str("IOC"),
            TimeInForce::Fok => f.write_str("FOK"),
        }
    }
}

// Status
impl FromStr for Status {
    type Err = FromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "PENDING" => Ok(Status::Pending),
            "OPEN" => Ok(Status::Open),
            "CLOSED" => Ok(Status::Closed),
            _ => Err(FromStrError::new::<Self>(s)),
        }
    }
}
impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Pending => f.write_str("Pending"),
            Status::Open => f.write_str("Open"),
            Status::Closed => f.write_str("Closed"),
        }
    }
}

// Symbol
impl FromStr for Symbol {
    type Err = FromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match regex_captures!(r#"([a-z0-9]+)([^a-z0-9]{1})([a-z0-9]+)"#i, s) {
            Some((_, base, _delim, quote)) => {
                return Ok(Symbol {
                    base: base.to_uppercase(),
                    quote: quote.to_uppercase(),
                });
            }
            None => Err(FromStrError::new::<Self>(s)),
        }
    }
}
impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return f.write_fmt(format_args!("{}/{}", self.base, self.quote));
    }
}
impl Symbol {
    pub fn to_string_with_delim(&self, delim: char) -> String {
        return format!("{}{}{}", self.base, delim, self.quote);
    }
    pub fn to_string_without_delim(&self) -> String {
        return format!("{}{}", self.base, self.quote);
    }
}

// Fill
impl Display for Fill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return f.write_fmt(format_args!("({} @ {})", self.quantity, self.price));
    }
}
