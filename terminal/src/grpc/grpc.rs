use std::str::FromStr;
use anyhow::Result;
use infra::model::{
    health::*,
    message_stream::{*, message_stream_client::MessageStreamClient},
};
use tonic::{transport::Endpoint, metadata::{MetadataValue, Ascii}, Status, service::Interceptor};
use tokio::{sync::mpsc::{self, Sender}, task::JoinHandle};
use tokio_stream::wrappers::ReceiverStream;

pub type GrpcClient = Sender<MessageStreamCall>;

#[derive(Clone)]
pub struct GrpcInterceptor {
    token: MetadataValue<Ascii>,
}
impl Interceptor for GrpcInterceptor {
    fn call(&mut self, mut req: tonic::Request<()>) -> Result<tonic::Request<()>, Status> {
        req.metadata_mut().insert("authorization", self.token.clone());
        return Ok(req);
    }
}

pub async fn start_client(uri: &str) -> Result<(GrpcClient, JoinHandle<()>)> {
    let endpoint = Endpoint::from_str(uri)?;
    let channel = endpoint.connect().await?;
    let interceptor = GrpcInterceptor {
        token: "TOKEN".parse()?,
    };

    let mut client = MessageStreamClient::with_interceptor(channel, interceptor);
    let (grpc_tx, grpc_rx) = mpsc::channel::<MessageStreamCall>(1);

    // establish message stream
    let res = client.message(ReceiverStream::new(grpc_rx)).await?;
    let mut stream = res.into_inner();

    // listen for messages
    let handle = infra::spawn!((grpc_tx) => {
        println!("waiting");
        while let msg = stream.message().await {
            match msg {
                Ok(Some(rep)) => {
                    if let Some(new) = rep.new {
                        println!("Got new {:?}", new);
                    }
                    if let Some(cxl) = rep.cxl {
                        println!("Got cxl {:?}", cxl);
                    }
                    if let Some(book) = rep.book {
                        println!("Got book {:?}", book);
                    }
                    if let Some(ping) = rep.ping {
                        println!("Got ping");
                        match grpc_tx.send(MessageStreamCall::from(PingMessage::new())).await {
                            Ok(_) => {},
                            Err(e) => {
                                println!("Ping failed due to {}", e);
                            },
                        }
                    }
                }
                Ok(None) => { println!("Connection gone"); break; }
                Err(e) => { println!("Message failed due to {}", e); }
            }
        }
        println!("done waiting");
    });

    // send initial ping
    println!("send ping");
    match grpc_tx.send(MessageStreamCall::from(PingMessage::new())).await {
        Ok(_) => { println!("ping sent"); }
        Err(e) => { println!("Initial ping failed due to {}", e); },
    }

    return Ok((grpc_tx, handle));
}
