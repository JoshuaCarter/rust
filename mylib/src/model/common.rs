pub use std::str::FromStr;

use std::fmt::Display;
use simple_error::SimpleError;
use strum_macros::{AsRefStr, EnumString, Display as EDisplay};
use lazy_regex::regex_captures;

#[derive(Debug, EnumString, AsRefStr, EDisplay)]
#[strum(ascii_case_insensitive)]
pub enum Exchange {
    Binance,
    FTX,
}

#[derive(Debug, EnumString, AsRefStr, EDisplay)]
#[strum(ascii_case_insensitive)]
pub enum Side {
    Ask,
    Bid
}

impl Side {
    pub fn to_buy_sell_string(&self) -> String {
        match &self {
            Side::Bid => { return String::from("Buy"); }
            Side::Ask => { return String::from("Sell"); }
        }
    }
}

#[derive(Debug, EnumString, AsRefStr, EDisplay)]
#[strum(ascii_case_insensitive)]
pub enum Type {
    Limit,
    Market
}

#[derive(Debug, EnumString, AsRefStr, EDisplay)]
#[strum(ascii_case_insensitive)]
pub enum TimeInForce {
    GTC,
    IOC,
    FOK,
}

#[derive(Debug)]
pub struct Symbol {
    base: String,
    quote: String,
}

impl Symbol {
    pub fn to_string_with_delim(&self, delim: char) -> String {
        return format!("{}{}{}", self.base, delim, self.quote);
    }
    pub fn to_string_without_delim(&self) -> String {
        return format!("{}{}", self.base, self.quote);
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return f.write_fmt(format_args!("{}/{}", self.base, self.quote));
    }
}

impl FromStr for Symbol {
    type Err = SimpleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match regex_captures!(r#"([a-z0-9]+)([^a-z0-9]{1})([a-z0-9]+)"#i, s) {
            Some((_, base, delim, quote)) => {
                println!("STR {}, BASE {}, DELIM {}, QUOTE {}", s, base, delim, quote);
                return Ok(Symbol {
                    base: base.to_uppercase(),
                    quote: quote.to_uppercase(),
                });
            }
            None => {
                return Err(SimpleError::new(
                    format!("Symbol '{}' doesn't have 3 parts {{base}}{{delim}}{{quote}}, e.g. ETH/BTC", s)
                ));
            }
        }
    }
}
