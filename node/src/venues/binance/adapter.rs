// https://binance-docs.github.io/apidocs/spot/en/#general-api-information

use super::api;
use infra::model::common::*;
use anyhow::Result;

pub fn str_to_status(status: String) -> Result<Status> {
    match status.as_str() {
        "NEW" | "PARTIALLY_FILLED" => { return Ok(Status::Open); }
        "FILLED" |"CANCELED" | "PENDING_CANCEL" | "REJECTED" | "EXPIRED" => { return Ok(Status::Closed); }
        _ => { return Err(anyhow::anyhow!("Unhandled status '{}'", status)); }
    }
}

pub fn fills_to_fills(fills: Vec<api::Fill>) -> Result<Vec<Fill>> {
    let mut new: Vec<Fill> = Vec::new();

    for f in fills {
        new.push(Fill {
            price: f.price.parse::<f64>()?,
            quantity: f.qty.parse::<f64>()?,
        });
    }

    return Ok(new);
}

pub fn strs_to_fills(fills: Vec<Vec<String>>) -> Result<Vec<Fill>> {
    let mut new: Vec<Fill> = Vec::new();

    for f in fills {
        new.push(Fill {
            price: f[0].parse::<f64>()?,
            quantity: f[1].parse::<f64>()?,
        });
    }

    return Ok(new);
}
