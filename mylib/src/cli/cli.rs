use clap::{Command, arg, ArgMatches};
use crate::model::*;

pub fn cli() -> Order
{
    return get_command_order();
}

fn get_command_matches() -> ArgMatches {
    // you don't actually specify "rust" in the cli
    return Command::new("rust")
        .about("Rusty")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("place")
                .about("Place new order")
                .arg(arg!(<EXCHANGE> "The exchange to place the order on"))
                .arg(arg!(<MARKET> "The market to place the order on"))
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("bid")
                        .about("Place new order")
                        .arg(arg!(<QUANTITY> "The quantity to trade"))
                        .arg(arg!(<PRICE> "The price per quantity to trade"))
                        .arg_required_else_help(true),
                )
                .subcommand(
                    Command::new("ask")
                        .about("Place new order")
                        .arg(arg!(<QUANTITY> "The quantity to on"))
                        .arg(arg!(<PRICE> "The price per quantity to trade"))
                        .arg_required_else_help(true),
                )
                .subcommand(
                    Command::new("cxl")
                        .about("Place cxl order")
                        .arg_required_else_help(true),
                )
        ).get_matches();
}

fn get_arg_str<'a>(args: &'a ArgMatches, name: &str) -> &'a String {
    return args
        .get_one::<String>(name)
        .unwrap();
}

fn get_command_order() -> Order {
    match get_command_matches().subcommand() {
        Some(("place", args)) => {
            let (exchange, market, quantity, price, side);

            exchange = Exchange::from_string(get_arg_str(args, "EXCHANGE")).unwrap();
            market = get_arg_str(args, "MARKET");

            match args.subcommand() {
                Some(("bid", args)) => {
                    quantity = get_arg_str(args, "QUANTITY");
                    price = get_arg_str(args, "PRICE");
                    side = Side::Bid;
                }
                Some(("ask", args)) => {
                    quantity = get_arg_str(args, "QUANTITY");
                    price = get_arg_str(args, "PRICE");
                    side = Side::Ask;
                }
                // Some(("cxl", args)) => {
                //     return Order::new(e, m, q, p, s);
                // }
                _ => { panic!("no subcommand"); }
            }

            return Order::new(exchange, market, quantity, price, side);
        }
        _ => { panic!("no subcommand"); }
    }
}
