#![allow(clippy::needless_return)]
#![allow(clippy::module_inception)]
#![allow(clippy::new_without_default)]

mod grpc;
mod venues;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // load .env
    dotenvy::dotenv().ok();

    ctrlc::set_handler(|| {
        println!("CTRLC @ {}", infra::utils::time::now_ms());
        std::process::exit(0);
    })?;

    let uri = std::env::var("ENV_NODE_URI").unwrap();
    grpc::start_server(uri.as_str()).await?;

    return Ok(());
}
