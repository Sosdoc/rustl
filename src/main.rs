mod lisp;

use std::io;
use std::io::Write;
use lisp::eval::parse_and_eval;

fn main() {
    let mut env = lisp::env::Environment::default();

    println!("Lispr interpreter - v 0.1");
    println!("^C to exit");

    loop {
        print!(">> ");
        io::stdout().flush().ok().expect("Cannot flush stdout.");

        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .ok()
            .expect("Error when reading.");

        match parse_and_eval(&input, &mut env) {
            Ok(value) => println!("{}", value),
            Err(error) => println!("Error: {:?}", error.get_message())
        }
    }
}
