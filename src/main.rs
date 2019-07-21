mod calc;
mod time;

use std::env;

use crate::calc::eval::eval;

fn main() {
    let args: Vec<String> = env::args().collect();

    match eval(&args[1..].join(" ")) {
        Result::Ok(result) => println!("{}", result),
        Result::Err(error) => println!("{:?}", error),
    }
}
