mod calc;
mod time;

use std::env;

use crate::calc::eval::eval;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{}", eval(&args[1..].join(" ")));
}
