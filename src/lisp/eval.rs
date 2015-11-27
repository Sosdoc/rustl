use lisp::lex::{tokenize, parse_tree_from_tokens};
use lisp::cell::Cell;
use lisp::env::Environment;

pub fn eval(token: Cell, env: &mut Environment) -> Cell {
    match token {
        Cell::Symbol(name) => lookup_symbol(name, env),
        Cell::List(tokens) => eval_list(tokens, env),
        _ => token,
    }
}

// Looks up the symbol in the environment, returning the associated value
fn lookup_symbol(name: String, env: &Environment) -> Cell {
    match env.map.get(&name) {
        Some(cell) => {
            match cell {
                &Cell::Number(n) => Cell::Number(n),
                &Cell::Symbol(ref n) => Cell::Symbol(n.to_string()),
                _ => Cell::Nil,
            }
        }
        _ => Cell::Nil,
    }
}

// Evaluates the list
// if the first element is a function or keyword, it executes that, otherwise returns
// the list itself
fn eval_list(mut tokens: Vec<Cell>, env: &mut Environment) -> Cell {
    // match on the first token then take action
    let first = tokens.remove(0);

    if let Cell::Symbol(name) = first {

        if let Some(&Cell::Proc(function)) = env.map.get(&name) {
            // eager eval: each of the arguments is evaluated before calling
            let mut args: Vec<Cell> = Vec::new();
            for arg in tokens {
                args.push(eval(arg, env));
            }

            function(Cell::List(args))
        } else {
            let result = match name.as_ref() {
                "quote" => eval_quote(&mut tokens),
                "if" => eval_if(&mut tokens, env),
                "set" => eval_set(&mut tokens, env),
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

// Implementation for set
// usage: (set name value ...)
fn eval_set(args: &mut Vec<Cell>, env: &mut Environment) -> Cell {
    // usage: set! var_name expression
    if let Cell::Symbol(name) = args.remove(0) {
        let t = eval(args.remove(0), env);
        env.map.insert(name, t);
    }
    // assignment expressions return Nil
    Cell::Nil
}

// Implementation for quote
// usage: (quote value ...)
fn eval_quote(args: &mut Vec<Cell>) -> Cell {
    args.remove(0)
}

// Implementation for if
// usage: (if test eval_if_true [eval_if_false])
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
