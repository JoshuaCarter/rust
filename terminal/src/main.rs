#![allow(clippy::needless_return)]
#![allow(clippy::module_inception)]
#![allow(clippy::new_without_default)]

use infra::model::message_stream::MessageStreamCall;
use tokio::sync::mpsc::Sender;

mod cli;
mod grpc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // load .env
    dotenvy::dotenv().ok();

    ctrlc::set_handler(|| {
        println!("CTRLC @ {}", infra::utils::time::now_ms());
        std::process::exit(0);
    })?;

    let task = cli::process_input()?;

    let uri = format!("http://{}", std::env::var("ENV_NODE_URI").unwrap());
    let (grpc_client, join_handle) = grpc::start_client(uri.as_str()).await?;

    match task {
        cli::Task::Trading(x) => {
            match x {
                cli::Trading::NewOrder(call) => {
                    send(grpc_client, MessageStreamCall::from(call)).await;
                }
                cli::Trading::CxlOrder(call) => {
                    send(grpc_client, MessageStreamCall::from(call)).await;
                }
            }
        }
        cli::Task::Market(x) => {
            match x {
                cli::Market::BookUpdates(call) => {
                    send(grpc_client, MessageStreamCall::from(call)).await;
                }
            }
        }
    }

    match join_handle.await {
        Ok(_) => { println!("Session complete"); }
        Err(e) => { println!("Session error: {}", e); }
    }
    println!("EXIT");

    return Ok(());
}

async fn send(grpc_client: Sender<MessageStreamCall>, call: MessageStreamCall) {
    infra::spawn!((grpc_client) => {
        match grpc_client.send(call).await {
            Ok(_) => { println!("Send success"); }
            Err(x) => { println!("Send error {}", x); }
        }
    });
}
