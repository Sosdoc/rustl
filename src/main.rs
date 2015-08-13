mod lisp;

use lisp::env::{Environment};
use lisp::lex;

use std::io;

extern crate regex;

fn main() {

    println!("Lispr interpreter - v 0.1");
    println!("^C to exit");

    loop {
        print!(">> ");

        let mut input : String = String::new();

        io::stdin().read_line(&mut input)
            .ok()
            .expect("Error when reading.");

        println!("Your input was {}", input);
    }
}
