use std::str::FromStr;
use anyhow::Result;
use clap::{Command, arg, ArgMatches, App};
use infra::model::common::*;
use infra::model::trading::*;

pub use clap::Parser;

#[derive(Debug)]
pub enum Trade {
    NewOrder(NewOrderCall),
    CxlOrder(CxlOrderCall),
}

#[derive(Debug)]
pub enum Task {
    Trade(Trade),
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

fn arg_parse<T: FromStr>(args: &ArgMatches, name: &str) -> Result<T, anyhow::Error> {
    let s = args
        .get_one::<String>(name)
        .unwrap()
        .clone();

    match s.parse::<T>() {
        Ok(x) => Ok(x),
        Err(_e) => anyhow::bail!("Failed to parse arg '{}'", name),
    }
}

fn app_to_task(app: Box<App>) -> Result<Task> {
    match app.get_matches().subcommand() {
        Some(("new", args)) => {
            let exchange = arg_parse::<Exchange>(args, "EXCHANGE")?;
            let symbol = arg_parse::<Symbol>(args, "SYMBOL")?;
            let quantity = arg_parse::<f64>(args, "QUANTITY")?;
            let price = arg_parse::<f64>(args, "PRICE")?;
            let side = arg_parse::<Side>(args, "SIDE")?;
            let r#type = Type::Limit;

            return Ok(Task::Trade(Trade::NewOrder(NewOrderCall {
                exchange: exchange as i32,
                symbol: Some(symbol),
                side: side as i32,
                r#type: r#type as i32,
                quantity,
                price,
                time_in_force: TimeInForce::Gtc as i32,
            })));
        }
        Some(("cxl", args)) => {
            let exchange = arg_parse::<Exchange>(args, "EXCHANGE")?;
            let symbol = arg_parse::<Symbol>(args, "SYMBOL")?;
            let order_id = arg_parse::<String>(args, "ID")?;

            return Ok(Task::Trade(Trade::CxlOrder(CxlOrderCall {
                order_id,
                exchange: exchange as i32,
                symbol: Some(symbol),
            })));
        }
        _ => { panic!("No such command!"); }
    }
}
