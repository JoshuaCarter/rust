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
    println!("{:#?}", task);

    let mut grpc_client = grpc::start_client("http://[::]:50051").await?;

    match task {
        cli::Task::Trade(trade) => {
            match trade {
                cli::Trade::NewOrder(req) => {
                    let res = grpc_client.trading.new_order(req).await?;
                    println!("{:#?}", res);
                }
                cli::Trade::CxlOrder(req) => {
                    let res = grpc_client.trading.cxl_order(req).await?;
                    println!("{:#?}", res);
                }
            }
        }
    }

    return Ok(());
}
