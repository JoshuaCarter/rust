// https://binance-docs.github.io/apidocs/spot/en/#general-api-information

use crate::utils::time;
use crate::net::Params;
use super::api::*;
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

    pub async fn request_time(&self) -> Result<types::TimeResponse, reqwest::Error> {
        let req = self.http.get(endpoints::TIME);
        let res_json = req.send().await?.text().await?;
        println!("RES = {}", res_json);
        let res = serde_json::from_str::<types::TimeResponse>(res_json.as_str()).unwrap();

        return Ok(res);
    }

    pub async fn request_account(&self) -> Result<String, reqwest::Error> {
        let mut params = Params::from(vec![]);

        let req = self.sign_request(self.http.get(endpoints::ACCOUNT), &mut params);
        let res = req.send().await?;
        let res_json = res.text().await?;

        return Ok(res_json);
    }

    pub async fn request_new_order(&self) -> Result<types::NewOrderResponse, reqwest::Error> {
        let body = types::NewOrderRequest {
            symbol: String::from("ETHUSDT"),
            type_: String::from("LIMIT"),
            side: String::from("BUY"),
            timeInForce: String::from("GTC"),
            price: 250.0,
            quantity: 0.04,
        };

        let mut params = Params::from(&serde_json::json!(body));
        let req = self.sign_request(self.http.post(endpoints::ORDER), &mut params);
        let res = req.send().await?;
        let txt = res.text().await?;
        let order = serde_json::from_str::<types::NewOrderResponse>(txt.as_str()).unwrap();

        return Ok(order);
    }

    pub async fn request_cxl_order(&self, id: u64) -> Result<types::CxlOrderResponse, reqwest::Error> {
        let body = types::CxlOrderRequest {
            symbol: String::from("ETHUSDT"),
            orderId: id,
        };

        let mut params = Params::from(&serde_json::json!(body));
        let req = self.sign_request(self.http.delete(endpoints::ORDER), &mut params);
        let res = req.send().await?;
        let txt = res.text().await?;
        let order = serde_json::from_str::<types::CxlOrderResponse>(txt.as_str());

        return Ok(order.unwrap());
    }

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
