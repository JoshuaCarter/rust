// https://binance-docs.github.io/apidocs/spot/en/#general-api-information

use super::api;
use infra::{utils::time, model::venue::Venue};
use infra::net::Params;
use infra::model::common::*;
use anyhow::Result;
use reqwest::{Client, RequestBuilder};
use ring::hmac;
use proto_types::trading::*;

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
impl ProtoService for Binance {
    async fn new_order(&self, request: tonic::Request<NewRequest>) -> Result<tonic::Response<NewReply>, tonic::Status> {
        let req = request.into_inner();
        let api_req = api::HttpNewRequest {
            symbol: req.symbol.to_owned(),
            r#type: req.r#type.to_string().to_uppercase(),
            side: req.side.to_uppercase(),
            timeInForce: req.time_in_force.to_string(),
            price: req.price,
            quantity: req.quantity,
        };
        let mut api_params = Params::from(&serde_json::json!(api_req));

        let http_req = self.sign_request(self.http.post(api::endpoints::ORDER), &mut api_params);
        let http_res = http_req.send().await.map_err(err_to_status)?;
        let http_txt = http_res.text().await.map_err(err_to_status)?;

        // TODO: check res status and err 'code'

        let api_res = serde_json::from_str::<api::HttpNewResponse>(http_txt.as_str())
            .map_err(err_to_status)?;

        let res = NewReply {
            exchange: req.exchange,
            side: req.side,
            symbol: req.symbol.to_owned(),
            r#type: req.r#type,
            time_in_force: req.time_in_force,
            order_id: api_res.orderId.to_string(),
            status: api_res.status,
            price: api_res.price.parse::<f64>().map_err(err_to_status)?,
            quantity: api_res.origQty.parse::<f64>().map_err(err_to_status)?,
            executed: api_res.executedQty.parse::<f64>().map_err(err_to_status)?,
            fills: adapt_fills(api_res.fills).map_err(err_to_status)?,
        };

        return Ok(tonic::Response::new(res));
    }

    async fn cxl_order(&self, request: tonic::Request<CxlRequest>) -> Result<tonic::Response<CxlReply>, tonic::Status>{
        let req = request.into_inner();
        let api_req = api::HttpCxlRequest {
            symbol: req.symbol.to_owned(),
            orderId: u64::from_str(req.order_id.as_str()).map_err(err_to_status)?,
        };
        let mut api_params = Params::from(&serde_json::json!(api_req));

        let http_req = self.sign_request(self.http.delete(api::endpoints::ORDER), &mut api_params);
        let http_res = http_req.send().await.map_err(err_to_status)?;
        let http_txt = http_res.text().await.map_err(err_to_status)?;

        // TODO: check res status and err 'code'

        let api_res = serde_json::from_str::<api::HttpCxlResponse>(http_txt.as_str()).map_err(err_to_status)?;

        let res = CxlReply {
            exchange: req.exchange,
            symbol: req.symbol.to_owned(),
            order_id: api_res.orderId.to_string(),
            status: api_res.status,
        };

        return Ok(tonic::Response::new(res));
    }
}

// #[async_trait]
// impl TradeHandler for Binance {
//     async fn new_order_request(&self, req: NewOrderRequest) -> Result<NewOrderResponse> {
//         let api_req = api::HttpNewRequest {
//             symbol: req.symbol.to_string_without_delim(),
//             r#type: req.r#type.to_string().to_uppercase(),
//             side: req.side.to_buy_sell_string().to_uppercase(),
//             timeInForce: req.time_in_force.to_string(),
//             price: req.price,
//             quantity: req.quantity,
//         };
//         let mut api_params = Params::from(&serde_json::json!(api_req));

//         let http_req = self.sign_request(self.http.post(api::endpoints::ORDER), &mut api_params);
//         let http_res = http_req.send().await?;
//         let http_txt = http_res.text().await?;

//         // TODO: check res status and err 'code'

//         let api_res = serde_json::from_str::<api::HttpNewResponse>(http_txt.as_str())
//             .with_context(|| format!("Failed to decode response {:?}", http_txt))?;

//         let res = NewOrderResponse {
//             exchange: req.exchange,
//             side: req.side,
//             symbol: req.symbol,
//             r#type: req.r#type,
//             time_in_force: req.time_in_force,
//             order_id: api_res.orderId.to_string(),
//             status: adapt_status(api_res.status)?,
//             price: api_res.price.parse::<f64>()?,
//             quantity: api_res.origQty.parse::<f64>()?,
//             executed: api_res.executedQty.parse::<f64>()?,
//             fills: adapt_fills(api_res.fills)?,
//         };

//         return Ok(res);
//     }
//     async fn cxl_order_request(&self, req: CxlOrderRequest) -> Result<CxlOrderResponse> {
//         let api_req = api::HttpCxlRequest {
//             symbol: req.symbol.to_string_without_delim(),
//             orderId: u64::from_str(req.order_id.as_str())?,
//         };
//         let mut api_params = Params::from(&serde_json::json!(api_req));

//         let http_req = self.sign_request(self.http.delete(api::endpoints::ORDER), &mut api_params);
//         let http_res = http_req.send().await?;
//         let http_txt = http_res.text().await?;

//         // TODO: check res status and err 'code'

//         let api_res = serde_json::from_str::<api::HttpCxlResponse>(http_txt.as_str())
//             .with_context(|| format!("Failed to decode response {:?}", http_txt))?;

//         let res = CxlOrderResponse {
//             exchange: req.exchange,
//             symbol: req.symbol,
//             order_id: api_res.orderId.to_string(),
//             status: adapt_status(api_res.status)?,
//         };

//         return Ok(res);
//     }
// }

fn adapt_status(status: String) -> Result<Status> {
    match status.as_str() {
        "NEW" | "PARTIALLY_FILLED" => { return Ok(Status::Open); }
        "FILLED" |"CANCELED" | "PENDING_CANCEL" | "REJECTED" | "EXPIRED" => { return Ok(Status::Closed); }
        _ => { return Err(anyhow::anyhow!("Unhandled status '{}'", status)); }
    }
}

fn adapt_fills(fills: Vec<api::Fill>) -> Result<Vec<proto_types::common::Fill>> {
    let mut new: Vec<proto_types::common::Fill> = Vec::new();

    for f in fills {
        new.push(proto_types::common::Fill {
            price: f.price.parse::<f64>()?,
            quantity: f.qty.parse::<f64>()?,
        });
    }

    return Ok(new);
}

fn err_to_status<T: ToString>(err: T) -> tonic::Status {
    return tonic::Status::internal(err.to_string());
}
