use mylib::{
    cli,
    utils::time,
    venues::{
        binance
    },
};

#[tokio::main]
async fn main() {
    // load .env
    dotenvy::dotenv().ok();

    let cmd = cli::process_input();
    println!("{:?}", cmd);

    let venue = binance::Binance::new();

    let time = venue.request_time().await.unwrap();
    println!("{:?}", time);
    // let acc = venue.request_account().await.unwrap();
    // println!("{:?}", acc);
    let new_order = venue.request_new_order().await.unwrap();
    println!("{:?}", new_order);
    time::sleep(1000);
    let cxl_order = venue.request_cxl_order(new_order.orderId).await.unwrap();
    println!("{:?}", cxl_order);
}
