use tonic::{
    Request,
    Response,
    Status,
};
use infra::model::venue::*;
use infra::model::trading::*;
use infra::model::trading::trading_server::Trading;
use crate::venues;

#[derive(Debug, Default)]
pub struct TradingService {}

#[tonic::async_trait]
impl Trading for TradingService {
    async fn new_order(&self, request: Request<NewOrderCall>) -> Result<Response<NewOrderReply>, Status> {
        let req = NewOrderRequest::from(request.into_inner());
        println!("NEW REQ: {:#?}", req);

        let venue = venues::create_venue(req.exchange).map_err(err_to_status)?;
        let res = venue.new_order(req).await.map_err(err_to_status)?;

        return Ok(Response::new(NewOrderReply::from(res)));
    }

    async fn cxl_order(&self, request: Request<CxlOrderCall>) -> Result<Response<CxlOrderReply>, Status> {
        let req = CxlOrderRequest::from(request.into_inner());
        println!("CXL REQ: {:#?}", req);

        let venue = venues::create_venue(req.exchange).map_err(err_to_status)?;
        let res = venue.cxl_order(req).await.map_err(err_to_status)?;

        return Ok(Response::new(CxlOrderReply::from(res)));
    }
}

fn err_to_status<T: ToString>(err: T) -> Status {
    return Status::internal(err.to_string());
}
