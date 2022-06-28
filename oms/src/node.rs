mod grpc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load .env
    dotenvy::dotenv().ok();

    grpc::start_server("[::]:50051").await?;

    Ok(())
}
