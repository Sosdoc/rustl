mod lisp;

use lisp::env::Env;
use lisp::lex;


fn main() {
    let line = "( var some = ( 1 + 2 ) )";
    for token in lex::tokenize(line) {
        println!("{}", token);
    }
    let res = Env::add(1.0, 2.2);
    println!("res : {}", res);
}
