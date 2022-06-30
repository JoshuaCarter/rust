use anyhow::Result;
use clap::{Command, arg, ArgMatches, App};
use infra::model::common::*;
use proto_types::trading as proto;

pub use clap::Parser;

#[derive(Debug)]
pub enum TradeRequest {
    NewOrder(proto::NewRequest),
    CxlOrder(proto::CxlRequest),
}

#[derive(Debug)]
pub enum Task {
    TradeRequest(TradeRequest),
}

/// Process command line args into an order object
pub fn process_input() -> Result<Task>
{
    let app = make_app();
    return app_to_task(app);
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

fn app_to_task(app: Box<App>) -> Result<Task> {
    match app.get_matches().subcommand() {
        Some(("new", args)) => {
            let exchange = arg_to_str(args, "EXCHANGE").parse::<Exchange>()?;
            let symbol = arg_to_str(args, "SYMBOL").parse::<Symbol>()?;
            let quantity = arg_to_str(args, "QUANTITY").parse::<f64>()?;
            let price = arg_to_str(args, "PRICE").parse::<f64>()?;
            let side = arg_to_str(args, "SIDE").parse::<Side>()?;
            let type_ = Type::Limit;

            return Ok(Task::TradeRequest(TradeRequest::NewOrder(proto::NewRequest {
                exchange: exchange.to_string(),
                symbol: symbol.to_string(),
                side: side.to_string(),
                r#type: type_.to_string(),
                quantity,
                price,
                time_in_force: TimeInForce::GTC.to_string(),
            })));
        }
        Some(("cxl", args)) => {
            let exchange = Exchange::from_str(arg_to_str(args, "EXCHANGE").as_str())?;
            let symbol = Symbol::from_str(arg_to_str(args, "SYMBOL").as_str())?;
            let order_id = arg_to_str(args, "ID");

            return Ok(Task::TradeRequest(TradeRequest::CxlOrder(proto::CxlRequest {
                order_id,
                exchange: exchange.to_string(),
                symbol: symbol.to_string(),
            })));
        }
        _ => { panic!("No such command!"); }
    }
}
