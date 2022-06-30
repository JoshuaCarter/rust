use anyhow::Result;
use infra::model::{venue::Venue};
use proto_types::common::Exchange;
use crate::Binance;

pub fn create_venue(exchange: &Exchange) -> Result<Box<dyn Venue>> {
    match exchange {
        Exchange::Binance => { return Ok(Box::new(Binance::new())); },
        _ => { anyhow::bail!("Unsupported exchange") }
    }
}
