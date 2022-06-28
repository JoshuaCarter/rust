pub use clap::Parser;

use clap::{Command, arg, ArgMatches, App};
use infra::model::common::*;
use infra::model::trading;

#[derive(Debug)]
pub enum Task {
    TradeRequest(trading::TradeRequest),
}

/// Process command line args into an order object
pub fn process_input() -> Task
{
    let app = make_app();
    return app_to_cmd(app);
}

fn make_app() -> Box<App<'static>> {
    // you don't actually specify "rust" in the cli
    return Box::new(Command::new("rust")
        .about("Rusty")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("new")
                .about("Place new order")
                .arg_required_else_help(true)
                .arg(arg!(<EXCHANGE> "The exchange to trade on"))
                .arg(arg!(<SYMBOL> "The symbol to trade"))
                .arg(arg!(<SIDE> "Which side to trade"))
                .arg(arg!(<QUANTITY> "The quantity to trade"))
                .arg(arg!(<PRICE> "The price per quantity to trade"))
        )
        .subcommand(
            Command::new("cxl")
                .about("Cancel an order")
                .arg_required_else_help(true)
                .arg(arg!(<EXCHANGE> "The exchange with our order"))
                .arg(arg!(<SYMBOL> "The symbol for our order"))
                .arg(arg!(<ID> "The order id of our order"))
        ));
}

fn arg_to_str(args: &ArgMatches, name: &str) -> String {
    return args
        .get_one::<String>(name)
        .unwrap()
        .clone();
}

fn app_to_cmd(app: Box<App>) -> Task {
    match app.get_matches().subcommand() {
        Some(("new", args)) => {
            let (exchange, symbol, quantity, price, side, type_);

            exchange = arg_to_str(args, "EXCHANGE").parse::<Exchange>().unwrap();
            symbol = arg_to_str(args, "SYMBOL").parse::<Symbol>().unwrap();
            quantity = arg_to_str(args, "QUANTITY").parse::<f64>().unwrap();
            price = arg_to_str(args, "PRICE").parse::<f64>().unwrap();
            side = arg_to_str(args, "SIDE").parse::<Side>().unwrap();
            type_ = Type::Limit;

            return Task::TradeRequest(trading::TradeRequest::NewOrderRequest(trading::NewOrderRequest {
                exchange,
                symbol,
                side,
                type_,
                quantity,
                price,
                time_in_force: TimeInForce::GTC,
            }));
        }
        Some(("cxl", args)) => {
            let (exchange, symbol, order_id);

            exchange = Exchange::from_str(arg_to_str(args, "EXCHANGE").as_str()).unwrap();
            symbol = Symbol::from_str(arg_to_str(args, "SYMBOL").as_str()).unwrap();
            order_id = arg_to_str(args, "ID");

            return Task::TradeRequest(trading::TradeRequest::CxlOrderRequest(trading::CxlOrderRequest {
                exchange,
                symbol,
                order_id,
            }));
        }
        _ => { panic!("No such command!"); }
    }
}
