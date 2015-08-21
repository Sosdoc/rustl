mod lisp;

use std::io;
use lisp::eval::{parse_and_eval};

fn main() {

    let mut env = lisp::env::Environment::default();

    println!("Lispr interpreter - v 0.1");
    println!("^C to exit");

    loop {
        let mut input : String = String::new();

        io::stdin().read_line(&mut input)
            .ok()
            .expect("Error when reading.");

        println!(":: {}", parse_and_eval(&input, &mut env));
    }
}
