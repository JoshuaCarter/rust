use anyhow::Result;
use infra::model::venue::Venue;
use infra::model::common::Exchange;
use super::Binance;

pub fn create_venue(exchange: Exchange) -> Result<impl Venue> {
    match exchange {
        Exchange::Binance => { return Ok(Binance::new()); },
        _ => { anyhow::bail!("Unsupported exchange") }
    }
}
