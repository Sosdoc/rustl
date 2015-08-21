use lisp::lex::{tokenize, parse_tree_from_tokens};
use lisp::cell::Cell;
use lisp::env::Environment;

pub fn eval(token : Cell, env : &mut Environment) -> Cell {

    match token {

        Cell::Symbol(name) => {
            match *env.map.get(&name).unwrap() {
                Cell::Number(n) => Cell::Number(n),
                Cell::Symbol(ref n) => Cell::Symbol(n.to_string()),
                _ => Cell::Nil
            }
        },
        Cell::Number(n) => Cell::Number(n),

        Cell::List(mut tokens) => {
            // This branch does some heavy lifting
            // match on the first token then take action
            let first = tokens.remove(0);

            if let Cell::Symbol(name) = first {

                if let Some(&Cell::Proc(f)) = env.map.get(&name) {
                    // eval each of the following tokens in the list
                    let mut args : Vec<Cell> = Vec::new();

                    for arg in tokens {
                        args.push(eval(arg, env));
                    }
                    // executes the function with evaluated arguments
                    f(Cell::List(args))

                } else {
                    // TODO: some keyword implementations here
                    let result : Cell = match name.as_ref() {
                        "quote" => {
                            // returns the following token literally (should it be a symbol?)
                            tokens.remove(0)
                        },
                        "if" => {
                            Cell::Nil
                        },
                        "set!" => {
                            // usage: set! var_name expression
                            if let Cell::Symbol(name) = tokens.remove(0) {
                                let t = eval(tokens.remove(0), env);
                                env.map.insert(name, t);
                            }

                            Cell::Nil
                        }

                        _ => Cell::Nil
                    };

                    result
                }

            } else {
                // first is not a symbol, it's a regular list
                // TODO: should put back the first element
                Cell::Nil
            }
        },


        _ => panic!("Unrecognized token") //Cell::Nil
    }
}


// Utility function
pub fn parse_and_eval(input : &str, env : &mut Environment) -> Cell {
    let mut str_vec = tokenize(input);
    let tokens = parse_tree_from_tokens(&mut str_vec);
    eval(tokens.unwrap(), env)
}


#[test]
fn eval_returns_pi() {
    let t = Cell::Symbol("pi".to_string());
    let env = Environment::default();

    let res = match eval(t, &env) {
        Cell::Number(n) => n,
        _ => 0.0
    };

    assert_eq!(res, 3.14);
}


#[test]
fn eval_sum() {
    let args = vec![Cell::Symbol("+".to_string()),
                    Cell::Number(2.0),
                    Cell::Number(1.0)];

    let t = Cell::List(args);
    let env = Environment::default();

    let res = match eval(t, &env) {
        Cell::Number(n) => n,
        _ => 0.0
    };

    assert_eq!(res, 3.0);

    let args = vec![Cell::Symbol("+".to_string()),
                    Cell::Number(3.0),
                    Cell::Number(2.0),
                    Cell::Number(1.0)];

    let t = Cell::List(args);
    let env = Environment::default();

    let res = match eval(t, &env) {
        Cell::Number(n) => n,
        _ => 0.0
    };

    assert_eq!(res, 6.0);
}


#[test]
fn eval_sub() {
    let args = vec![Cell::Symbol("-".to_string()),
                    Cell::Number(3.0),
                    Cell::Number(2.0),
                    Cell::Number(1.0)];

    let t = Cell::List(args);
    let env = Environment::default();

    let res = match eval(t, &env) {
        Cell::Number(n) => n,
        _ => -10.0
    };

    assert_eq!(res, 0.0);
}
