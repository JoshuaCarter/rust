use mylib::{
    cli,
    model::{
        common::*,
        trading::*,
    },
    venues::*,
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
                            match binance::Binance::new().new_order_request(req).await {
                                Ok(res) => { println!("{:#?}", res); }
                                Err(err) => { println!("Error: {:?}", err); }
                            }
                        }
                        _ => { panic!("Unsupported exchange"); }
                    }
                }
                TradeRequest::CxlOrderRequest(req) => {
                    match req.exchange {
                        Exchange::Binance => {
                            match binance::Binance::new().cxl_order_request(req).await {
                                Ok(res) => { println!("{:#?}", res); }
                                Err(err) => { println!("Error: {:?}", err); }
                            }
                        }
                        _ => { panic!("Unsupported exchange"); }
                    }
                }
            }
        }
    }
}
