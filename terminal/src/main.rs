#![allow(clippy::needless_return)]
#![allow(clippy::module_inception)]
#![allow(clippy::new_without_default)]

use anyhow::Result;

mod cli;
mod grpc;

#[tokio::main]
async fn main() -> Result<()> {
    // load .env
    dotenvy::dotenv().ok();

    let task = cli::process_input()?;

    let uri = format!("http://{}", std::env::var("ENV_NODE_URI").unwrap());
    let mut grpc_client = grpc::start_client(uri.as_str()).await?;

    match task {
        cli::Task::Trading(x) => {
            match x {
                cli::Trading::NewOrder(req) => {
                    println!("{:#?}", req);
                    let res = grpc_client.trading.new_order(req).await?;
                    println!("{:#?}", res);
                }
                cli::Trading::CxlOrder(req) => {
                    println!("{:#?}", req);
                    let res = grpc_client.trading.cxl_order(req).await?;
                    println!("{:#?}", res);
                }
            }
        }
        cli::Task::Market(x) => {
            match x {
                cli::Market::BookUpdates(req) => {
                    println!("{:#?}", req);
                    let res = grpc_client.market.book_updates(req).await?;
                    println!("{:#?}", res);

                    let mut stream = res.into_inner();

                    while let Some(rep) = stream.message().await? {
                        println!("{:#?}", rep);
                    }
                }
            }
        }
    }

    return Ok(());
}
