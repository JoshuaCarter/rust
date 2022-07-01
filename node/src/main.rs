#![allow(clippy::needless_return)]
#![allow(clippy::module_inception)]
#![allow(clippy::new_without_default)]

mod grpc;
mod venues;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load .env
    dotenvy::dotenv().ok();

    let uri = std::env::var("ENV_NODE_URI").unwrap();
    grpc::start_server(uri.as_str()).await?;

    return Ok(());
}
