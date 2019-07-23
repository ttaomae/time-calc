mod calc;
mod time;

use std::env;
use std::process;

use crate::calc::eval::eval;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        print_usage();
        process::exit(1);
    }

    match eval(&args[1..].join(" ")) {
        Result::Ok(result) => println!("{}", result),
        Result::Err(error) => {
            eprintln!("{:?}", error);
            process::exit(2);
        }
    }
}

fn print_usage() {
    eprintln!("Usage: time-calc <expression>");
}
