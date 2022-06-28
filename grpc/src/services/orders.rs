use anyhow::Result;
use tonic::{
    Request,
    Response,
    Status,
    transport::Channel
};

mod orders_proto { tonic::include_proto!("orders"); }
use orders_proto::orders_server::Orders;
use orders_proto::orders_server::OrdersServer;
use orders_proto::orders_client::OrdersClient;
use orders_proto::*;

#[derive(Debug, Default)]
pub struct OrdersGrpc {}

impl super::Service for OrdersGrpc {
    type Server = OrdersServer<OrdersGrpc>;
    type Client = OrdersClient<Channel>;

    fn get_server() -> Self::Server {
        return OrdersServer::new(OrdersGrpc::default());
    }
    fn get_client(channel: Channel) -> Self::Client {
        return OrdersClient::new(channel);
    }
}

#[tonic::async_trait]
impl Orders for OrdersGrpc {
    async fn new_order(&self, request: Request<NewRequest>) -> Result<Response<NewReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = NewReply {
            msg: format!("NEW ORDER {}!", request.into_inner().msg),
        };

        return Ok(Response::new(reply));
    }
    async fn cxl_order(&self, request: Request<CxlRequest>) -> Result<Response<CxlReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = CxlReply {
            msg: format!("CXL ORDER {}!", request.into_inner().msg),
        };

        return Ok(Response::new(reply));
    }
}
