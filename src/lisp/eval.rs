use lisp::lex::{tokenize, parse_tree_from_tokens};
use lisp::cell::Cell;
use lisp::env::Environment;

// TODO: eval is getting big, refactor me maybe?
pub fn eval(token: Cell, env: &mut Environment) -> Cell {
    match token {
        Cell::Symbol(name) => {
            match *env.map.get(&name).unwrap() {
                Cell::Number(n) => Cell::Number(n),
                Cell::Symbol(ref n) => Cell::Symbol(n.to_string()),
                _ => Cell::Nil,
            }
        }
        Cell::Number(n) => Cell::Number(n),
        Cell::List(mut tokens) => {
            // match on the first token then take action
            let first = tokens.remove(0);

            if let Cell::Symbol(name) = first {

                if let Some(&Cell::Proc(function)) = env.map.get(&name) {
                    // eager: eval each of the arguments
                    let mut args: Vec<Cell> = Vec::new();

                    for arg in tokens {
                        args.push(eval(arg, env));
                    }

                    // executes the function with arguments
                    function(Cell::List(args))
                } else {
                    // TODO: should keywords be part of eval?
                    let result = match name.as_ref() {
                        "quote" => eval_quote(&mut tokens),
                        "if" => eval_if(&mut tokens, env),
                        "set!" => eval_set(&mut tokens, env),
                        _ => Cell::Nil,
                    };

                    result
                }
            } else {
                // first is not a symbol, it's a regular list
                tokens.insert(0, first);
                Cell::List(tokens)
            }
        }

        _ => panic!("Unrecognized token"), //Cell::Nil
    }
}

fn eval_set(args: &mut Vec<Cell>, env: &mut Environment) -> Cell {
    // usage: set! var_name expression
    if let Cell::Symbol(name) = args.remove(0) {
        let t = eval(args.remove(0), env);
        env.map.insert(name, t);
    }
    // assignment expressions return Nil
    Cell::Nil
}

fn eval_quote(args: &mut Vec<Cell>) -> Cell {
    args.remove(0)
}

// implementation of "if" keyword
fn eval_if(args: &mut Vec<Cell>, env: &mut Environment) -> Cell {
    let has_else = if args.len() == 3 {
        true
    } else {
        false
    };
    let condition = args.remove(0);

    match eval(condition, env) {
        Cell::True => eval(args.remove(0), env),
        Cell::False if has_else => eval(args.remove(1), env),
        _ => Cell::Nil,
    }
}


// Utility function
pub fn parse_and_eval(input: &str, env: &mut Environment) -> Cell {
    let mut str_vec = tokenize(input);
    let tokens = parse_tree_from_tokens(&mut str_vec);
    eval(tokens.unwrap(), env)
}

// TODO: move tests in separate file
#[test]
fn eval_returns_pi() {
    let t = Cell::Symbol("pi".to_string());
    let mut env = Environment::default();

    let res = match eval(t, &mut env) {
        Cell::Number(n) => n,
        _ => 0.0,
    };

    assert_eq!(res, 3.14159265);
}


#[test]
fn eval_sum() {
    let args = vec![Cell::Symbol("+".to_string()), Cell::Number(2.0), Cell::Number(1.0)];

    let t = Cell::List(args);
    let mut env = Environment::default();

    let res = match eval(t, &mut env) {
        Cell::Number(n) => n,
        _ => 0.0,
    };

    assert_eq!(res, 3.0);

    let args = vec![Cell::Symbol("+".to_string()),
                    Cell::Number(3.0),
                    Cell::Number(2.0),
                    Cell::Number(1.0)];

    let t = Cell::List(args);
    let mut env = Environment::default();

    let res = match eval(t, &mut env) {
        Cell::Number(n) => n,
        _ => 0.0,
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
    let mut env = Environment::default();

    let res = match eval(t, &mut env) {
        Cell::Number(n) => n,
        _ => -10.0,
    };

    assert_eq!(res, 0.0);
}
