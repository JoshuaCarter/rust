// https://binance-docs.github.io/apidocs/spot/en/#general-api-information

use super::api;
use infra::utils::time;
use infra::net::Params;
use infra::model::common::*;
use infra::model::trading::*;
use anyhow::*;
use async_trait::async_trait;
use reqwest::{Client, RequestBuilder};
use ring::hmac;

#[derive(Debug)]
pub struct Binance {
    http: Client,
    key: String,
    secret: String,
}

impl Binance {
    pub fn new() -> Self {
        return Binance {
            http: Client::new(),
            key: std::env::var("ENV_BINANCE_KEY").unwrap(),
            secret: std::env::var("ENV_BINANCE_SECKEY").unwrap(),
        };
    }

    // pub async fn request_time(&self) -> Result<types::TimeResponse> {
    //     let req = self.http.get(endpoints::TIME);
    //     let res_json = req.send().await?.text().await?;
    //     let res = serde_json::from_str::<types::TimeResponse>(res_json.as_str())?;

    //     return Ok(res);
    // }

    // pub async fn request_account(&self) -> Result<String> {
    //     let mut params = Params::from(vec![]);

    //     let req = self.sign_request(self.http.get(endpoints::ACCOUNT), &mut params);
    //     let res = req.send().await?;
    //     let res_json = res.text().await?;

    //     return Ok(res_json);
    // }

    fn sign_request(&self, request: RequestBuilder, params: &mut Params) -> RequestBuilder {
        params.insert("timestamp", time::now_ms().to_string().as_str());

        let signed_key = hmac::Key::new(hmac::HMAC_SHA256, self.secret.as_bytes());
        let signature = hex::encode(hmac::sign(&signed_key, params.to_string().as_bytes()).as_ref());

        params.insert("signature", signature.as_str());

        return request
            .header("X-MBX-APIKEY", self.key.as_str())
            .query(params.for_query());
    }
}

#[async_trait]
impl TradeHandler for Binance {
    async fn new_order_request(&self, req: NewOrderRequest) -> Result<NewOrderResponse> {
        let api_req = api::HttpNewRequest {
            symbol: req.symbol.to_string_without_delim(),
            type_: req.type_.to_string().to_uppercase(),
            side: req.side.to_buy_sell_string().to_uppercase(),
            timeInForce: req.time_in_force.to_string(),
            price: req.price,
            quantity: req.quantity,
        };
        let mut api_params = Params::from(&serde_json::json!(api_req));

        let http_req = self.sign_request(self.http.post(api::endpoints::ORDER), &mut api_params);
        let http_res = http_req.send().await?;
        let http_txt = http_res.text().await?;

        // TODO: check res status and err 'code'

        let api_res = serde_json::from_str::<api::HttpNewResponse>(http_txt.as_str())
            .with_context(|| format!("Failed to decode response {:?}", http_txt))?;

        let res = NewOrderResponse {
            exchange: req.exchange,
            side: req.side,
            symbol: req.symbol,
            type_: req.type_,
            time_in_force: req.time_in_force,
            order_id: api_res.orderId.to_string(),
            status: adapt_status(api_res.status)?,
            price: api_res.price.parse::<f64>()?,
            quantity: api_res.origQty.parse::<f64>()?,
            executed: api_res.executedQty.parse::<f64>()?,
            fills: adapt_fills(api_res.fills)?,
        };

        return Ok(res);
    }

    async fn cxl_order_request(&self, req: CxlOrderRequest) -> Result<CxlOrderResponse> {
        let api_req = api::HttpCxlRequest {
            symbol: req.symbol.to_string_without_delim(),
            orderId: u64::from_str(req.order_id.as_str())?,
        };
        let mut api_params = Params::from(&serde_json::json!(api_req));

        let http_req = self.sign_request(self.http.delete(api::endpoints::ORDER), &mut api_params);
        let http_res = http_req.send().await?;
        let http_txt = http_res.text().await?;

        // TODO: check res status and err 'code'

        let api_res = serde_json::from_str::<api::HttpCxlResponse>(http_txt.as_str())
            .with_context(|| format!("Failed to decode response {:?}", http_txt))?;

        let res = CxlOrderResponse {
            exchange: req.exchange,
            symbol: req.symbol,
            order_id: api_res.orderId.to_string(),
            status: adapt_status(api_res.status)?,
        };

        return Ok(res);
    }
}

fn adapt_status(status: String) -> Result<Status> {
    match status.as_str() {
        "NEW" | "PARTIALLY_FILLED" => { return Ok(Status::Open); }
        "FILLED" |"CANCELED" | "PENDING_CANCEL" | "REJECTED" | "EXPIRED" => { return Ok(Status::Closed); }
        _ => { return Err(anyhow::anyhow!("Unhandled status '{}'", status)); }
    }
}

fn adapt_fills(fills: Vec<api::Fill>) -> Result<Vec<Fill>> {
    let mut new: Vec<Fill> = Vec::new();

    for f in fills {
        new.push(Fill {
            price: f.price.parse::<f64>()?,
            quantity: f.qty.parse::<f64>()?,
        });
    }

    return Ok(new);
}
