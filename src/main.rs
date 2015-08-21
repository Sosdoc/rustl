mod lisp;

use std::io;

extern crate regex;

fn main() {

    let mut env = lisp::env::Environment::default();

    println!("Lispr interpreter - v 0.1");
    println!("^C to exit");

    loop {
        let mut input : String = String::new();

        io::stdin().read_line(&mut input)
            .ok()
            .expect("Error when reading.");

        let mut tok_vec = lisp::lex::tokenize(&input);
        let tokens = lisp::lex::parse_tree_from_tokens(&mut tok_vec);
        let result = lisp::env::eval(tokens.unwrap(), &mut env);

        println!(":: {}", result);
    }
}
