use std::str::FromStr;
use anyhow::Result;
use infra::model::{trading::trading_client::TradingClient, market::market_client::MarketClient, health::{health_client::HealthClient, PingPong}};
use tonic::{transport::{Channel, Endpoint}, metadata::{MetadataValue, Ascii}, codegen::InterceptedService, Status, service::Interceptor};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

type GrpcService = InterceptedService<Channel, GrpcInterceptor>;

pub struct GrpcClient {
    pub trading: TradingClient<GrpcService>,
    pub market: MarketClient<GrpcService>,
    pub health: HealthClient<GrpcService>,
}

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

pub async fn start_client(uri: &str) -> Result<GrpcClient> {
    let endpoint = Endpoint::from_str(uri)?;
    let channel = endpoint.connect().await?;
    let interceptor = GrpcInterceptor {
        token: "TOKEN".parse()?,
    };

    let client = GrpcClient {
        trading: TradingClient::with_interceptor(channel.clone(), interceptor.clone()),
        market: MarketClient::with_interceptor(channel.clone(), interceptor.clone()),
        health: HealthClient::with_interceptor(channel, interceptor),
    };

    ping(client.health.clone()).await?;

    return Ok(client);
}

async fn ping(mut client: HealthClient<GrpcService>) -> Result<()> {
    let (grpc_tx, grpc_rx) = mpsc::channel::<PingPong>(1);

    // initial ping
    let res = client.ping(ReceiverStream::new(grpc_rx)).await?;
    let mut stream = res.into_inner();

    tokio::spawn(async move {
        // send intial ping
        match grpc_tx.send(PingPong { sequence: 0 }).await {
            Ok(_) => {
                // ping the pongs
                while let Ok(Some(ping)) = stream.message().await {
                    // send to server
                    let pong = PingPong { sequence: ping.sequence + 1 };
                    match grpc_tx.send(pong).await {
                        Ok(_) => continue,
                        Err(_) => { println!("PING DISCON @ {}", infra::utils::time::now_ms()); break; },
                    }
                }
            },
            Err(_) => { println!("INITIAL PING FAILED @ {}", infra::utils::time::now_ms()); },
        }

    });

    return Ok(());
}
