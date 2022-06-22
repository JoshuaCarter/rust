use clap::{Command, arg, ArgMatches, App};
use crate::model::*;

/// Process command line args into an order object
pub fn process_input() -> CliCommand
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
            Command::new("place")
                .about("Place new order")
                .arg_required_else_help(true)
                .subcommand_required(true)
                .arg(arg!(<EXCHANGE> "The exchange to place the order on"))
                .arg(arg!(<MARKET> "The market to place the order on"))
                .subcommand(
                    Command::new("bid")
                        .about("Place new order")
                        .arg_required_else_help(true)
                        .arg(arg!(<QUANTITY> "The quantity to trade"))
                        .arg(arg!(<PRICE> "The price per quantity to trade"))
                )
                .subcommand(
                    Command::new("ask")
                        .about("Place new order")
                        .arg_required_else_help(true)
                        .arg(arg!(<QUANTITY> "The quantity to on"))
                        .arg(arg!(<PRICE> "The price per quantity to trade"))
                )
                .subcommand(
                    Command::new("cxl")
                        .about("Place cxl order")
                        .arg_required_else_help(true)
                )
        ));
}

fn arg_to_str<'a>(args: &'a ArgMatches, name: &str) -> &'a String {
    return args
        .get_one::<String>(name)
        .unwrap();
}

fn app_to_cmd(app: Box<App>) -> CliCommand {
    match app.get_matches().subcommand() {
        Some(("place", args)) => {
            let (exchange, market, quantity, price, side);

            exchange = Exchange::from_string(arg_to_str(args, "EXCHANGE")).unwrap();
            market = arg_to_str(args, "MARKET");

            match args.subcommand() {
                Some(("bid", args)) => {
                    quantity = arg_to_str(args, "QUANTITY");
                    price = arg_to_str(args, "PRICE");
                    side = Side::Bid;
                }
                Some(("ask", args)) => {
                    quantity = arg_to_str(args, "QUANTITY");
                    price = arg_to_str(args, "PRICE");
                    side = Side::Ask;
                }
                // Some(("cxl", args)) => {
                //     return Order::new(e, m, q, p, s);
                // }
                _ => { panic!("no subcommand"); }
            }

            return CliCommand::new(exchange, market, quantity, price, side);
        }
        _ => { panic!("no subcommand"); }
    }
}
