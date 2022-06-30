use anyhow::Result;
use tonic::{
    Request,
    Response,
    Status,
};
use proto_types::common::*;
use proto_types::trading::*;

#[derive(Debug, Default)]
pub struct TradingService {}

fn err_to_status<T: ToString>(err: T) -> Status {
    return Status::internal(err.to_string());
}

fn str_to_exchange(s: &str) -> Result<Exchange> {
    match s.to_uppercase().as_str() {
        "BINANCE" => Ok(Exchange::Binance),
        "FTX" => Ok(Exchange::Ftx),
        _ => anyhow::bail!("foo"),
    }
}

#[tonic::async_trait]
impl ProtoService for TradingService {
    async fn new_order(&self, request: Request<NewRequest>) -> Result<Response<NewReply>, Status> {
        let req = request.into_inner();
        println!("NEW REQ: {:#?}", req);

        let exchange = str_to_exchange(req.exchange.as_str()).map_err(err_to_status)?;

        let venue = venues::create_venue(&exchange)
            .map_err(err_to_status)?;

        let res = venue.new_order(Request::new(req)).await
            .map_err(err_to_status)?;

        return Ok(res);
    }
    async fn cxl_order(&self, request: Request<CxlRequest>) -> Result<Response<CxlReply>, Status> {
        let req = request.into_inner();
        println!("CXL REQ: {:#?}", req);

        let exchange = str_to_exchange(req.exchange.as_str()).map_err(err_to_status)?;

        let venue = venues::create_venue(&exchange)
            .map_err(err_to_status)?;

        let res = venue.cxl_order(Request::new(req)).await
            .map_err(err_to_status)?;

        return Ok(res);
    }
}
