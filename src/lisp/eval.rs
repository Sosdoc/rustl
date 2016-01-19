use lisp::lex::{tokenize, parse_form};
use lisp::cell::Cell;
use lisp::env::Environment;

pub fn eval(ast: Cell, env: &mut Environment) -> Cell {
    match ast {
        Cell::Symbol(name) => lookup_symbol(name, env),
        Cell::List(tokens) => eval_list(tokens, env),
        _ => ast,
    }
}

// Looks up the symbol in the environment, returning the associated value
fn lookup_symbol(name: String, env: &Environment) -> Cell {
    match env.lookup(&name) {
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
    // empty list -> no action
    if tokens.is_empty() {
        return Cell::List(tokens);
    }

    let first = tokens.remove(0);

    if let Cell::Symbol(name) = first {

        if let Some(&Cell::Proc(function)) = env.lookup(&name) {
            // eager eval: each of the arguments is evaluated before calling
            let mut args: Vec<Cell> = Vec::new();
            for arg in tokens {
                args.push(eval(arg, env));
            }

            function(Cell::List(args))
        } else {
            eval_core(name.as_ref(), &mut tokens, env)
        }
    } else {
        // first is not a symbol, it's a regular list
        tokens.insert(0, first);
        Cell::List(tokens)
    }
}

fn eval_core(keyword: &str, args: &mut Vec<Cell>, env: &mut Environment) -> Cell {
    match keyword {
        "quote" => eval_quote(args),
        "if" => eval_if(args, env),
        "def!" => eval_def(args, env),
        _ => Cell::Nil,
    }
}

// Implementation for def
// usage: (def! name value ...)
fn eval_def(args: &mut Vec<Cell>, env: &mut Environment) -> Cell {
    if let Cell::Symbol(name) = args.remove(0) {
        let t = eval(args.remove(0), env);
        env.insert(name, t);
    }
    // assignment expressions return the assigned value
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
    let mut tokens = tokenize(input);
    let tree = parse_form(&mut tokens);
    // TODO: have descriptive error messages
    match tree {
        Ok(cell) => eval(cell, env),
        Err(_) => Cell::Symbol("error".to_string()),
    }
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
