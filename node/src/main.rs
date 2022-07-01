#![allow(clippy::needless_return)]
#![allow(clippy::module_inception)]
#![allow(clippy::new_without_default)]

mod grpc;
mod venues;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load .env
    dotenvy::dotenv().ok();

    grpc::start_server("[::]:50051").await?;

    return Ok(());
}
