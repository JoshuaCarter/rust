// https://binance-docs.github.io/apidocs/spot/en/#general-api-information

use super::api;
use super::adapter;
use std::str::FromStr;
use infra::utils::time;
use infra::net::Params;
use infra::model::trading::*;
use infra::model::market::*;
use infra::model::venue::*;
use anyhow::Result;
use reqwest::{Client, RequestBuilder};
use ring::hmac;
use tokio::sync::mpsc::Sender;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures::{StreamExt, SinkExt};

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

impl Venue for Binance {}

#[tonic::async_trait]
impl TradingVenue for Binance {
    async fn new_order(&mut self, req: NewOrderRequest) -> Result<NewOrderResponse> {
        let api_req = api::HttpNewRequest {
            symbol: req.symbol.to_string_without_delim(),
            r#type: req.r#type.to_string().to_uppercase(),
            side: req.side.to_buy_sell().to_uppercase(),
            timeInForce: req.time_in_force.to_string(),
            price: req.price,
            quantity: req.quantity,
        };
        let mut api_params = Params::from(&serde_json::json!(api_req));

        let http_req = self.sign_request(self.http.post(api::endpoints::ORDER), &mut api_params);
        let http_res = http_req.send().await?;
        let http_txt = http_res.text().await?;

        // TODO: check res status and err 'code'

        let api_res = serde_json::from_str::<api::HttpNewResponse>(http_txt.as_str())?;

        let res = NewOrderResponse {
            order_id: api_res.orderId.to_string(),
            exchange: req.exchange,
            symbol: req.symbol,
            side: req.side,
            status: adapter::str_to_status(api_res.status)?,
            r#type: req.r#type,
            time_in_force: req.time_in_force,
            price: api_res.price.parse::<f64>()?,
            quantity: api_res.origQty.parse::<f64>()?,
            executed: api_res.executedQty.parse::<f64>()?,
            fills: adapter::fills_to_fills(api_res.fills)?,
        };

        return Ok(res);
    }

    async fn cxl_order(&mut self, req: CxlOrderRequest) -> Result<CxlOrderResponse> {
        let api_req = api::HttpCxlRequest {
            symbol: req.symbol.to_string_without_delim(),
            orderId: u64::from_str(req.order_id.as_str())?,
        };
        let mut api_params = Params::from(&serde_json::json!(api_req));

        let http_req = self.sign_request(self.http.delete(api::endpoints::ORDER), &mut api_params);
        let http_res = http_req.send().await?;
        let http_txt = http_res.text().await?;

        // TODO: check res status and err 'code'

        let api_res = serde_json::from_str::<api::HttpCxlResponse>(http_txt.as_str())?;

        let res = CxlOrderResponse {
            order_id: api_res.orderId.to_string(),
            exchange: req.exchange,
            symbol: req.symbol,
            status: adapter::str_to_status(api_res.status)?,
        };

        return Ok(res);
    }
}

#[tonic::async_trait]
impl MarketVenue for Binance {
    async fn book_updates(&mut self, req: BookUpdatesRequest, tx: Sender<BookUpdatesMessage>) -> Result<()> {
        // connect sock
        let (socket, _) = connect_async(url::Url::parse(api::endpoints::WEBSOCKET)?).await?;
        let (mut sender, mut receiver) = socket.split();

        // sub to book
        let sock_req = api::SockDepthStreamRequest {
            method: api::SockMethods::SUBSCRIBE.into(),
            params: vec![format!("{}@depth5", req.symbol.to_string_without_delim().to_lowercase())],
            id: 0,
        };
        sender.send(Message::Text(serde_json::to_string(&sock_req)?)).await?;

        // listen for book updates
        while let Some(Ok(next)) = receiver.next().await {
            let text = next.into_text()?;

            let sock_msg = serde_json::from_str::<api::SockDepthStreamMessage>(text.as_str());
            match sock_msg {
                Ok(msg) => {

                    let res = BookUpdatesMessage {
                        exchange: req.exchange,
                        symbol: req.symbol.clone(),
                        asks: adapter::strs_to_fills(msg.asks).unwrap(),
                        bids: adapter::strs_to_fills(msg.bids).unwrap(),
                    };

                    // pass msg to caller
                    match tx.send(res).await {
                        Ok(_) => {}
                        Err(_) => { println!("listener gone"); break; }
                    }
                }
                Err(e) => { println!("failed to decode... {}, err {:#?}", text, e); }
            }
        }

        // close sock
        sender.close().await?;
        return Ok(());
    }
}
