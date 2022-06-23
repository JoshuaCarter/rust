use mylib::{
    cli,
    model::{
        common::*,
        trading::*,
    },
    venues::{
        binance
    },
};

#[tokio::main]
async fn main() {
    // load .env
    dotenvy::dotenv().ok();

    let task = cli::process_input();
    println!("{:?}", task);

    match task {
        cli::Task::TradeRequest(task) => {
            match task {
                TradeRequest::NewOrderRequest(req) => {
                    match req.exchange {
                        Exchange::Binance => {
                            match binance::Binance::new().request_new_order(req).await {
                                Ok(res) => { println!("{:?}", res); }
                                Err(err) => { println!("Error: {:?}", err); }
                            }
                        }
                        _ => { panic!("Unsupported exchange"); }
                    }
                }
                TradeRequest::CxlOrderRequest(req) => {
                    match req.exchange {
                        Exchange::Binance => {
                            match binance::Binance::new().request_cxl_order(req).await {
                                Ok(res) => { println!("{:?}", res); }
                                Err(err) => { println!("Error: {:?}", err); }
                            }
                        }
                        _ => { panic!("Unsupported exchange"); }
                    }
                }
            }
        }
    }

    // let venue = binance::Binance::new();
    // let time = venue.request_time().await.unwrap();
    // println!("{:?}", time);
    // let acc = venue.request_account().await.unwrap();
    // println!("{:?}", acc);
}
