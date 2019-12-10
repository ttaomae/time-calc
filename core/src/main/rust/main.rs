mod calc;
mod time;

use std::env;
use std::io;
use std::process;

use crate::calc::eval::eval;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Interactive mode.
    if args.len() == 1 {
        match interactive_mode() {
            Err(_) => process::exit(1),
            _ => process::exit(0),
        }
    }

    // Evaluate single expression.
    match eval(&args[1..].join(" ")) {
        Ok(result) => println!("{}", result),
        Err(error) => {
            eprintln!("{:?}", error);
            process::exit(2);
        }
    }
}

fn interactive_mode() -> Result<(), io::Error> {
    let stdin = io::stdin();
    loop {
        let mut expression = String::new();
        match stdin.read_line(&mut expression) {
            Ok(0) => return Result::Ok(()),
            Ok(_) => match eval(expression.as_str()) {
                Ok(result) => println!("{}", result),
                Err(error) => eprintln!("{:?}", error),
            },
            Err(e) => return Result::Err(e),
        }
    }
}
